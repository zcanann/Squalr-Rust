use crate::structures::data_types::built_in_types::u64be::data_type_u64be::DataTypeU64be;
use crate::structures::data_types::comparisons::vector_comparable::VectorComparable;
use crate::structures::data_types::comparisons::vector_comparable::{
    VectorCompareFnDelta16, VectorCompareFnDelta32, VectorCompareFnDelta64, VectorCompareFnImmediate16, VectorCompareFnImmediate32, VectorCompareFnImmediate64,
    VectorCompareFnRelative16, VectorCompareFnRelative32, VectorCompareFnRelative64,
};
use crate::structures::data_types::comparisons::vector_generics::VectorGenerics;
use crate::structures::scanning::scan_parameters::ScanParameters;
use std::ptr;
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdInt;
use std::simd::{LaneCount, Simd, SupportedLaneCount};

type PrimitiveType = u64;
type SwapCompatibleType = i64;

struct DataTypeU64beVector {}

impl DataTypeU64beVector {
    pub fn get_vector_compare_equal<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, immediate_value_ptr: *const u8| unsafe {
            // No endian byte swap required.
            let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));
            let immediate_value = Simd::splat(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(immediate_value))
        }
    }

    pub fn get_vector_compare_not_equal<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, immediate_value_ptr: *const u8| unsafe {
            // No endian byte swap required.
            let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));
            let immediate_value = Simd::splat(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_ne(immediate_value))
        }
    }

    pub fn get_vector_compare_greater_than<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, immediate_value_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let immediate_value: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                immediate_value_ptr as *const SwapCompatibleType,
            ))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_gt(immediate_value))
        }
    }

    pub fn get_vector_compare_greater_than_or_equal<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, immediate_value_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let immediate_value: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                immediate_value_ptr as *const SwapCompatibleType,
            ))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_ge(immediate_value))
        }
    }

    pub fn get_vector_compare_less_than<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, immediate_value_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let immediate_value: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                immediate_value_ptr as *const SwapCompatibleType,
            ))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_lt(immediate_value))
        }
    }

    pub fn get_vector_compare_less_than_or_equal<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, immediate_value_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let immediate_value: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                immediate_value_ptr as *const SwapCompatibleType,
            ))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_le(immediate_value))
        }
    }

    pub fn get_vector_compare_changed<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, previous_values_ptr: *const u8| unsafe {
            // No endian byte swap required.
            let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));
            let previous_values = Simd::from_array(ptr::read_unaligned(previous_values_ptr as *const [PrimitiveType; N]));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_ne(previous_values))
        }
    }

    pub fn get_vector_compare_unchanged<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, previous_values_ptr: *const u8| unsafe {
            // No endian byte swap required.
            let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));
            let previous_values = Simd::from_array(ptr::read_unaligned(previous_values_ptr as *const [PrimitiveType; N]));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(previous_values))
        }
    }

    pub fn get_vector_compare_increased<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, previous_values_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let previous_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                previous_values_ptr as *const SwapCompatibleType,
            ))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_gt(previous_values))
        }
    }

    pub fn get_vector_compare_decreased<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, previous_values_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let previous_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                previous_values_ptr as *const SwapCompatibleType,
            ))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_lt(previous_values))
        }
    }

    pub fn get_vector_compare_increased_by<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, previous_values_ptr: *const u8, delta_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let previous_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                previous_values_ptr as *const SwapCompatibleType,
            ))));
            let delta_value: Simd<PrimitiveType, N> =
                VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(delta_ptr as *const SwapCompatibleType))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(previous_values + delta_value))
        }
    }

    pub fn get_vector_compare_decreased_by<const N: usize>(scan_parameters: &ScanParameters) -> fn(*const u8, *const u8, *const u8) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        |current_values_ptr: *const u8, previous_values_ptr: *const u8, delta_ptr: *const u8| unsafe {
            let current_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::from_array(ptr::read_unaligned(
                current_values_ptr as *const [SwapCompatibleType; N],
            ))));
            let previous_values: Simd<PrimitiveType, N> = VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(
                previous_values_ptr as *const SwapCompatibleType,
            ))));
            let delta_value: Simd<PrimitiveType, N> =
                VectorGenerics::transmute(&Simd::swap_bytes(Simd::splat(ptr::read_unaligned(delta_ptr as *const SwapCompatibleType))));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(previous_values - delta_value))
        }
    }
}

impl VectorComparable for DataTypeU64be {
    fn get_vector_compare_equal_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate64 {
        DataTypeU64beVector::get_vector_compare_equal(scan_parameters)
    }

    fn get_vector_compare_equal_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate32 {
        DataTypeU64beVector::get_vector_compare_equal(scan_parameters)
    }

    fn get_vector_compare_equal_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate16 {
        DataTypeU64beVector::get_vector_compare_equal(scan_parameters)
    }

    fn get_vector_compare_not_equal_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate64 {
        DataTypeU64beVector::get_vector_compare_not_equal(scan_parameters)
    }

