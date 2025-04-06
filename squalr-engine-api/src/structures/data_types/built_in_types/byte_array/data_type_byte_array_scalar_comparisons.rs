use crate::structures::data_types::built_in_types::byte_array::data_type_byte_array::DataTypeByteArray;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarComparable;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarCompareFnDelta;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarCompareFnImmediate;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarCompareFnRelative;
use crate::structures::scanning::parameters::mapped::mapped_scan_parameters::MappedScanParameters;
use std::cmp::Ordering;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Shl;
use std::ops::Shr;

/// Scalar comparison functions for comparing byte arrays. Note that these functions operate on single array values.
/// For performance-critical scans, specialized algorithms are implemented elsewhere.
/// Additionally, many comparison functions for arrays are not defined, such as inequalities or delta scans,
/// and are subject to change. The only clearly defined behaviors are: equal, not equal, changed, and unchanged.
impl ScalarComparable for DataTypeByteArray {
    fn get_compare_equal(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnImmediate> {
        let immediate_values = scan_parameters.get_data_value();
        let immediate_values = immediate_values.get_value_bytes().clone();
        let len = immediate_values.len();

        Some(Box::new(move |current_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            current_values == immediate_values
        }))
    }

    fn get_compare_not_equal(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnImmediate> {
        let immediate_values = scan_parameters.get_data_value();
        let immediate_values = immediate_values.get_value_bytes().clone();
        let len = immediate_values.len();

        Some(Box::new(move |current_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            current_values != immediate_values
        }))
    }

    fn get_compare_greater_than(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnImmediate> {
        let immediate_values = scan_parameters.get_data_value();
        let immediate_values = immediate_values.get_value_bytes().clone();
        let len = immediate_values.len();

        Some(Box::new(move |current_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            current_values
                .iter()
                .zip(immediate_values.iter())
                .all(|(current_values, immediate_values)| current_values > immediate_values)
        }))
    }

    fn get_compare_greater_than_or_equal(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnImmediate> {
        let immediate_values = scan_parameters.get_data_value();
        let immediate_values = immediate_values.get_value_bytes().clone();
        let len = immediate_values.len();

        Some(Box::new(move |current_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);

            current_values.cmp(&immediate_values) == Ordering::Greater || current_values == immediate_values
        }))
    }

    fn get_compare_less_than(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnImmediate> {
        let immediate_values = scan_parameters.get_data_value();
        let immediate_values = immediate_values.get_value_bytes().clone();
        let len = immediate_values.len();

        Some(Box::new(move |current_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);

            current_values.cmp(&immediate_values) == Ordering::Less
        }))
    }

    fn get_compare_less_than_or_equal(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnImmediate> {
        let immediate_values = scan_parameters.get_data_value();
        let immediate_values = immediate_values.get_value_bytes().clone();
        let len = immediate_values.len();

        Some(Box::new(move |current_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);

            current_values.cmp(&immediate_values) == Ordering::Less || current_values == immediate_values
        }))
    }

    fn get_compare_changed(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnRelative> {
        let len = scan_parameters.get_data_type().get_size_in_bytes() as usize;

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);
            current_values != previous_values
        }))
    }

    fn get_compare_unchanged(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnRelative> {
        let len = scan_parameters.get_data_type().get_size_in_bytes() as usize;

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values == previous_values
        }))
    }

    fn get_compare_increased(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnRelative> {
        let len = scan_parameters.get_data_type().get_size_in_bytes() as usize;

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .all(|(current_value, previous_value)| current_value > previous_value)
        }))
    }

    fn get_compare_decreased(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnRelative> {
        let len = scan_parameters.get_data_type().get_size_in_bytes() as usize;

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .all(|(current_value, previous_value)| current_value < previous_value)
        }))
    }

    fn get_compare_increased_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.wrapping_add(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_decreased_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.wrapping_sub(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_multiplied_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.mul(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_divided_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        if delta_values.iter().any(|value| *value == 0) {
            return None;
        }

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.div(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_modulo_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        if delta_values.iter().any(|value| *value == 0) {
            return None;
        }

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.rem(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_shift_left_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.shl(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_shift_right_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.shr(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_logical_and_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.bitand(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_logical_or_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.bitor(*delta_value) == *previous_value)
        }))
    }

    fn get_compare_logical_xor_by(
        &self,
        scan_parameters: &MappedScanParameters,
    ) -> Option<ScalarCompareFnDelta> {
        let immediate_values = scan_parameters.get_data_value();
        let delta_values = immediate_values.get_value_bytes().clone();
        let len = delta_values.len();

        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            let current_values = std::slice::from_raw_parts(current_values_ptr, len);
            let previous_values = std::slice::from_raw_parts(previous_values_ptr, len);

            current_values
                .iter()
                .zip(previous_values.iter())
                .zip(delta_values.iter())
                .all(|((current_value, previous_value), delta_value)| current_value.bitxor(*delta_value) == *previous_value)
        }))
    }
}
