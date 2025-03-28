use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::encoders::snapshot_region_filter_run_length_encoder::SnapshotRegionFilterRunLengthEncoder;
use crate::scanners::snapshot_scanner::Scanner;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type::ScanCompareType;
use squalr_engine_api::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use squalr_engine_api::structures::scanning::scan_parameters_local::ScanParametersLocal;

pub struct ScannerScalarIterative {}

impl ScannerScalarIterative {}

/// Implements a scalar (ie CPU bound, non-SIMD) region scanning algorithm. This simply iterates over a region of memory,
/// comparing each element based on the provided parameters. Elements that pass the scan are grouped into filter ranges and returned.
impl Scanner for ScannerScalarIterative {
    /// Performs a sequential iteration over a region of memory, performing the scan comparison. A run-length encoding algorithm
    /// is used to generate new sub-regions as the scan progresses.
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Vec<SnapshotRegionFilter> {
        let base_address = snapshot_region_filter.get_base_address();
        let memory_alignment = scan_parameters_local.get_memory_alignment_or_default();
        let memory_alignment_size = memory_alignment as u64;
        let data_type = scan_parameters_local.get_data_type();
        let data_type_size = data_type.get_size_in_bytes();
        let data_type_size_padding = data_type_size.saturating_sub(memory_alignment_size);
        let element_count = snapshot_region_filter.get_element_count(data_type, memory_alignment);
        let current_value_pointer = snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter);
        let previous_value_pointer = snapshot_region.get_previous_values_filter_pointer(&snapshot_region_filter);
        let mut run_length_encoder = SnapshotRegionFilterRunLengthEncoder::new(base_address);

        unsafe {
            // Run length encoding for the scan results.
            let mut encode_results = |compare_result: bool| {
                if compare_result {
                    run_length_encoder.encode_range(memory_alignment_size);
                } else {
                    run_length_encoder.finalize_current_encode_data_size_padded(memory_alignment_size, data_type_size_padding);
                }
            };

            match scan_parameters_global.get_compare_type() {
                ScanCompareType::Immediate(scan_compare_type_immediate) => {
                    if let Some(compare_func) =
                        data_type.get_scalar_compare_func_immediate(&scan_compare_type_immediate, scan_parameters_global, scan_parameters_local)
                    {
                        for index in 0..element_count {
                            let current_value_pointer = current_value_pointer.add(index as usize * memory_alignment as usize);
                            let result = compare_func(current_value_pointer);

                            encode_results(result);
                        }
                    }
                }
                ScanCompareType::Relative(scan_compare_type_relative) => {
                    if let Some(compare_func) =
                        data_type.get_scalar_compare_func_relative(&scan_compare_type_relative, scan_parameters_global, scan_parameters_local)
                    {
                        for index in 0..element_count {
                            let current_value_pointer = current_value_pointer.add(index as usize * memory_alignment as usize);
                            let previous_value_pointer = previous_value_pointer.add(index as usize * memory_alignment as usize);
                            let result = compare_func(current_value_pointer, previous_value_pointer);

                            encode_results(result);
                        }
                    }
                }
                ScanCompareType::Delta(scan_compare_type_delta) => {
                    if let Some(compare_func) = data_type.get_scalar_compare_func_delta(&scan_compare_type_delta, scan_parameters_global, scan_parameters_local)
                    {
                        for index in 0..element_count {
                            let current_value_pointer = current_value_pointer.add(index as usize * memory_alignment as usize);
                            let previous_value_pointer = previous_value_pointer.add(index as usize * memory_alignment as usize);
                            let result = compare_func(current_value_pointer, previous_value_pointer);

                            encode_results(result);
                        }
                    }
                }
            }
        }

        run_length_encoder.finalize_current_encode_data_size_padded(memory_alignment_size, data_type_size_padding);
        run_length_encoder.take_result_regions()
    }
}
