use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::comparers::snapshot_scanner::Scanner;
use crate::scanners::comparers::scalar::scanner_scalar_encoder::ScannerScalarEncoder;
use crate::scanners::constraints::scan_constraint::ScanConstraint;
use crate::scanners::constraints::scan_filter_constraint::ScanFilterConstraint;
use crate::snapshots::snapshot_region::SnapshotRegion;
use rayon::prelude::*;
use std::sync::Once;

pub struct ScannerScalarIterativeChunked {
}

impl ScannerScalarIterativeChunked {
    fn new(
    ) -> Self {
        Self { }
    }
    
    pub fn get_instance(
    ) -> &'static ScannerScalarIterativeChunked {
        static mut INSTANCE: Option<ScannerScalarIterativeChunked> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = ScannerScalarIterativeChunked::new();
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap_unchecked();
        }
    }
}

impl Scanner for ScannerScalarIterativeChunked {

    /// Performs a parallel iteration over a region of memory, performing the scan comparison. A parallelized run-length encoding algorithm
    /// is used to generate new sub-regions as the scan progresses.
    /// 
    /// This is substantially faster than the sequential version, but requires a post-step of stitching together subregions that are adjacent.
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_constraint: &ScanConstraint,
        filter_constraint: &ScanFilterConstraint,
    ) -> Vec<SnapshotRegionFilter> {
        let current_value_pointer = snapshot_region.get_current_values_pointer(&snapshot_region_filter);
        let previous_value_pointer = snapshot_region.get_previous_values_pointer(&snapshot_region_filter);
        let data_type = filter_constraint.get_data_type();
        let data_type_size = data_type.size_in_bytes();
        let memory_alignment = filter_constraint.get_memory_alignment_or_default(data_type);
        let element_count = snapshot_region_filter.get_element_count(memory_alignment, data_type_size) as usize;

        // Convert raw pointers to slices
        let current_values_slice = unsafe {
            std::slice::from_raw_parts(current_value_pointer, element_count * memory_alignment as usize)
        };
        let previous_values_slice = unsafe {
            std::slice::from_raw_parts(previous_value_pointer, element_count * memory_alignment as usize)
        };

        // Experimentally 1MB seemed to be the optimal chunk size on my CPU to keep all threads busy
        let chunk_size = 1 << 20;
        let num_chunks = (element_count + chunk_size - 1) / chunk_size;

        let all_subregions: Vec<SnapshotRegionFilter> = (0..num_chunks)
            .into_par_iter()
            .map(|chunk_index| {
                let first_element_index = (chunk_index * chunk_size) as u64;
                let last_element_index = ((chunk_index + 1) * chunk_size).min(element_count) as u64;
                let chunk_address_offset = first_element_index * memory_alignment as u64;
                let local_encoder = ScannerScalarEncoder::get_instance();
                let base_address = snapshot_region_filter.get_base_address() + chunk_address_offset;

                unsafe {
                    return local_encoder.encode(
                        current_values_slice.as_ptr().add(chunk_address_offset as usize),
                        previous_values_slice.as_ptr().add(chunk_address_offset as usize),
                        scan_constraint,
                        filter_constraint,
                        base_address,
                        last_element_index - first_element_index,
                    );
                }
            })
            .reduce_with(|mut region_a, region_b| {
                // Merge the results of two chunks in parallel
                region_a.extend(region_b);
                return region_a;
            })
            .unwrap_or_else(Vec::new);
        
        // TODO: Boundary merging on adjacent regions

        return all_subregions;
    }
}
