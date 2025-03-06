use crate::structures::data_types::comparisons::scalar_comparable::{ScalarCompareFnDelta, ScalarCompareFnImmediate, ScalarCompareFnRelative};
use crate::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use crate::structures::scanning::scan_parameters_local::ScanParametersLocal;
use num_traits::{PrimInt, WrappingAdd, WrappingSub};
use std::ptr;

pub struct ScalarComparisonsIntegerBigEndian {}

impl ScalarComparisonsIntegerBigEndian {
    pub fn get_compare_equal<PrimitiveType: PartialEq + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            // Optimization: no endian byte swap required for immediate or current values.
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType);

                Some(Box::new(move |current_value_ptr| {
                    let current_value = ptr::read_unaligned(current_value_ptr as *const PrimitiveType);

                    current_value == immediate_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_not_equal<PrimitiveType: PartialEq + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            // Optimization: no endian byte swap required for immediate or current values.
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType);

                Some(Box::new(move |current_value_ptr| {
                    let current_value = ptr::read_unaligned(current_value_ptr as *const PrimitiveType);

                    current_value != immediate_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_greater_than<PrimitiveType: PartialOrd + PrimInt + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = PrimitiveType::swap_bytes(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_value_ptr| {
                    let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));

                    current_value > immediate_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_greater_than_or_equal<PrimitiveType: PartialOrd + PrimInt + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = PrimitiveType::swap_bytes(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_value_ptr| {
                    let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));

                    current_value >= immediate_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_less_than<PrimitiveType: PartialOrd + PrimInt + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = PrimitiveType::swap_bytes(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_value_ptr| {
                    let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));

                    // No checks tolerance required.
                    current_value < immediate_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_less_than_or_equal<PrimitiveType: PartialOrd + PrimInt + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = PrimitiveType::swap_bytes(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_value_ptr| {
                    let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));

                    // No checks tolerance required.
                    current_value <= immediate_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_changed<PrimitiveType: PartialEq + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        Some(Box::new(move |current_value_ptr, previous_value_ptr| unsafe {
            let current_value = ptr::read_unaligned(current_value_ptr as *const PrimitiveType);
            let previous_value = ptr::read_unaligned(previous_value_ptr as *const PrimitiveType);

            current_value != previous_value
        }))
    }

    pub fn get_compare_unchanged<PrimitiveType: PartialEq + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        Some(Box::new(move |current_value_ptr, previous_value_ptr| unsafe {
            let current_value = ptr::read_unaligned(current_value_ptr as *const PrimitiveType);
            let previous_value = ptr::read_unaligned(previous_value_ptr as *const PrimitiveType);

            current_value == previous_value
        }))
    }

    pub fn get_compare_increased<PrimitiveType: PartialOrd + PrimInt + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        Some(Box::new(move |current_value_ptr, previous_value_ptr| unsafe {
            let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));
            let previous_value = PrimitiveType::swap_bytes(ptr::read_unaligned(previous_value_ptr as *const PrimitiveType));

            current_value > previous_value
        }))
    }

    pub fn get_compare_decreased<PrimitiveType: PartialOrd + PrimInt + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        Some(Box::new(move |current_value_ptr, previous_value_ptr| unsafe {
            let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));
            let previous_value = PrimitiveType::swap_bytes(ptr::read_unaligned(previous_value_ptr as *const PrimitiveType));

            current_value < previous_value
        }))
    }

    pub fn get_compare_increased_by<PrimitiveType: Copy + PartialEq + PrimInt + WrappingAdd<Output = PrimitiveType> + WrappingAdd + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnDelta> {
        if let Some(delta_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            unsafe {
                let delta_value_ptr = delta_value.as_ptr();
                let delta_value: PrimitiveType = PrimitiveType::swap_bytes(ptr::read_unaligned(delta_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_value_ptr, previous_value_ptr| {
                    let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));
                    let previous_value = PrimitiveType::swap_bytes(ptr::read_unaligned(previous_value_ptr as *const PrimitiveType));
                    let target_value = previous_value.wrapping_add(&delta_value);

                    current_value == target_value
                }))
            }
        } else {
            None
        }
    }

    pub fn get_compare_decreased_by<PrimitiveType: Copy + PartialEq + PrimInt + WrappingSub<Output = PrimitiveType> + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnDelta> {
        if let Some(delta_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            unsafe {
                let delta_value_ptr = delta_value.as_ptr();
                let delta_value: PrimitiveType = PrimitiveType::swap_bytes(ptr::read_unaligned(delta_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_value_ptr, previous_value_ptr| {
                    let current_value = PrimitiveType::swap_bytes(ptr::read_unaligned(current_value_ptr as *const PrimitiveType));
                    let previous_value = PrimitiveType::swap_bytes(ptr::read_unaligned(previous_value_ptr as *const PrimitiveType));
                    let target_value = previous_value.wrapping_sub(&delta_value);

                    current_value == target_value
                }))
            }
        } else {
            None
        }
    }
}
