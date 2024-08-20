use crate::results::scan_results::ScanResults;
use crate::scanners::comparers::scan_dispatcher::ScanDispatcher;
use crate::scanners::constraints::scan_constraint::ScanConstraint;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use squalr_engine_common::conversions::value_to_metric_size;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::tasks::trackable_task::TrackableTask;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use std::thread;

pub struct ManualScanner;

/// Implementation of a task that performs a scan against the provided snapshot. Does not collect new values.
/// Caller is assumed to have already done this if desired.
impl ManualScanner {
    const NAME: &'static str = "Manual Scan";

    pub fn scan(
        scan_results: Arc<RwLock<ScanResults>>,
        constraint: &ScanConstraint,
        task_identifier: Option<String>,
        with_logging: bool
    ) -> Arc<TrackableTask<()>> {
        let task = TrackableTask::<()>::create(
            ManualScanner::NAME.to_string(),
            task_identifier,
        );

        let task_clone = task.clone();
        let constraint_clone = constraint.clone();

        thread::spawn(move || {
            Self::scan_task(
                scan_results,
                &constraint_clone,
                task_clone.clone(),
                task_clone.get_cancellation_token().clone(),
                with_logging
            );

            task_clone.complete(());
        });

        return task;
    }

    fn scan_task(
        scan_results: Arc<RwLock<ScanResults>>,
        constraint: &ScanConstraint,
        task: Arc<TrackableTask<()>>,
        cancellation_token: Arc<AtomicBool>,
        with_logging: bool,
    ) {
        let scan_results = scan_results.write().unwrap();
        let snapshot = scan_results.get_snapshot();

        if snapshot.is_none() {
            Logger::get_instance().log(LogLevel::Error, "No snapshot provided, aborting manual scan.", None);
            return;
        }

        if with_logging {
            Logger::get_instance().log(LogLevel::Info, "Performing manual scan...", None);
        }

        let snapshot = snapshot.unwrap();
        let mut snapshot = snapshot.write().unwrap();
        let region_count = snapshot.get_region_count();
        let snapshot_regions = snapshot.get_snapshot_regions_for_update();
        let constraint = &constraint.clone_and_resolve_auto_alignment();
        let start_time = Instant::now();
        let processed_region_count = Arc::new(AtomicUsize::new(0));
        
        // Iterate over every snapshot region
        snapshot_regions
            .par_iter_mut()
            .for_each(|snapshot_region| {
                if !snapshot_region.can_compare_with_constraint(constraint) {
                    processed_region_count.fetch_add(1, Ordering::SeqCst);
                    return;
                }

                snapshot_region.create_filters(constraint.get_data_types());
                let snapshot_region_filters = snapshot_region.get_filters();
                
                // Iterate over all data type filters. Generally there is only 1 data type, but this is to support multi-data type scans.
                // Each filter is responsible for tracking which ranges of the snapshot region are kept for the scan.
                snapshot_region_filters
                    .into_par_iter()
                    .for_each(|(data_type, snapshot_region_filter)| {
                        if cancellation_token.load(Ordering::SeqCst) {
                            return;
                        }
        
                        // snapshot_region_filter.set_alignment(constraint.get_alignment());
        
                        let scan_dispatcher = ScanDispatcher::get_instance();
                        let scan_results = scan_dispatcher.dispatch_scan_parallel(
                            snapshot_region,
                            snapshot_region_filter,
                            constraint,
                            data_type,
                        );
        
                        // snapshot_region_filter.set_snapshot_sub_regions(scan_results.to_owned());
                    });
        
                let processed = processed_region_count.fetch_add(1, Ordering::SeqCst);

                // To reduce performance impact, only periodically send progress updates.
                if processed % 32 == 0 {
                    let progress = (processed as f32 / region_count as f32) * 100.0;
                    task.set_progress(progress);
                }
            });

            // snapshot.set_name(ManualScanner::NAME.to_string());
        
            let element_count = 420; //snapshot.get_element_count(constraint.get_alignment(), constraint.get_data_type().size_in_bytes());
            let byte_count = 69; // snapshot.get_byte_count();
    
            Logger::get_instance().log(LogLevel::Info, &format!("Results: {} ({} bytes)", element_count, value_to_metric_size(byte_count)), None);
            let duration = start_time.elapsed();
            Logger::get_instance().log(LogLevel::Info, &format!("Scan complete in: {:?}", duration), None);
    }
}
