use crate::structures::data_types::built_in_types::u8::data_type_u8::DataTypeU8;
use crate::structures::data_types::comparisons::vector_comparable::VectorComparable;
use crate::structures::data_types::comparisons::vector_comparable::{
    VectorCompareFnDelta16, VectorCompareFnDelta32, VectorCompareFnDelta64, VectorCompareFnImmediate16, VectorCompareFnImmediate32, VectorCompareFnImmediate64,
    VectorCompareFnRelative16, VectorCompareFnRelative32, VectorCompareFnRelative64,
};
use crate::structures::data_types::comparisons::vector_comparisons_integer::VectorComparisonsInteger;
use crate::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use crate::structures::scanning::scan_parameters_local::ScanParametersLocal;

type PrimitiveType = u8;

const BYTE_COUNT_64: usize = 64;
const ELEMENT_COUNT_64: usize = BYTE_COUNT_64 / size_of::<PrimitiveType>();

const BYTE_COUNT_32: usize = 32;
const ELEMENT_COUNT_32: usize = BYTE_COUNT_64 / size_of::<PrimitiveType>();

const BYTE_COUNT_16: usize = 16;
const ELEMENT_COUNT_16: usize = BYTE_COUNT_64 / size_of::<PrimitiveType>();

impl VectorComparable for DataTypeU8 {
    fn get_vector_compare_equal_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsInteger::get_vector_compare_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_equal_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsInteger::get_vector_compare_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_equal_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsInteger::get_vector_compare_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_not_equal_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsInteger::get_vector_compare_not_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_not_equal_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsInteger::get_vector_compare_not_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_not_equal_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsInteger::get_vector_compare_not_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_greater_than_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsInteger::get_vector_compare_greater_than::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_greater_than_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsInteger::get_vector_compare_greater_than::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_greater_than_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsInteger::get_vector_compare_greater_than::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_greater_than_or_equal_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsInteger::get_vector_compare_greater_than_or_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_greater_than_or_equal_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsInteger::get_vector_compare_greater_than_or_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_greater_than_or_equal_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsInteger::get_vector_compare_greater_than_or_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_less_than_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsInteger::get_vector_compare_less_than::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_less_than_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsInteger::get_vector_compare_less_than::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_less_than_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsInteger::get_vector_compare_less_than::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_less_than_or_equal_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate64> {
        VectorComparisonsInteger::get_vector_compare_less_than_or_equal::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_less_than_or_equal_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate32> {
        VectorComparisonsInteger::get_vector_compare_less_than_or_equal::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_less_than_or_equal_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnImmediate16> {
        VectorComparisonsInteger::get_vector_compare_less_than_or_equal::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_changed_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsInteger::get_vector_compare_changed::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_changed_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsInteger::get_vector_compare_changed::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_changed_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsInteger::get_vector_compare_changed::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_unchanged_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsInteger::get_vector_compare_unchanged::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_unchanged_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsInteger::get_vector_compare_unchanged::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_unchanged_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsInteger::get_vector_compare_unchanged::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_increased_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsInteger::get_vector_compare_increased::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_increased_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsInteger::get_vector_compare_increased::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_increased_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsInteger::get_vector_compare_increased::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_decreased_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative64> {
        VectorComparisonsInteger::get_vector_compare_decreased::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_decreased_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative32> {
        VectorComparisonsInteger::get_vector_compare_decreased::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_decreased_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnRelative16> {
        VectorComparisonsInteger::get_vector_compare_decreased::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_increased_by_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnDelta64> {
        VectorComparisonsInteger::get_vector_compare_increased_by::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_increased_by_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnDelta32> {
        VectorComparisonsInteger::get_vector_compare_increased_by::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_increased_by_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnDelta16> {
        VectorComparisonsInteger::get_vector_compare_increased_by::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_decreased_by_64(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnDelta64> {
        VectorComparisonsInteger::get_vector_compare_decreased_by::<{ BYTE_COUNT_64 }, { ELEMENT_COUNT_64 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_decreased_by_32(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnDelta32> {
        VectorComparisonsInteger::get_vector_compare_decreased_by::<{ BYTE_COUNT_32 }, { ELEMENT_COUNT_32 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }

    fn get_vector_compare_decreased_by_16(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<VectorCompareFnDelta16> {
        VectorComparisonsInteger::get_vector_compare_decreased_by::<{ BYTE_COUNT_16 }, { ELEMENT_COUNT_16 }, PrimitiveType>(
            scan_parameters_global,
            scan_parameters_local,
        )
    }
}
