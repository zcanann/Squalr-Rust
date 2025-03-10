use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::encoders::snapshot_region_filter_run_length_encoder::SnapshotRegionFilterRunLengthEncoder;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type::ScanCompareType;
use squalr_engine_api::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use squalr_engine_api::structures::scanning::scan_parameters_local::ScanParametersLocal;
use std::collections::HashMap;

pub struct ScannerScalarEncoderByteArray {}

impl ScannerScalarEncoderByteArray {
    /// Scans a region of memory for an array of bytes defined by the given parameters. Comines the Boyer-Moore
    /// algorithm and a run length encoder to produce matches.
    ///
    /// This combination is important to efficiently capture repeated array of byte scans that are sequential in memory.
    /// The run length encoder only produces scan results after encountering a false result (scan failure / mismatch),
    /// or when no more bytes are present (and a full matching byte array was just encoded).
    pub fn scalar_encode_byte_array(
        current_value_pointer: *const u8,
        _prevous_value_pointer: *const u8,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
        base_address: u64,
        region_size: u64,
    ) -> Vec<SnapshotRegionFilter> {
        /*
        match data_type {
            DataType::Bytes(_) => {}
            _ => panic!("Unsupported data type passed to byte array scanner"),
        } */

        match scan_parameters_global.get_compare_type() {
            ScanCompareType::Immediate(scan_compare_type_immediate) => {
                // JIRA: This should be the data_value.get_size_in_bytes() to support container types
                /*
                return ScannerScalarEncoderByteArray::encode_byte_array(
                    current_value_pointer,
                    array_ptr,
                    data_type.get_size_in_bytes(),
                    base_address,
                    region_size,
                ); */
            }
            ScanCompareType::Relative(scan_compare_type_relative) => {
                panic!("Not supported yet (or maybe ever)");
            }
            ScanCompareType::Delta(scan_compare_type_delta) => {
                panic!("Not supported yet (or maybe ever)");
            }
        }
        vec![]
    }

    /// Public encoder without scan parameter and filter args to allow re-use by other scanners.
    pub fn encode_byte_array(
        current_value_pointer: *const u8,
        array_ptr: *const u8,
        array_length: u64,
        base_address: u64,
        region_size: u64,
    ) -> Vec<SnapshotRegionFilter> {
        let mut run_length_encoder = SnapshotRegionFilterRunLengthEncoder::new(base_address);
        let mut mismatch_shift_table = HashMap::<u8, u64>::new();
        let mut matching_suffix_shift_table = vec![0; array_length as usize];

        // Build the mismatch shift table per the Boyer-Moore algorithm.
        // This dictates how far we shift our comparison window if a byte match fails.
        for index in 0..array_length {
            let byte_value = unsafe { *array_ptr.add(index as usize) };
            let shift_value = array_length - index - 1;

            // Populated as mismatch_shift_table[byte_value] => length_of_array - byte_index - 1
            // JIRA: When we support masking, skip adding any elements that have a corresponding mask entry.
            mismatch_shift_table.insert(byte_value, shift_value);
        }

        // Build the Matching (good) Suffix Rule shift table.
        // This is an optimization used to more optimally shift when there are partial matches.
        let mut suffix_length = 0;
        for index in (0..array_length).rev() {
            if Self::is_prefix(array_ptr, index as usize + 1, array_length) {
                suffix_length = array_length - 1 - index;
            }
            matching_suffix_shift_table[index as usize] = suffix_length + (array_length - 1 - index);
        }

        for index in 0..array_length - 1 {
            let suffix_length = Self::suffix_length(array_ptr, index as usize, array_length);
            matching_suffix_shift_table[suffix_length as usize] = array_length - 1 - index + suffix_length;
        }

        let mut index = 0;

        // Main body of the Boyer-Moore algorithm, see https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm for details.
        // Or honestly go watch a YouTube video, visuals are probably better for actually understanding. It's pretty simple actually.
        while index <= region_size - array_length as u64 {
            let mut match_found = true;
            let mut shift_value = 1;

            for inverse_array_index in (0..array_length).rev() {
                let current_byte = unsafe { *current_value_pointer.add((index + inverse_array_index) as usize) };
                let pattern_byte = unsafe { *array_ptr.add(inverse_array_index as usize) };
                // JIRA: Also check masking table when we decide to support masking.
                let is_mismatch = current_byte != pattern_byte;

                if is_mismatch {
                    match_found = false;

                    let bad_char_shift = *mismatch_shift_table.get(&current_byte).unwrap_or(&array_length);
                    let good_suffix_shift = matching_suffix_shift_table[inverse_array_index as usize];
                    shift_value = bad_char_shift.max(good_suffix_shift);

                    break;
                }
            }

            // The one key difference to vanilla Boyer-Moore -- our run length encoder needs to advance every time our
            // index advances. This is either going to be by the array length (for a match), or the shift length (for a mismatch).
            if match_found {
                run_length_encoder.encode_range(array_length);
                index += array_length;
            } else {
                run_length_encoder.finalize_current_encode(shift_value);
                index += shift_value;
            }
        }

        run_length_encoder.finalize_current_encode_with_minimum_size(0, array_length);
        run_length_encoder.take_result_regions()
    }

    fn is_prefix(
        pattern_ptr: *const u8,
        suffix_start: usize,
        pattern_length: u64,
    ) -> bool {
        let suffix_len = pattern_length.saturating_sub(suffix_start as u64);

        for index in 0..suffix_len {
            if unsafe { *pattern_ptr.add(index as usize) } != unsafe { *pattern_ptr.add(suffix_start + index as usize) } {
                return false;
            }
        }

        true
    }

    fn suffix_length(
        pattern_ptr: *const u8,
        match_pos: usize,
        pattern_length: u64,
    ) -> u64 {
        let mut length = 0;
        let mut suffix_index = match_pos as isize;
        let mut pattern_end_index = pattern_length as isize - 1;

        while suffix_index >= 0 && unsafe { *pattern_ptr.add(suffix_index as usize) } == unsafe { *pattern_ptr.add(pattern_end_index as usize) } {
            length += 1;
            suffix_index -= 1;
            pattern_end_index -= 1;
        }

        length
    }
}
