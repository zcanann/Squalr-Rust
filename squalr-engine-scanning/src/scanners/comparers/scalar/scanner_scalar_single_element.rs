use crate::snapshots::snapshot_region::SnapshotRegion;
use crate::scanners::comparers::scalar::scanner_scalar_comparer::ScannerScalarComparer;
use crate::scanners::comparers::snapshot_scanner::Scanner;
use crate::scanners::constraints::scan_constraint::ScanConstraint;
use crate::snapshots::snapshot_sub_region::SnapshotSubRegion;
use std::borrow::BorrowMut;
use std::sync::Once;

pub struct ScannerScalarSingleElement {
}

impl ScannerScalarSingleElement {
    fn new() -> Self {
        Self { }
    }
    
    pub fn get_instance() -> &'static ScannerScalarSingleElement {
        static mut INSTANCE: Option<ScannerScalarSingleElement> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = ScannerScalarSingleElement::new();
                INSTANCE = Some(instance);
            });

            return INSTANCE.as_ref().unwrap_unchecked();
        }
    }
}

/// Implements a scalar (ie CPU bound, non-SIMD) scanner which only scans a single element of memory (ie only containing 1 data type).
impl Scanner for ScannerScalarSingleElement {
    fn scan_region(&self,
        snapshot_region: &SnapshotRegion,
        snapshot_sub_region: &SnapshotSubRegion,
        constraint: &ScanConstraint
    ) -> Vec<SnapshotSubRegion> {
        let scalar_comparer = ScannerScalarComparer::get_instance();
        let compare_result;
        let data_type = constraint.get_data_type();
        let memory_load_func = data_type.get_load_memory_function_ptr();
        
        unsafe {
            if constraint.is_immediate_constraint() {
                let current_value_pointer = snapshot_region.get_sub_region_current_values_pointer(&snapshot_sub_region);
                let mut current_value = data_type.to_default_value();
                let immediate_value = constraint.get_constraint_value().unwrap();
                let compare_func = scalar_comparer.get_immediate_compare_func(constraint.get_constraint_type());
    
                compare_result = compare_func(&memory_load_func, current_value_pointer, current_value.borrow_mut(), &immediate_value);
            } else if constraint.is_relative_constraint() {
                let current_value_pointer = snapshot_region.get_sub_region_current_values_pointer(&snapshot_sub_region);
                let previous_value_pointer = snapshot_region.get_sub_region_previous_values_pointer(&snapshot_sub_region);
                let mut current_value = data_type.to_default_value();
                let mut previous_value = data_type.to_default_value();
                let compare_func = scalar_comparer.get_relative_compare_func(constraint.get_constraint_type());
    
                compare_result = compare_func(&memory_load_func, current_value_pointer, previous_value_pointer, current_value.borrow_mut(), previous_value.borrow_mut());
            } else if constraint.is_immediate_constraint() {
                let current_value_pointer = snapshot_region.get_sub_region_current_values_pointer(&snapshot_sub_region);
                let previous_value_pointer = snapshot_region.get_sub_region_previous_values_pointer(&snapshot_sub_region);
                let mut current_value = data_type.to_default_value();
                let mut previous_value = data_type.to_default_value();
                let compare_func = scalar_comparer.get_relative_delta_compare_func(constraint.get_constraint_type());
                let delta_arg = constraint.get_constraint_value().unwrap(); // TODO: Handle and complain
    
                compare_result = compare_func(&memory_load_func, current_value_pointer, previous_value_pointer, current_value.borrow_mut(), previous_value.borrow_mut(), delta_arg);
            } else {
                panic!("Unrecognized constraint");
            }
        }

        if compare_result {
            return vec![snapshot_sub_region.to_owned()];
        } else {
            return vec![];
        }
    }
}