    fn get_vector_compare_not_equal_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate32 {
        DataTypeU64beVector::get_vector_compare_not_equal(scan_parameters)
    }

    fn get_vector_compare_not_equal_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate16 {
        DataTypeU64beVector::get_vector_compare_not_equal(scan_parameters)
    }

    fn get_vector_compare_greater_than_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate64 {
        DataTypeU64beVector::get_vector_compare_greater_than(scan_parameters)
    }

    fn get_vector_compare_greater_than_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate32 {
        DataTypeU64beVector::get_vector_compare_greater_than(scan_parameters)
    }

    fn get_vector_compare_greater_than_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate16 {
        DataTypeU64beVector::get_vector_compare_greater_than(scan_parameters)
    }

    fn get_vector_compare_greater_than_or_equal_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate64 {
        DataTypeU64beVector::get_vector_compare_greater_than_or_equal(scan_parameters)
    }

    fn get_vector_compare_greater_than_or_equal_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate32 {
        DataTypeU64beVector::get_vector_compare_greater_than_or_equal(scan_parameters)
    }

    fn get_vector_compare_greater_than_or_equal_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate16 {
        DataTypeU64beVector::get_vector_compare_greater_than_or_equal(scan_parameters)
    }

    fn get_vector_compare_less_than_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate64 {
        DataTypeU64beVector::get_vector_compare_less_than(scan_parameters)
    }

    fn get_vector_compare_less_than_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate32 {
        DataTypeU64beVector::get_vector_compare_less_than(scan_parameters)
    }

    fn get_vector_compare_less_than_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate16 {
        DataTypeU64beVector::get_vector_compare_less_than(scan_parameters)
    }

    fn get_vector_compare_less_than_or_equal_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate64 {
        DataTypeU64beVector::get_vector_compare_less_than_or_equal(scan_parameters)
    }

    fn get_vector_compare_less_than_or_equal_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate32 {
        DataTypeU64beVector::get_vector_compare_less_than_or_equal(scan_parameters)
    }

    fn get_vector_compare_less_than_or_equal_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnImmediate16 {
        DataTypeU64beVector::get_vector_compare_less_than_or_equal(scan_parameters)
    }

    fn get_vector_compare_changed_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative64 {
        DataTypeU64beVector::get_vector_compare_changed(scan_parameters)
    }

    fn get_vector_compare_changed_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative32 {
        DataTypeU64beVector::get_vector_compare_changed(scan_parameters)
    }

    fn get_vector_compare_changed_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative16 {
        DataTypeU64beVector::get_vector_compare_changed(scan_parameters)
    }

    fn get_vector_compare_unchanged_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative64 {
        DataTypeU64beVector::get_vector_compare_unchanged(scan_parameters)
    }

    fn get_vector_compare_unchanged_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative32 {
        DataTypeU64beVector::get_vector_compare_unchanged(scan_parameters)
    }

    fn get_vector_compare_unchanged_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative16 {
        DataTypeU64beVector::get_vector_compare_unchanged(scan_parameters)
    }

    fn get_vector_compare_increased_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative64 {
        DataTypeU64beVector::get_vector_compare_increased(scan_parameters)
    }

    fn get_vector_compare_increased_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative32 {
        DataTypeU64beVector::get_vector_compare_increased(scan_parameters)
    }

    fn get_vector_compare_increased_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative16 {
        DataTypeU64beVector::get_vector_compare_increased(scan_parameters)
    }

    fn get_vector_compare_decreased_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative64 {
        DataTypeU64beVector::get_vector_compare_decreased(scan_parameters)
    }

    fn get_vector_compare_decreased_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative32 {
        DataTypeU64beVector::get_vector_compare_decreased(scan_parameters)
    }

    fn get_vector_compare_decreased_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnRelative16 {
        DataTypeU64beVector::get_vector_compare_decreased(scan_parameters)
    }

    fn get_vector_compare_increased_by_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnDelta64 {
        DataTypeU64beVector::get_vector_compare_increased_by(scan_parameters)
    }

    fn get_vector_compare_increased_by_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnDelta32 {
        DataTypeU64beVector::get_vector_compare_increased_by(scan_parameters)
    }

    fn get_vector_compare_increased_by_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnDelta16 {
        DataTypeU64beVector::get_vector_compare_increased_by(scan_parameters)
    }

    fn get_vector_compare_decreased_by_64(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnDelta64 {
        DataTypeU64beVector::get_vector_compare_decreased_by(scan_parameters)
    }

    fn get_vector_compare_decreased_by_32(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnDelta32 {
        DataTypeU64beVector::get_vector_compare_decreased_by(scan_parameters)
    }

    fn get_vector_compare_decreased_by_16(
        &self,
        scan_parameters: &ScanParameters,
    ) -> VectorCompareFnDelta16 {
        DataTypeU64beVector::get_vector_compare_decreased_by(scan_parameters)
    }
}
