use crate::scanners::comparers::scalar::scanner_scalar_encoder::ScannerScalarEncoder;
use crate::scanners::comparers::snapshot_scanner::Scanner;
use crate::scanners::constraints::scan_constraint::ScanConstraint;
use crate::snapshots::snapshot_region::SnapshotRegion;
use crate::snapshots::snapshot_sub_region::SnapshotSubRegion;
use std::sync::Once;

pub struct ScannerScalarIterative {
}

impl ScannerScalarIterative {
    fn new() -> Self { Self { } }
    
    pub fn get_instance() -> &'static ScannerScalarIterative {
        static mut INSTANCE: Option<ScannerScalarIterative> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = ScannerScalarIterative::new();
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap_unchecked();
        }
    }
}

/// Implements a scalar (ie CPU bound, non-SIMD) region scanning algorithm. This simply iterates over a region of memory,
/// comparing each element based on the provided constraints. Elements that pass the constraint are grouped in sub-regions and returned.
impl Scanner for ScannerScalarIterative {

    /// Performs a sequential iteration over a region of memory, performing the scan comparison. A run-length encoding algorithm
    /// is used to generate new sub-regions as the scan progresses.
    fn scan_region(&self, snapshot_region: &SnapshotRegion, snapshot_sub_region: &SnapshotSubRegion, constraint: &ScanConstraint) -> Vec<SnapshotSubRegion> {
        let data_type_size = constraint.get_element_type().size_in_bytes();
        let aligned_element_count = snapshot_sub_region.get_element_count(constraint.get_alignment(), data_type_size);
        let encoder = ScannerScalarEncoder::get_instance();
        let current_value_pointer = snapshot_region.get_sub_region_current_values_pointer(&snapshot_sub_region);
        let previous_value_pointer = snapshot_region.get_sub_region_previous_values_pointer(&snapshot_sub_region);

        let results = encoder.encode(
            current_value_pointer,
            previous_value_pointer,
            constraint,
            snapshot_sub_region.get_base_address(),
            aligned_element_count
        );
        
        // TODO: Boundary merging on adjacent regions

        return results;
    }
}
