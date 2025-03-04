use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::encoders::vector::scanner_vector_encoder::ScannerVectorEncoder;
use crate::scanners::parameters::scan_parameters::ScanParameters;
use crate::scanners::snapshot_scanner::Scanner;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_common::structures::data_types::comparisons::vector_compare::VectorCompare;
use squalr_engine_common::structures::data_types::data_type_ref::DataTypeRef;
use squalr_engine_common::structures::memory_alignment::MemoryAlignment;
use std::simd::{LaneCount, Simd, SupportedLaneCount};

pub struct ScannerVectorAligned<const N: usize>
where
    LaneCount<N>: SupportedLaneCount + VectorCompare<N>, {}

impl<const N: usize> Scanner for ScannerVectorAligned<N>
where
    LaneCount<N>: SupportedLaneCount + VectorCompare<N>,
{
    /// Performs a sequential iteration over a region of memory, performing the scan comparison.
    /// A run-length encoding algorithm is used to generate new sub-regions as the scan progresses.
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_parameters: &ScanParameters,
        data_type: &DataTypeRef,
        _: MemoryAlignment,
    ) -> Vec<SnapshotRegionFilter> {
        let vector_encoder = ScannerVectorEncoder::<N>::new();
        let simd_all_true_mask = Simd::<u8, N>::splat(0xFF);

        let results = vector_encoder.vector_encode(
            snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter),
            snapshot_region.get_previous_values_filter_pointer(&snapshot_region_filter),
            scan_parameters,
            data_type,
            snapshot_region_filter.get_base_address(),
            snapshot_region_filter.get_region_size(),
            simd_all_true_mask,
        );

        results
    }
}
