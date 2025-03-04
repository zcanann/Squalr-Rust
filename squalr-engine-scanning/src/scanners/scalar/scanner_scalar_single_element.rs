use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::parameters::scan_parameters::ScanParameters;
use crate::scanners::snapshot_scanner::Scanner;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_common::structures::data_types::data_type_ref::DataTypeRef;
use squalr_engine_common::structures::memory_alignment::MemoryAlignment;
use squalr_engine_common::structures::scanning::scan_compare_type::ScanCompareType;

pub struct ScannerScalarSingleElement {}

/// Implements a scalar (ie CPU bound, non-SIMD) scanner which only scans a single element of memory (ie only containing 1 data type).
impl Scanner for ScannerScalarSingleElement {
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_parameters: &ScanParameters,
        data_type: &DataTypeRef,
        _: MemoryAlignment,
    ) -> Vec<SnapshotRegionFilter> {
        let mut compare_result = false;

        unsafe {
            match scan_parameters.get_compare_type() {
                ScanCompareType::Immediate(scan_compare_type_immediate) => {
                    if let Some(compare_func) = data_type.get_scalar_compare_func_immediate(&scan_compare_type_immediate) {
                        let current_value_pointer = snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter);
                        if let Some(immediate_value) = scan_parameters.deanonymize_immediate(&data_type) {
                            let immediate_value_ptr = immediate_value.as_ptr();

                            compare_result = compare_func(current_value_pointer, immediate_value_ptr);
                        }
                    }
                }
                ScanCompareType::Relative(scan_compare_type_relative) => {
                    if let Some(compare_func) = data_type.get_scalar_compare_func_relative(&scan_compare_type_relative) {
                        let current_value_pointer = snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter);
                        let previous_value_pointer = snapshot_region.get_previous_values_filter_pointer(&snapshot_region_filter);

                        compare_result = compare_func(current_value_pointer, previous_value_pointer);
                    }
                }
                ScanCompareType::Delta(scan_compare_type_delta) => {
                    if let Some(compare_func) = data_type.get_scalar_compare_func_delta(&scan_compare_type_delta) {
                        let current_value_pointer = snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter);
                        let previous_value_pointer = snapshot_region.get_previous_values_filter_pointer(&snapshot_region_filter);
                        if let Some(delta_arg) = scan_parameters.deanonymize_immediate(&data_type) {
                            let delta_arg_ptr = delta_arg.as_ptr();

                            compare_result = compare_func(current_value_pointer, previous_value_pointer, delta_arg_ptr);
                        }
                    }
                }
            }
        }

        if compare_result {
            vec![SnapshotRegionFilter::new(
                snapshot_region_filter.get_base_address(),
                snapshot_region_filter.get_region_size(),
            )]
        } else {
            vec![]
        }
    }
}
