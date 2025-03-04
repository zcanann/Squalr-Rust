use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::parameters::scan_parameters::ScanParameters;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_common::structures::data_types::data_type::DataType;
use squalr_engine_common::structures::memory_alignment::MemoryAlignment;

pub trait Scanner: Send + Sync {
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_parameters: &ScanParameters,
        data_type: &Box<dyn DataType>,
        memory_alignment: MemoryAlignment,
    ) -> Vec<SnapshotRegionFilter>;
}
