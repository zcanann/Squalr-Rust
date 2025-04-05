use crate::structures::data_types::built_in_types::i32be::data_type_i32be::DataTypeI32be;
use crate::structures::data_types::comparisons::vector_comparable::VectorComparable;
use crate::structures::data_types::comparisons::vector_comparable::{
    VectorCompareFnDelta16, VectorCompareFnDelta32, VectorCompareFnDelta64, VectorCompareFnImmediate16, VectorCompareFnImmediate32, VectorCompareFnImmediate64,
    VectorCompareFnRelative16, VectorCompareFnRelative32, VectorCompareFnRelative64,
};
use crate::structures::data_types::comparisons::vector_comparisons_integer_big_endian::VectorComparisonsIntegerBigEndian;
use crate::structures::scanning::parameters::mapped_scan_parameters::ScanParametersCommon;

type PrimitiveType = i32;

const BYTE_COUNT_64: usize = 64;
const ELEMENT_COUNT_64: usize = BYTE_COUNT_64 / size_of::<PrimitiveType>();

const BYTE_COUNT_32: usize = 32;
const ELEMENT_COUNT_32: usize = BYTE_COUNT_64 / size_of::<PrimitiveType>();

const BYTE_COUNT_16: usize = 16;
const ELEMENT_COUNT_16: usize = BYTE_COUNT_64 / size_of::<PrimitiveType>();

impl VectorComparable for DataTypeI32be {
    fn get_vector_compare_equal_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_equal_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_equal_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_not_equal_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_not_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_not_equal_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_not_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_not_equal_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_not_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_greater_than_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_greater_than::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_greater_than_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_greater_than::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_greater_than_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_greater_than::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_greater_than_or_equal_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_greater_than_or_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_greater_than_or_equal_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_greater_than_or_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_greater_than_or_equal_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_greater_than_or_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_less_than_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_less_than::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_less_than_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_less_than::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_less_than_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_less_than::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_less_than_or_equal_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_less_than_or_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_less_than_or_equal_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_less_than_or_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_less_than_or_equal_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_less_than_or_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_changed_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_changed::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_changed_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_changed::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_changed_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_changed::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_unchanged_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_unchanged::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_unchanged_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_unchanged::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_unchanged_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_unchanged::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_increased_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_increased::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_increased_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_increased::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_increased_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_increased::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_decreased_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_decreased::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_decreased_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_decreased::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_decreased_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_decreased::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_increased_by_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnDelta64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_increased_by::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_increased_by_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnDelta32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_increased_by::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_increased_by_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnDelta16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_increased_by::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_decreased_by_64(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnDelta64> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_decreased_by::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_decreased_by_32(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnDelta32> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_decreased_by::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(scan_parameters)
    }

    fn get_vector_compare_decreased_by_16(
        &self,
        scan_parameters: &ScanParametersCommon,
    ) -> Option<VectorCompareFnDelta16> {
        VectorComparisonsIntegerBigEndian::get_vector_compare_decreased_by::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(scan_parameters)
    }
}
