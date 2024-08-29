use crate::scanners::parameters::scan_filter_parameters::ScanFilterParameters;
use crate::scanners::parameters::scan_parameters::ScanParameters;
use crate::scanners::scalar::scanner_scalar_iterative::ScannerScalarIterative;
use crate::scanners::scalar::scanner_scalar_iterative_byte_array::ScannerScalarIterativeByteArray;
use crate::scanners::scalar::scanner_scalar_single_element::ScannerScalarSingleElement;
use crate::scanners::vector::scanner_vector_aligned::ScannerVectorAligned;
use crate::scanners::vector::scanner_vector_cascading::ScannerVectorCascading;
use crate::scanners::vector::scanner_vector_sparse::ScannerVectorSparse;
use crate::snapshots::snapshot_region::SnapshotRegion;
use crate::{filters::snapshot_region_filter::SnapshotRegionFilter, scanners::snapshot_scanner::Scanner};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use squalr_engine_common::values::data_type::DataType;
use std::sync::Once;

pub struct ScanDispatcher {
    scanner_aligned_64: ScannerVectorAligned<u8, 64>,
    scanner_aligned_32: ScannerVectorAligned<u8, 32>,
    scanner_aligned_16: ScannerVectorAligned<u8, 16>,
    scanner_sparse_64: ScannerVectorSparse<u8, 64>,
    scanner_sparse_32: ScannerVectorSparse<u8, 32>,
    scanner_sparse_16: ScannerVectorSparse<u8, 16>,
    scanner_cascading_64: ScannerVectorCascading<u8, 64>,
    scanner_cascading_32: ScannerVectorCascading<u8, 32>,
    scanner_cascading_16: ScannerVectorCascading<u8, 16>,
}

/// Implements a scan dispatcher, which picks the best scanner based on the scan constraints and the region being scanned.
/// Choosing the best scanner is critical to maintaining high performance scans.
impl ScanDispatcher {
    fn new() -> Self {
        Self {
            scanner_aligned_64: ScannerVectorAligned::<u8, 64>::new(),
            scanner_aligned_32: ScannerVectorAligned::<u8, 32>::new(),
            scanner_aligned_16: ScannerVectorAligned::<u8, 16>::new(),
            scanner_sparse_64: ScannerVectorSparse::<u8, 64>::new(),
            scanner_sparse_32: ScannerVectorSparse::<u8, 32>::new(),
            scanner_sparse_16: ScannerVectorSparse::<u8, 16>::new(),
            scanner_cascading_64: ScannerVectorCascading::<u8, 64>::new(),
            scanner_cascading_32: ScannerVectorCascading::<u8, 32>::new(),
            scanner_cascading_16: ScannerVectorCascading::<u8, 16>::new(),
        }
    }

    pub fn get_instance() -> &'static ScanDispatcher {
        static mut INSTANCE: Option<ScanDispatcher> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = ScanDispatcher::new();
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap_unchecked();
        }
    }

    pub fn dispatch_scan(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filters: &Vec<SnapshotRegionFilter>,
        scan_parameters: &ScanParameters,
        scan_filter_parameters: &ScanFilterParameters,
    ) -> Vec<SnapshotRegionFilter> {
        let mut results = vec![];

        for snapshot_region_filter in snapshot_region_filters {
            let scanner_instance = self.acquire_scanner_instance(snapshot_region_filter, scan_filter_parameters);
            let result_sub_regions = unsafe { scanner_instance.scan_region(snapshot_region, snapshot_region_filter, scan_parameters, scan_filter_parameters) };

            for result_sub_region in result_sub_regions {
                results.push(result_sub_region);
            }
        }

        return results;
    }

    pub fn dispatch_scan_parallel(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filters: &Vec<SnapshotRegionFilter>,
        scan_parameters: &ScanParameters,
        scan_filter_parameters: &ScanFilterParameters,
    ) -> Vec<SnapshotRegionFilter> {
        snapshot_region_filters
            // Convert the iterator to a parallel iterator
            .par_iter()
            .flat_map(|snapshot_region_filter| {
                let scanner_instance = self.acquire_scanner_instance(snapshot_region_filter, scan_filter_parameters);

                return unsafe { scanner_instance.scan_region(snapshot_region, snapshot_region_filter, scan_parameters, scan_filter_parameters) };
            })
            .collect()
    }

    fn acquire_scanner_instance(
        &self,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_filter_parameters: &ScanFilterParameters,
    ) -> &dyn Scanner {
        let data_type = scan_filter_parameters.get_data_type();
        let data_type_size = data_type.get_size_in_bytes();
        let memory_alignment = scan_filter_parameters.get_memory_alignment_or_default();
        let memory_alignment_size = memory_alignment as u64;
        let element_count = snapshot_region_filter.get_element_count(memory_alignment, data_type_size);
        let region_size = snapshot_region_filter.get_region_size();

        if element_count == 1 {
            // Single element scanner
            return ScannerScalarSingleElement::get_instance();
        } else {
            match data_type {
                DataType::Bytes(_) => {
                    return ScannerScalarIterativeByteArray::get_instance();
                }
                _ => {
                    // We actually don't really care whether the processor supports AVX-512, AVX2, etc, Rust is smart enough to abstract this.
                    // It is actually more performant to greedily try to use AVX-512 even if its not available, because Rust falls back to
                    // essentially unrolled loops of AVX2 or SSE2 code, and it ends up being faster than the AVX2/SSE-first implementations.
                    if region_size >= 64 {
                        if memory_alignment_size == data_type_size {
                            return &self.scanner_aligned_64;
                        } else if memory_alignment_size > data_type_size {
                            return &self.scanner_sparse_64;
                        } else {
                            return &self.scanner_cascading_64;
                        }
                    } else if region_size >= 32 {
                        if memory_alignment_size == data_type_size {
                            return &self.scanner_aligned_32;
                        } else if memory_alignment_size > data_type_size {
                            return &self.scanner_sparse_32;
                        } else {
                            return &self.scanner_cascading_32;
                        }
                    } else if region_size >= 16 {
                        if memory_alignment_size == data_type_size {
                            return &self.scanner_aligned_16;
                        } else if memory_alignment_size > data_type_size {
                            return &self.scanner_sparse_16;
                        } else {
                            return &self.scanner_cascading_16;
                        }
                    }
                }
            }
        }

        // Default to scalar iterative
        return ScannerScalarIterative::get_instance();
    }
}
