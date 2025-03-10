use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::encoders::snapshot_region_filter_run_length_encoder::SnapshotRegionFilterRunLengthEncoder;
use squalr_engine_api::structures::data_types::generics::vector_comparer::VectorComparer;
use squalr_engine_api::structures::scanning::comparisons::scan_compare_type::ScanCompareType;
use squalr_engine_api::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use squalr_engine_api::structures::scanning::scan_parameters_local::ScanParametersLocal;
use std::simd::prelude::SimdPartialEq;
use std::simd::{LaneCount, Simd, SupportedLaneCount};

pub struct ScannerVectorEncoderCascadingPeriodic<const N: usize>
where
    LaneCount<N>: SupportedLaneCount + VectorComparer<N>, {}

/// Implements a memory region scanner to find cascading matches using "Periodicity Scans with RLE Discard".
/// This is an algorithm that is optmized/specialized for data with repeating 1-8 byte patterns.
///     For 1-periodic scans (all same byte A)
///         Just do a normal SIMD byte scan, and discard all RLEs < data type size
///     For 2-periodic scans (repeating 2 bytes A, B)
///         Create a vector of <A,B,A,B,...> and <B,A,B,A,..>
///         Do 2-byte SIMD comparisons, and OR the results together.
///         Note that the shifted pattern could result in matching up to 2 unwanted bytes at the start/end of the RLE encoding.
///         In the RLE encoder, the first/last bytes need to be manually checked to filter these. Discard RLEs < data size.
///     For 4-periodic scans (repeating 4 bytes A, B, C, D)
///         Create a vector of <A,B,C,D,A,B,C,D,...> <B,C,D,A,B,C,D,A,..> <C,D,A,B,C,D,A,B,..> <D,A,B,C,D,A,B,C,..>
///         As before, we do 4-byte SIMD comparisons. From here we can generalize the RLE trimming.
///         We can use the first byte + size of run length to determine how much we need to trim.
///     For 8-periodic, extrapolate.
///
/// It is very important to realize that even if the user is scanning for a large data type (ie 8 bytes), it can still fall into
/// 1, 2, or 4 periodic! This will give us substantial gains over immediately going for the 8-periodic implementation.
///
/// Similarly, the same is true for byte array scans! If the array of bytes can be decomposed into periodic sequences, periodicty
/// scans will results in substantial savings, given that the array fits into a hardware vector Simd<> type.
impl<const N: usize> ScannerVectorEncoderCascadingPeriodic<N>
where
    LaneCount<N>: SupportedLaneCount + VectorComparer<N>,
{
    pub fn new() -> Self {
        Self {}
    }

    pub fn vector_encode(
        &self,
        current_value_pointer: *const u8,
        _: *const u8,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
        base_address: u64,
        region_size: u64,
        true_mask: Simd<u8, N>,
    ) -> Vec<SnapshotRegionFilter> {
        let data_type = scan_parameters_local.get_data_type();
        let data_type_size_bytes = data_type.get_size_in_bytes();

        match scan_parameters_global.get_compare_type() {
            ScanCompareType::Immediate(scan_compare_type_immediate) => {
                if let Some(compare_func) =
                    data_type.get_vector_compare_func_immediate(&scan_compare_type_immediate, scan_parameters_global, scan_parameters_local)
                {
                    if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(data_type) {
                        let periodicity = Self::calculate_periodicity(immediate_value.get_value_bytes(), data_type_size_bytes);

                        match periodicity {
                            1 => {
                                let mut run_length_encoder = SnapshotRegionFilterRunLengthEncoder::new(base_address);
                                let vector_size_in_bytes = N;
                                let iterations = region_size / vector_size_in_bytes as u64;
                                let remainder_bytes = region_size % vector_size_in_bytes as u64;
                                let remainder_ptr_offset = iterations.saturating_sub(1) as usize * vector_size_in_bytes;
                                let false_mask = Simd::<u8, N>::splat(0);

                                // Compare as many full vectors as we can
                                unsafe {
                                    for index in 0..iterations {
                                        let current_value_pointer = current_value_pointer.add(index as usize * vector_size_in_bytes);
                                        let compare_result = compare_func(current_value_pointer);

                                        self.encode_results(
                                            &compare_result,
                                            &mut run_length_encoder,
                                            data_type_size_bytes,
                                            true_mask,
                                            false_mask,
                                            data_type_size_bytes,
                                        );
                                    }

                                    // Handle remainder elements
                                    if remainder_bytes > 0 {
                                        let current_value_pointer = current_value_pointer.add(remainder_ptr_offset);
                                        let compare_result = compare_func(current_value_pointer);
                                        self.encode_remainder_results(
                                            &compare_result,
                                            &mut run_length_encoder,
                                            data_type_size_bytes,
                                            remainder_bytes,
                                            data_type_size_bytes,
                                        );
                                    }
                                }

                                // Early exit. No post-scan cleanup needed for 1-byte periodicity.
                                run_length_encoder.finalize_current_encode(0);

                                return run_length_encoder.take_result_regions();
                            }
                            // JIRA: 2, 4, 8 if they are more efficient than byte array scans
                            _ => {}
                        };
                    }
                }
            }
            _ => {
                log::error!("Unsupported comparison! Cascading periodic scans only work for immediate scans.");
            }
        }

        // Default to an array of byte scan for unsupported periodicity lengths.
        // ScannerScalarEncoderByteArray::encode_byte_array(current_value_pointer, immediate_value_ptr, data_type_size_bytes, base_address, region_size)

        vec![]
    }

    fn encode_results(
        &self,
        compare_result: &Simd<u8, N>,
        run_length_encoder: &mut SnapshotRegionFilterRunLengthEncoder,
        data_type_size_bytes: u64,
        true_mask: Simd<u8, N>,
        false_mask: Simd<u8, N>,
        minimum_size_bytes: u64,
    ) {
        // Optimization: Check if all scan results are true. This helps substantially when scanning for common values like 0.
        if compare_result.simd_eq(true_mask).all() {
            run_length_encoder.encode_range(N as u64);
        // Optimization: Check if all scan results are false. This is also a very common result, and speeds up scans.
        } else if compare_result.simd_ne(false_mask).all() {
            run_length_encoder.finalize_current_encode_with_minimum_size(N as u64, minimum_size_bytes);
        // Otherwise, there is a mix of true/false results that need to be processed manually.
        } else {
            for byte_index in (0..N).step_by(data_type_size_bytes as usize) {
                if compare_result[byte_index] != 0 {
                    run_length_encoder.encode_range(data_type_size_bytes);
                } else {
                    run_length_encoder.finalize_current_encode_with_minimum_size(data_type_size_bytes, minimum_size_bytes);
                }
            }
        }
    }

    fn encode_remainder_results(
        &self,
        compare_result: &Simd<u8, N>,
        run_length_encoder: &mut SnapshotRegionFilterRunLengthEncoder,
        data_type_size_bytes: u64,
        remainder_bytes: u64,
        minimum_size_bytes: u64,
    ) {
        let start_byte_index = N - remainder_bytes as usize;

        for byte_index in (start_byte_index..N).step_by(data_type_size_bytes as usize) {
            if compare_result[byte_index] != 0 {
                run_length_encoder.encode_range(data_type_size_bytes);
            } else {
                run_length_encoder.finalize_current_encode_with_minimum_size(N as u64, minimum_size_bytes);
            }
        }
    }

    /// Calculates the length of repeating byte patterns within a given data type and value combination.
    /// If there are no repeating patterns, the periodicity will be equal to the data type size.
    /// For example, 7C 01 7C 01 has a data typze size of 4, but a periodicity of 2.
    fn calculate_periodicity(
        immediate_value_bytes: &[u8],
        data_type_size_bytes: u64,
    ) -> u64 {
        // Assume optimal periodicity to begin with
        let mut period = 1;

        // Loop through all remaining bytes, and increase the periodicity when we encounter a byte that violates the current assumption.
        for byte_index in 1..data_type_size_bytes as usize {
            if immediate_value_bytes[byte_index] != immediate_value_bytes[byte_index % period] {
                period = byte_index + 1;
            }
        }

        period as u64
    }
}
