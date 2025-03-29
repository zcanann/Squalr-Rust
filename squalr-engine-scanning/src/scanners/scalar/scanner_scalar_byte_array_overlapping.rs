use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::snapshot_scanner::Scanner;
use crate::scanners::structures::boyer_moore_table::BoyerMooreTable;
use crate::scanners::structures::snapshot_region_filter_run_length_encoder::SnapshotRegionFilterRunLengthEncoder;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type::ScanCompareType;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type_immediate::ScanCompareTypeImmediate;
use squalr_engine_api::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use squalr_engine_api::structures::scanning::scan_parameters_local::ScanParametersLocal;

pub struct ScannerScalarByteArrayNonOverlapping {}

impl ScannerScalarByteArrayNonOverlapping {
    /// Extracts a smaller pattern from a large one. For example, 00 00 00 01 01 01 00 00 00 becomes 00 00 00 01 01 01.
    /// The length returned however is the original length, so this returns a length of 9.
    fn extract_sub_pattern(scan_pattern: &[u8]) -> &[u8] {
        let pattern_length = scan_pattern.len();

        for candidate_len in 1..=pattern_length {
            let candidate_pattern = &scan_pattern[..candidate_len];
            let mut generated_pattern = Vec::with_capacity(pattern_length);

            while generated_pattern.len() < pattern_length {
                generated_pattern.extend_from_slice(candidate_pattern);
            }

            generated_pattern.truncate(pattern_length);

            if generated_pattern == scan_pattern {
                return candidate_pattern;
            }
        }

        // No repeating sub-pattern found, return the full pattern.
        scan_pattern
    }
}

/// Implements a scalar (ie CPU bound, non-SIMD) array of bytes region scanning algorithm. This works by using a modified version of
/// the Boyer-Moore algorithm to encode matches as they are discovered. This algorithm has been adapted to support overlapping results.
impl Scanner for ScannerScalarByteArrayNonOverlapping {
    /// Performs a sequential iteration over a region of memory, performing the scan comparison. A run-length encoding algorithm
    /// is used to generate new sub-regions as the scan progresses.
    fn scan_region(
        &self,
        snapshot_region: &SnapshotRegion,
        snapshot_region_filter: &SnapshotRegionFilter,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Vec<SnapshotRegionFilter> {
        let data_value = match scan_parameters_global.get_compare_type() {
            ScanCompareType::Immediate(scan_compare_type_immediate) => match scan_compare_type_immediate {
                ScanCompareTypeImmediate::Equal => {
                    if let Some(data_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
                        data_value
                    } else {
                        log::error!("Failed to deanonymize array of byte value.");
                        return vec![];
                    }
                }
                _ => {
                    log::error!("Unsupported immediate scan constraint. Only equality is supported for array of byte scans.");
                    return vec![];
                }
            },
            ScanCompareType::Relative(_scan_compare_type_relative) => {
                log::error!("Relative array of byte scans are not supported yet (or maybe ever).");
                return vec![];
            }
            ScanCompareType::Delta(_scan_compare_type_delta) => {
                log::error!("Delta array of byte scans are not supported yet (or maybe ever).");
                return vec![];
            }
        };

        let current_value_pointer = snapshot_region.get_current_values_filter_pointer(&snapshot_region_filter);
        let base_address = snapshot_region_filter.get_base_address();
        let region_size = snapshot_region_filter.get_region_size();
        let memory_alignment = scan_parameters_local.get_memory_alignment_or_default() as u64;

        let original_pattern = data_value.get_value_bytes();
        let full_pattern_length = original_pattern.len() as u64;

        let scan_pattern = Self::extract_sub_pattern(original_pattern);
        let pattern_length = scan_pattern.len() as u64;
        let boyer_moore_table = BoyerMooreTable::new(&scan_pattern, memory_alignment);
        let mut run_length_encoder = SnapshotRegionFilterRunLengthEncoder::new(base_address);
        let mut scan_index: u64 = 0;

        // Main body of the Boyer-Moore algorithm, see https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm for details.
        // Or honestly go watch a YouTube video, visuals are probably better for actually understanding. It's pretty simple actually.
        while scan_index <= region_size - pattern_length {
            let mut is_mismatch = false;
            let mut mismatch_index = 0;
            let mut shift_value = pattern_length;

            for inverse_pattern_index in (0..pattern_length as usize).rev() {
                let current_byte = unsafe { *current_value_pointer.add((scan_index + inverse_pattern_index as u64) as usize) };
                let pattern_byte = scan_pattern[inverse_pattern_index];

                // JIRA: Also check masking table when we decide to support masking.
                is_mismatch = current_byte != pattern_byte;

                if is_mismatch {
                    mismatch_index = inverse_pattern_index;

                    let bad_char_shift = boyer_moore_table.get_mismatch_shift(current_byte);
                    let good_suffix_shift = boyer_moore_table.get_good_suffix_shift(inverse_pattern_index);
                    shift_value = bad_char_shift.max(good_suffix_shift).max(memory_alignment);
                    break;
                }
            }

            // Two key differences to vanilla Boyer-Moore. First, our run length encoder needs to advance every time our
            // index advances. This is either going to be by 1 (for a match), or the shift length (for a mismatch).
            // Note that the original algorithm advances by a full pattern length.
            // However, instead we have to take special care to advance by an amount that:
            // A) Respects alignment.
            // B) Supports overlapping patterns -- ie the end of the pattern could be the start of the next.
            if is_mismatch {
                let partial_match_size = pattern_length.saturating_sub(mismatch_index as u64);
                let mismatch_size = pattern_length.saturating_sub(partial_match_size);

                run_length_encoder.encode_range(partial_match_size);
                run_length_encoder.finalize_current_encode_with_minimum_size(mismatch_size, full_pattern_length);

                scan_index += partial_match_size.saturating_add(mismatch_size);
            } else {
                run_length_encoder.encode_range(shift_value);
                scan_index += shift_value;
            }
        }

        run_length_encoder.finalize_current_encode_with_minimum_size(0, pattern_length);
        run_length_encoder.take_result_regions()
    }
}
