use crate::results::snapshot_region_scan_results::SnapshotRegionScanResults;
use crate::scanners::scan_dispatcher::ScanDispatcher;
use crate::scanners::value_collector::ValueCollector;
use crate::snapshots::snapshot::Snapshot;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use squalr_engine_api::structures::processes::process_info::OpenedProcessInfo;
use squalr_engine_api::structures::scanning::memory_read_mode::MemoryReadMode;
use squalr_engine_api::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use squalr_engine_api::structures::tasks::engine_trackable_task_handle::EngineTrackableTaskHandle;
use squalr_engine_common::conversions::Conversions;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

pub struct ScanExecutor;

/// Implementation of a task that performs a scan against the provided snapshot. Does not collect new values.
/// Caller is assumed to have already done this if desired.
impl ScanExecutor {
    pub fn scan(
        task_handle: EngineTrackableTaskHandle,
        process_info: OpenedProcessInfo,
        snapshot: Arc<RwLock<Snapshot>>,
        scan_parameters_global: &ScanParametersGlobal,
        with_logging: bool,
    ) {
        let scan_parameters_clone = scan_parameters_global.clone();

        thread::spawn(move || {
            Self::scan_task(task_handle.clone(), process_info, snapshot, &scan_parameters_clone, with_logging);

            task_handle.complete();
        });
    }

    fn scan_task(
        task_handle: EngineTrackableTaskHandle,
        process_info: OpenedProcessInfo,
        snapshot: Arc<RwLock<Snapshot>>,
        scan_parameters_global: &ScanParametersGlobal,
        with_logging: bool,
    ) {
        // If the parameter is set, first collect values before the scan.
        // This is slower overall than interleaving the reads, but better for capturing values that may soon change.
        if scan_parameters_global.get_memory_read_mode() == MemoryReadMode::ReadBeforeScan {
            let fix_me = 0;
            // TODO: Some sort of child task function
            // ValueCollector::collect_values(task_handle.clone(), process_info.clone(), snapshot.clone(), None, with_logging).wait_for_completion();
        }

        if with_logging {
            log::info!("Performing manual scan...");
        }

        let mut snapshot = match snapshot.write() {
            Ok(guard) => guard,
            Err(e) => {
                if with_logging {
                    log::error!("Failed to acquire write lock on snapshot: {}", e);
                }

                return;
            }
        };

        let start_time = Instant::now();
        let processed_region_count = Arc::new(AtomicUsize::new(0));
        let total_region_count = snapshot.get_region_count();
        let cancellation_token = task_handle.get_cancellation_token();

        // Iterate over every snapshot region, from which we will grab the existing snapshot filters to perform our next scan.
        snapshot
            .get_snapshot_regions_mut()
            .par_iter_mut()
            .for_each(|snapshot_region| {
                if cancellation_token.load(Ordering::SeqCst) {
                    return;
                }

                if scan_parameters_global.get_memory_read_mode() == MemoryReadMode::ReadInterleavedWithScan {
                    // Attempt to read new (or initial) memory values. Ignore failures as they usually indicate deallocated pages. // JIRA: Remove failures somehow.
                    let _ = snapshot_region.read_all_memory(&process_info);
                }

                if !snapshot_region.can_compare_using_parameters(scan_parameters_global) {
                    processed_region_count.fetch_add(1, Ordering::SeqCst);
                    return;
                }

                // Iterate over each data type in the scan. Generally there is only 1, but multiple simultaneous scans are supported.
                let scan_results = SnapshotRegionScanResults::new(
                    snapshot_region
                        .get_scan_results()
                        .get_filter_collections()
                        .par_iter()
                        .map(|snapshot_region_filter_collection| {
                            // Perform the scan.
                            ScanDispatcher::dispatch_scan_parallel(snapshot_region, snapshot_region_filter_collection, scan_parameters_global)
                        })
                        .collect(),
                );

                snapshot_region.set_scan_results(scan_results);

                let processed = processed_region_count.fetch_add(1, Ordering::SeqCst);

                // To reduce performance impact, only periodically send progress updates.
                if processed % 32 == 0 {
                    let progress = (processed as f32 / total_region_count as f32) * 100.0;
                    task_handle.set_progress(progress);
                }
            });

        snapshot.discard_empty_regions();

        if with_logging {
            let byte_count = snapshot.get_byte_count();
            let duration = start_time.elapsed();

            log::info!("Results: {} bytes", Conversions::value_to_metric_size(byte_count));
            log::info!("Scan complete in: {:?}", duration);
        }
    }
}
