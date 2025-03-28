use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::snapshot_region_filter_run_length_encoder::SnapshotRegionFilterRunLengthEncoder;
use crate::scanners::snapshot_scanner::Scanner;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type::ScanCompareType;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type_immediate::ScanCompareTypeImmediate;
use squalr_engine_api::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use squalr_engine_api::structures::scanning::scan_parameters_local::ScanParametersLocal;
use std::collections::HashMap;

pub struct ScannerScalarByteArray {}

impl ScannerScalarByteArray {
    fn encode_byte_array(
        current_value_pointer: *const u8,
        array: &Vec<u8>,
        scan_parameters_local: &ScanParametersLocal,
        base_address: u64,
        region_size: u64,
    ) -> Vec<SnapshotRegionFilter> {
        let memory_alignment = scan_parameters_local.get_memory_alignment_or_default() as u64;
        let array_length = array.len();
        let mut run_length_encoder = SnapshotRegionFilterRunLengthEncoder::new(base_address);
        let mut mismatch_shift_table = HashMap::<u8, u64>::new();
        let mut matching_suffix_shift_table = vec![0u64; array_length];

        // Build the mismatch shift table per the Boyer-Moore algorithm.
        // This dictates how far we shift our comparison window if a byte match fails.
        for index in 0..array_length {
            let byte_value = array[index];
            let shift_value = array_length - index - 1;

            // Populated as mismatch_shift_table[byte_value] => length_of_array - byte_index - 1
            // JIRA: When we support masking, skip adding any elements that have a corresponding mask entry.
            mismatch_shift_table.insert(byte_value, shift_value as u64);
        }

        // Build the Matching (good) Suffix Rule shift table.
        // This is an optimization used to more optimally shift when there are partial matches.
        let mut suffix_length = 0;
        for index in (0..array_length).rev() {
            if Self::is_prefix(&array[index..], index as usize + 1, array_length) {
                suffix_length = array_length - 1 - index;
            }
            matching_suffix_shift_table[index as usize] = (suffix_length + (array_length - 1 - index)) as u64;
        }

        for index in 0..array_length - 1 {
            let suffix_length = Self::suffix_length(&array[index..], index as usize, array_length);
            matching_suffix_shift_table[suffix_length as usize] = (array_length - 1 - index + suffix_length) as u64;
        }

        let mut index = 0;

        // Main body of the Boyer-Moore algorithm, see https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm for details.
        // Or honestly go watch a YouTube video, visuals are probably better for actually understanding. It's pretty simple actually.
        while index <= region_size - array_length as u64 {
            let mut match_found = true;
            let mut shift_value = 1;

            for inverse_array_index in (0..array_length).rev() {
                let current_byte = unsafe { *current_value_pointer.add((index + inverse_array_index as u64) as usize) };
                let pattern_byte = array[inverse_array_index];
                // JIRA: Also check masking table when we decide to support masking.
                let is_mismatch = current_byte != pattern_byte;

                if is_mismatch {
                    match_found = false;

                    let bad_char_shift = *mismatch_shift_table
                        .get(&current_byte)
                        .unwrap_or(&(array_length as u64));
                    let good_suffix_shift = matching_suffix_shift_table[inverse_array_index as usize] as u64;
                    shift_value = bad_char_shift.max(good_suffix_shift);

                    break;
                }
            }

            // The one key difference to vanilla Boyer-Moore -- our run length encoder needs to advance every time our
            // index advances. This is either going to be by the array length (for a match), or the shift length (for a mismatch).
            if match_found {
                run_length_encoder.encode_range(memory_alignment);
                index += array_length as u64;
            } else {
                run_length_encoder.finalize_current_encode(memory_alignment);
                index += shift_value;
            }
        }

        run_length_encoder.finalize_current_encode_with_minimum_size(0, array_length as u64);
        run_length_encoder.take_result_regions()
    }

    fn is_prefix(
        array: &[u8],
        suffix_start: usize,
        pattern_length: usize,
    ) -> bool {
        let suffix_len = pattern_length.saturating_sub(suffix_start);

        for index in 0..suffix_len {
            if array[index as usize] != array[suffix_start + index as usize] {
                return false;
            }
        }

        true
    }

    fn suffix_length(
        array: &[u8],
        match_pos: usize,
        pattern_length: usize,
    ) -> usize {
        let mut length = 0;
        let mut suffix_index = match_pos as isize;
        let mut pattern_end_index = pattern_length as isize - 1;

        while suffix_index >= 0 && array[suffix_index as usize] == array[pattern_end_index as usize] {
            length += 1;
            suffix_index -= 1;
            pattern_end_index -= 1;
        }

        length
    }
}

/// Implements a scalar (ie CPU bound, non-SIMD) array of bytes region scanning algorithm. This works by using the Boyer-Moore
/// algorithm to encode matches as they are discovered.
impl Scanner for ScannerScalarByteArray {
    /// Performs a sequential iteration over a region of memory, performing the scan comparison. A run-length encoding algorithm
    /// is used to generate new sub-regions as the scan progresses.
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Vec<SnapshotRegionFilter> {
        let current_value_pointer = snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter);
        let base_address = snapshot_region_filter.get_base_address();
        let region_size = snapshot_region_filter.get_region_size();

        match scan_parameters_global.get_compare_type() {
            ScanCompareType::Immediate(scan_compare_type_immediate) => match scan_compare_type_immediate {
                ScanCompareTypeImmediate::Equal => {
                    if let Some(data_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
                        let array = data_value.get_value_bytes();

                        return Self::encode_byte_array(current_value_pointer, array, scan_parameters_local, base_address, region_size);
                    } else {
                        log::error!("Failed to deanonymize array of byte value.");
                    }
                }
                _ => {
                    log::error!("Unsupported immediate scan constraint. Only equality is supported for array of byte scans.");
                }
            },
            ScanCompareType::Relative(_scan_compare_type_relative) => {
                log::error!("Relative array of byte scans are not supported yet (or maybe ever).");
            }
            ScanCompareType::Delta(_scan_compare_type_delta) => {
                log::error!("Delta array of byte scans are not supported yet (or maybe ever).");
            }
        }

        vec![]
    }
}
