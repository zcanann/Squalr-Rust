use crate::structures::data_types::generics::vector_generics::VectorGenerics;
use crate::structures::scanning::scan_parameters_global::ScanParametersGlobal;
use crate::structures::scanning::scan_parameters_local::ScanParametersLocal;
use std::ops::{Add, Sub};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
use std::{mem, ptr};

pub trait ReadFloatBigEndian: Sized + SimdElement {
    fn read_float_be(value_ptr: *const u8) -> Self;

    fn read_float_vector_be<const N: usize>(value_ptr: *const u8) -> Simd<Self, N>
    where
        LaneCount<N>: SupportedLaneCount;
}

impl ReadFloatBigEndian for f32 {
    fn read_float_be(value_ptr: *const u8) -> Self {
        unsafe { mem::transmute::<u32, f32>(u32::swap_bytes(ptr::read_unaligned(value_ptr as *const u32))) }
    }

    fn read_float_vector_be<const N: usize>(value_ptr: *const u8) -> Simd<Self, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        unsafe { VectorGenerics::transmute(&SimdUint::swap_bytes(Simd::from_array(ptr::read_unaligned(value_ptr as *const [u32; N])))) }
    }
}

impl ReadFloatBigEndian for f64 {
    fn read_float_be(value_ptr: *const u8) -> Self {
        unsafe { mem::transmute::<u64, f64>(u64::swap_bytes(ptr::read_unaligned(value_ptr as *const u64))) }
    }

    fn read_float_vector_be<const N: usize>(value_ptr: *const u8) -> Simd<Self, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        unsafe { VectorGenerics::transmute(&SimdUint::swap_bytes(Simd::from_array(ptr::read_unaligned(value_ptr as *const [u64; N])))) }
    }
}

pub struct VectorComparisonsFloatBigEndian {}

