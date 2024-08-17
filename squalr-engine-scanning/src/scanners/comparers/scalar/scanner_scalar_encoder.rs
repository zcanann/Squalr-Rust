use squalr_engine_common::dynamic_struct::field_value::FieldValue;

use crate::scanners::comparers::scalar::scanner_scalar_comparer::ScannerScalarComparer;
use crate::scanners::comparers::snapshot_sub_region_run_length_encoder::SnapshotSubRegionRunLengthEncoder;
use crate::scanners::constraints::scan_constraint::ScanConstraint;
use crate::snapshots::snapshot_sub_region::SnapshotSubRegion;
use std::borrow::BorrowMut;
use std::sync::Once;

pub struct ScannerScalarEncoder {
}

impl ScannerScalarEncoder {
    fn new() -> Self {
        Self { }
    }
    
    pub fn get_instance() -> &'static ScannerScalarEncoder {
        static mut INSTANCE: Option<ScannerScalarEncoder> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = ScannerScalarEncoder::new();
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap_unchecked();
        }
    }

    pub fn encode(
        &self,
        current_value_pointer: *const u8,
        previous_value_pointer: *const u8,
        constraint: &ScanConstraint,
        base_address: u64,
        element_count: u64,
    ) -> Vec<SnapshotSubRegion> {
        let comparer = ScannerScalarComparer::get_instance();
        let mut run_length_encoder = SnapshotSubRegionRunLengthEncoder::new(base_address);
        let data_type_size = constraint.get_element_type().size_in_bytes();
        let alignment = constraint.get_alignment() as u64;

        unsafe {
            if constraint.is_immediate_constraint() {
                let mut current_value = FieldValue::default();
                let mut immediate_value = constraint.get_constraint_value().unwrap();
                let current_value = current_value.borrow_mut();
                let immediate_value = immediate_value.borrow_mut();
                let memory_load_func = current_value.get_load_memory_function_ptr();
                let compare_func = comparer.get_immediate_compare_func(constraint.get_constraint_type());

                for index in 0..element_count {
                    let current_value_pointer = current_value_pointer.add(index as usize * alignment as usize);

                    if compare_func(&memory_load_func, current_value_pointer, current_value, immediate_value) {
                        run_length_encoder.encode_range(alignment);
                    } else {
                        run_length_encoder.finalize_current_encode_unchecked(alignment, data_type_size);
                    }
                }
            } else if constraint.is_relative_constraint() {
                let mut current_value = FieldValue::default();
                let mut previous_value = FieldValue::default();
                let current_value = current_value.borrow_mut();
                let previous_value = previous_value.borrow_mut();
                let memory_load_func = current_value.get_load_memory_function_ptr();
                let compare_func = comparer.get_relative_compare_func(constraint.get_constraint_type());

                for index in 0..element_count {
                    let current_value_pointer = current_value_pointer.add(index as usize * alignment as usize);
                    let previous_value_pointer = previous_value_pointer.add(index as usize * alignment as usize);

                    if compare_func(
                        &memory_load_func,
                        current_value_pointer,
                        previous_value_pointer,
                        current_value,
                        previous_value,
                    ) {
                        run_length_encoder.encode_range(alignment);
                    } else {
                        run_length_encoder.finalize_current_encode_unchecked(alignment, data_type_size);
                    }
                }
            } else if constraint.is_immediate_constraint() {
                let mut current_value = FieldValue::default();
                let mut previous_value = FieldValue::default();
                let current_value = current_value.borrow_mut();
                let previous_value = previous_value.borrow_mut();
                let memory_load_func = current_value.get_load_memory_function_ptr();
                let compare_func = comparer.get_relative_delta_compare_func(constraint.get_constraint_type());
                let delta_arg = constraint.get_constraint_delta_value().unwrap();

                for index in 0..element_count {
                    let current_value_pointer = current_value_pointer.add(index as usize * alignment as usize);
                    let previous_value_pointer = previous_value_pointer.add(index as usize * alignment as usize);

                    if compare_func(
                        &memory_load_func,
                        current_value_pointer,
                        previous_value_pointer,
                        current_value,
                        previous_value,
                        delta_arg,
                    ) {
                        run_length_encoder.encode_range(alignment);
                    } else {
                        run_length_encoder.finalize_current_encode_unchecked(alignment, data_type_size);
                    }
                }
            } else {
                panic!("Unrecognized constraint");
            }
        }

        run_length_encoder.finalize_current_encode_unchecked(0, data_type_size);
        
        return run_length_encoder.get_collected_regions().to_owned();
    }
}
