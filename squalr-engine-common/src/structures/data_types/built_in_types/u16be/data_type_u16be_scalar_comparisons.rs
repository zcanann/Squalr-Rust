use crate::structures::data_types::built_in_types::u16be::data_type_u16be::DataTypeU16be;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarComparable;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarCompareFnDelta;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarCompareFnImmediate;
use crate::structures::data_types::comparisons::scalar_comparable::ScalarCompareFnRelative;
use crate::structures::data_types::comparisons::scalar_comparisons_integer_big_endian::ScalarComparisonsIntegerBigEndian;
use crate::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use crate::structures::scanning::scan_parameters_local::ScanParametersLocal;

type PrimitiveType = u16;

impl ScalarComparable for DataTypeU16be {
    fn get_compare_equal(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        ScalarComparisonsIntegerBigEndian::get_compare_equal::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_not_equal(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        ScalarComparisonsIntegerBigEndian::get_compare_not_equal::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_greater_than(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        ScalarComparisonsIntegerBigEndian::get_compare_greater_than::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_greater_than_or_equal(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        ScalarComparisonsIntegerBigEndian::get_compare_greater_than_or_equal::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_less_than(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        ScalarComparisonsIntegerBigEndian::get_compare_less_than::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_less_than_or_equal(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnImmediate> {
        ScalarComparisonsIntegerBigEndian::get_compare_less_than_or_equal::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_changed(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        ScalarComparisonsIntegerBigEndian::get_compare_changed::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_unchanged(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        ScalarComparisonsIntegerBigEndian::get_compare_unchanged::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_increased(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        ScalarComparisonsIntegerBigEndian::get_compare_increased::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_decreased(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnRelative> {
        ScalarComparisonsIntegerBigEndian::get_compare_decreased::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_increased_by(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnDelta> {
        ScalarComparisonsIntegerBigEndian::get_compare_increased_by::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }

    fn get_compare_decreased_by(
        &self,
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<ScalarCompareFnDelta> {
        ScalarComparisonsIntegerBigEndian::get_compare_decreased_by::<PrimitiveType>(scan_parameters_global, scan_parameters_local)
    }
}
