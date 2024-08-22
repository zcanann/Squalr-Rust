use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::comparers::snapshot_region_filter_run_length_encoder::SnapshotRegionFilterRunLengthEncoder;
use crate::scanners::comparers::vector::scanner_vector_comparer_128::ScannerVectorComparer;
use crate::scanners::parameters::scan_parameters::ScanParameters;
use crate::scanners::parameters::scan_filter_parameters::ScanFilterParameters;
use std::borrow::BorrowMut;
use std::sync::Once;

pub struct ScannerVectorEncoder {
}

impl ScannerVectorEncoder {
    fn new(
    ) -> Self {
        Self { }
    }
    
    pub fn get_instance(
    ) -> &'static ScannerVectorEncoder {
        static mut INSTANCE: Option<ScannerVectorEncoder> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = ScannerVectorEncoder::new();
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap_unchecked();
        }
    }

    pub fn encode(
        &self,
        current_value_pointer: *const u8,
        previous_value_pointer: *const u8,
        scan_parameters: &ScanParameters,
        scan_filter_parameters: &ScanFilterParameters,
        base_address: u64,
        element_count: u64,
    ) -> Vec<SnapshotRegionFilter> {
        let comparer = ScannerVectorComparer::get_instance();
        let mut run_length_encoder = SnapshotRegionFilterRunLengthEncoder::new(base_address);
        let data_type = scan_filter_parameters.get_data_type();
        let data_type_size = data_type.size_in_bytes();
        let memory_alignment = scan_filter_parameters.get_memory_alignment_or_default() as u64;
        let memory_load_func = data_type.get_load_memory_function_ptr_u8x16();
        
        unsafe {
            if scan_parameters.is_immediate_comparison() {
                let mut immediate_value = scan_parameters.deanonymize_type(&data_type).unwrap(); // TODO handle and complain
                let immediate_value = memory_load_func(immediate_value.borrow_mut());
                let compare_func = comparer.get_immediate_compare_func(scan_parameters.get_compare_type());

                for index in 0..element_count {
                    let current_value_pointer = current_value_pointer.add(index as usize * memory_alignment as usize);
                    let compare_result = compare_func(current_value_pointer, immediate_value);

                    if compare_result.all() {
                        run_length_encoder.encode_range(16);
                    } if !compare_result.any() {
                        run_length_encoder.finalize_current_encode_unchecked(16, data_type_size);
                    } else {
                        for result in compare_result.to_int().as_array() {
                            if *result != 0 {
                                run_length_encoder.encode_range(memory_alignment);
                            } else {
                                run_length_encoder.finalize_current_encode_unchecked(memory_alignment, data_type_size);
                            }
                        }
                    }
                }
            } else if scan_parameters.is_relative_comparison() {
                let compare_func = comparer.get_relative_compare_func(scan_parameters.get_compare_type());

                for index in 0..element_count {
                    let current_value_pointer = current_value_pointer.add(index as usize * memory_alignment as usize);
                    let previous_value_pointer = previous_value_pointer.add(index as usize * memory_alignment as usize);
                    let compare_result = compare_func(current_value_pointer, previous_value_pointer);

                    if compare_result.all() {
                        run_length_encoder.encode_range(16);
                    } if !compare_result.any() {
                        run_length_encoder.finalize_current_encode_unchecked(16, data_type_size);
                    } else {
                        for result in compare_result.to_int().as_array() {
                            if *result != 0 {
                                run_length_encoder.encode_range(memory_alignment);
                            } else {
                                run_length_encoder.finalize_current_encode_unchecked(memory_alignment, data_type_size);
                            }
                        }
                    }
                }
            } else if scan_parameters.is_immediate_comparison() {
                let compare_func = comparer.get_immediate_compare_func(scan_parameters.get_compare_type());
                let delta_arg = memory_load_func(&scan_parameters.deanonymize_type(&data_type).unwrap()); // TODO handle and complain

                for index in 0..element_count {
                    let current_value_pointer = current_value_pointer.add(index as usize * memory_alignment as usize);
                    let compare_result = compare_func(current_value_pointer, delta_arg);

                    if compare_result.all() {
                        run_length_encoder.encode_range(16);
                    } if !compare_result.any() {
                        run_length_encoder.finalize_current_encode_unchecked(16, data_type_size);
                    } else {
                        for result in compare_result.to_int().as_array() {
                            if *result != 0 {
                                run_length_encoder.encode_range(memory_alignment);
                            } else {
                                run_length_encoder.finalize_current_encode_unchecked(memory_alignment, data_type_size);
                            }
                        }
                    }
                }
            } else {
                panic!("Unrecognized comparison");
            }
        }

        run_length_encoder.finalize_current_encode_unchecked(memory_alignment, data_type_size);
        
        return run_length_encoder.result_regions;
    }
}
