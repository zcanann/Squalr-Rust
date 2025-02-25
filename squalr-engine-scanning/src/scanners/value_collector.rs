use crate::results::snapshot_region_filter::SnapshotRegionFilter;
use crate::results::snapshot_region_scan_results::SnapshotRegionScanResults;
use crate::snapshots::snapshot::Snapshot;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use squalr_engine_common::conversions::Conversions;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_common::tasks::trackable_task::TrackableTask;
use squalr_engine_processes::process_info::OpenedProcessInfo;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

pub struct ValueCollector;

/// Implementation of a task that collects new or initial values for the provided snapshot.
impl ValueCollector {
    const NAME: &'static str = "Value Collector";

    pub fn collect_values(
        process_info: OpenedProcessInfo,
        snapshot: Arc<RwLock<Snapshot>>,
        task_identifier: Option<String>,
        with_logging: bool,
    ) -> Arc<TrackableTask<()>> {
        let process_info = Arc::new(process_info);
        let task = TrackableTask::<()>::create(ValueCollector::NAME.to_string(), task_identifier);

        let task_clone = task.clone();
        let process_info_clone = process_info.clone();
        let snapshot = snapshot.clone();

        std::thread::spawn(move || {
            Self::collect_values_task(
                process_info_clone,
                snapshot,
                with_logging,
                task_clone.clone(),
                task_clone.get_cancellation_token(),
            );

            task_clone.complete(());
        });

        return task;
    }

    fn collect_values_task(
        process_info: Arc<OpenedProcessInfo>,
        snapshot: Arc<RwLock<Snapshot>>,
        with_logging: bool,
        task: Arc<TrackableTask<()>>,
        cancellation_token: Arc<AtomicBool>,
    ) {
        if with_logging {
            Logger::log(
                LogLevel::Info,
                &format!("Reading values from memory (process {})...", process_info.process_id),
                None,
            );
        }

        let data_types_and_alignments = {
            let snapshot = snapshot.read().unwrap();
            snapshot.get_data_types_and_alignments()
        };
        let mut snapshot = snapshot.write().unwrap();
        let total_region_count = snapshot.get_region_count();
        let start_time = Instant::now();
        let processed_region_count = Arc::new(AtomicUsize::new(0));
        let snapshot_regions = snapshot.get_snapshot_regions_for_update();

        snapshot_regions.par_iter_mut().for_each(|snapshot_region| {
            if cancellation_token.load(Ordering::SeqCst) {
                return;
            }

            // Attempt to read new (or initial) memory values. Ignore failures, as these are generally just deallocated pages.
            let _ = snapshot_region.read_all_memory(&process_info);

            let region_scan_results_map = snapshot_region.get_region_scan_results();

            // Create the initial scan results for this data type if none exist.
            data_types_and_alignments
                .par_iter()
                .for_each(|(data_type, memory_alignment)| {
                    if !region_scan_results_map.contains_key(&data_type) {
                        let initial_scan_results = vec![vec![SnapshotRegionFilter::new(
                            snapshot_region.get_base_address(),
                            snapshot_region.get_region_size(),
                        )]];
                        region_scan_results_map.insert(
                            data_type.clone(),
                            SnapshotRegionScanResults::new_from_filters(initial_scan_results, data_type, *memory_alignment),
                        );
                    }
                });

            // Report progress periodically (not every time for performance)
            let processed = processed_region_count.fetch_add(1, Ordering::SeqCst);
            if processed % 32 == 0 {
                let progress = (processed as f32 / total_region_count as f32) * 100.0;
                task.set_progress(progress);
            }
        });

        snapshot.discard_empty_regions();
        snapshot.build_scan_results();

        if with_logging {
            let duration = start_time.elapsed();
            let byte_count = snapshot.get_byte_count();

            Logger::log(LogLevel::Info, &format!("Values collected in: {:?}", duration), None);
            Logger::log(
                LogLevel::Info,
                &format!("{} bytes read ({})", byte_count, Conversions::value_to_metric_size(byte_count)),
                None,
            );
        }
    }
}