// JIRA: Tolerance support!
impl VectorComparisonsFloatBigEndian {
    pub fn get_vector_compare_equal<const N: usize, PrimitiveType: SimdElement + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialEq,
    {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            // Optimization: no endian byte swap required for immediate or current values.
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = Simd::splat(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_values_ptr| {
                    let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));

                    VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(immediate_value))
                }))
            }
        } else {
            None
        }
    }

    pub fn get_vector_compare_not_equal<const N: usize, PrimitiveType: SimdElement + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialEq,
    {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            // Optimization: no endian byte swap required for immediate or current values.
            unsafe {
                let immediate_value_ptr = immediate_value.as_ptr();
                let immediate_value = Simd::splat(ptr::read_unaligned(immediate_value_ptr as *const PrimitiveType));

                Some(Box::new(move |current_values_ptr| {
                    let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));

                    VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_ne(immediate_value))
                }))
            }
        } else {
            None
        }
    }

    pub fn get_vector_compare_greater_than<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialOrd,
    {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            let immediate_value_ptr = immediate_value.as_ptr();
            let immediate_value: Simd<PrimitiveType, N> = Simd::splat(ReadFloatBigEndian::read_float_be(immediate_value_ptr));

            Some(Box::new(move |current_values_ptr| {
                let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);

                VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_gt(immediate_value))
            }))
        } else {
            None
        }
    }

    pub fn get_vector_compare_greater_than_or_equal<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialOrd,
    {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            let immediate_value_ptr = immediate_value.as_ptr();
            let immediate_value: Simd<PrimitiveType, N> = Simd::splat(ReadFloatBigEndian::read_float_be(immediate_value_ptr));

            Some(Box::new(move |current_values_ptr| {
                let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);

                VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_ge(immediate_value))
            }))
        } else {
            None
        }
    }

    pub fn get_vector_compare_less_than<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialOrd,
    {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            let immediate_value_ptr = immediate_value.as_ptr();
            let immediate_value: Simd<PrimitiveType, N> = Simd::splat(ReadFloatBigEndian::read_float_be(immediate_value_ptr));

            Some(Box::new(move |current_values_ptr| {
                let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);

                VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_lt(immediate_value))
            }))
        } else {
            None
        }
    }

    pub fn get_vector_compare_less_than_or_equal<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialOrd,
    {
        if let Some(immediate_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            let immediate_value_ptr = immediate_value.as_ptr();
            let immediate_value: Simd<PrimitiveType, N> = Simd::splat(ReadFloatBigEndian::read_float_be(immediate_value_ptr));

            Some(Box::new(move |current_values_ptr| {
                let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);

                VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_le(immediate_value))
            }))
        } else {
            None
        }
    }

    pub fn get_vector_compare_changed<const N: usize, PrimitiveType: SimdElement + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8, *const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialEq,
    {
        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            // Optimization: no endian byte swap required.
            let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));
            let previous_values = Simd::from_array(ptr::read_unaligned(previous_values_ptr as *const [PrimitiveType; N]));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_ne(previous_values))
        }))
    }

    pub fn get_vector_compare_unchanged<const N: usize, PrimitiveType: SimdElement + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8, *const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialEq,
    {
        Some(Box::new(move |current_values_ptr, previous_values_ptr| unsafe {
            // Optimization: no endian byte swap required.
            let current_values = Simd::from_array(ptr::read_unaligned(current_values_ptr as *const [PrimitiveType; N]));
            let previous_values = Simd::from_array(ptr::read_unaligned(previous_values_ptr as *const [PrimitiveType; N]));

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(previous_values))
        }))
    }

    pub fn get_vector_compare_increased<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8, *const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialOrd,
    {
        Some(Box::new(move |current_values_ptr, previous_values_ptr| {
            let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);
            let previous_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(previous_values_ptr);

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_gt(previous_values))
        }))
    }

    pub fn get_vector_compare_decreased<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        _scan_parameters_global: &ScanParametersGlobal,
        _scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8, *const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialOrd,
    {
        Some(Box::new(move |current_values_ptr, previous_values_ptr| {
            let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);
            let previous_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(previous_values_ptr);

            VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_lt(previous_values))
        }))
    }

    pub fn get_vector_compare_increased_by<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8, *const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialEq + Add<Simd<PrimitiveType, N>, Output = Simd<PrimitiveType, N>>,
    {
        if let Some(delta_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            let delta_value_ptr = delta_value.as_ptr();
            let delta_value: Simd<PrimitiveType, N> = Simd::splat(ReadFloatBigEndian::read_float_be(delta_value_ptr));

            Some(Box::new(move |current_values_ptr, previous_values_ptr| {
                let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);
                let previous_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(previous_values_ptr);

                VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(previous_values.add(delta_value)))
            }))
        } else {
            None
        }
    }

    pub fn get_vector_compare_decreased_by<const N: usize, PrimitiveType: SimdElement + ReadFloatBigEndian + 'static>(
        scan_parameters_global: &ScanParametersGlobal,
        scan_parameters_local: &ScanParametersLocal,
    ) -> Option<Box<dyn Fn(*const u8, *const u8) -> Simd<u8, N>>>
    where
        LaneCount<N>: SupportedLaneCount,
        Simd<PrimitiveType, N>: SimdPartialEq + Sub<Simd<PrimitiveType, N>, Output = Simd<PrimitiveType, N>>,
    {
        if let Some(delta_value) = scan_parameters_global.deanonymize_immediate(scan_parameters_local.get_data_type()) {
            let delta_value_ptr = delta_value.as_ptr();
            let delta_value: Simd<PrimitiveType, N> = Simd::splat(ReadFloatBigEndian::read_float_be(delta_value_ptr));

            Some(Box::new(move |current_values_ptr, previous_values_ptr| {
                let current_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(current_values_ptr);
                let previous_values: Simd<PrimitiveType, N> = ReadFloatBigEndian::read_float_vector_be(previous_values_ptr);

                VectorGenerics::transmute_mask::<PrimitiveType, N>(&current_values.simd_eq(previous_values.sub(delta_value)))
            }))
        } else {
            None
        }
    }
}
