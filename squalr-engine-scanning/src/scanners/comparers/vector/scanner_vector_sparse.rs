
use crate::filters::snapshot_region_filter::SnapshotRegionFilter;
use crate::scanners::comparers::snapshot_scanner::Scanner;
use crate::scanners::comparers::vector::encoder::scanner_vector_encoder::ScannerVectorEncoder;
use crate::scanners::parameters::scan_filter_parameters::ScanFilterParameters;
use crate::scanners::parameters::scan_parameters::ScanParameters;
use crate::snapshots::snapshot_region::SnapshotRegion;
use squalr_engine_memory::memory_alignment::MemoryAlignment;
use std::sync::Once;
use std::simd::{u8x16, u8x32, u8x64};

pub struct ScannerVectorSparse<const VECTOR_SIZE_BITS: usize>;

macro_rules! impl_scanner_vector_sparse {
    ($bit_width:expr, $simd_type:ty, $vector_size_bytes:expr) => {
        impl ScannerVectorSparse<$bit_width> {
            pub fn get_instance() -> &'static ScannerVectorSparse<$bit_width> {
                static mut INSTANCE: Option<ScannerVectorSparse<$bit_width>> = None;
                static INIT: Once = Once::new();

                unsafe {
                    INIT.call_once(|| {
                        let instance = ScannerVectorSparse::<$bit_width>::new();
                        INSTANCE = Some(instance);
                    });

                    INSTANCE.as_ref().unwrap_unchecked()
                }
            }

            fn new() -> Self {
                Self {}
            }

            fn get_sparse_mask(memory_alignment: MemoryAlignment) -> $simd_type {
                match memory_alignment {
                    // This will produce a byte pattern of <0xFF, 0xFF...>.
                    MemoryAlignment::Alignment1 => {
                        <$simd_type>::splat(0xFF)
                    },
                    // This will produce a byte pattern of <0x00, 0xFF...>.
                    MemoryAlignment::Alignment2 => {
                        let mut mask = [0u8; $vector_size_bytes];
                        for i in (1..$vector_size_bytes).step_by(2) {
                            mask[i] = 0xFF;
                        }
                        <$simd_type>::from_array(mask)
                    }
                    // This will produce a byte pattern of <0x00, 0x00, 0x00, 0xFF...>.
                    MemoryAlignment::Alignment4 => {
                        let mut mask = [0u8; $vector_size_bytes];
                        for i in (3..$vector_size_bytes).step_by(4) {
                            mask[i] = 0xFF;
                        }
                        <$simd_type>::from_array(mask)
                    }
                    // This will produce a byte pattern of <0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF...>.
                    MemoryAlignment::Alignment8 => {
                        let mut mask = [0u8; $vector_size_bytes];
                        for i in (7..$vector_size_bytes).step_by(8) {
                            mask[i] = 0xFF;
                        }
                        <$simd_type>::from_array(mask)
                    }
                }
            }
            
        }
    };
}

macro_rules! impl_scanner_for_vector_sparse {
    ($bit_width:expr, $simd_type:ty) => {
        impl Scanner for ScannerVectorSparse<$bit_width> {
            fn scan_region(
                &self,
                snapshot_region: &SnapshotRegion,
                snapshot_region_filter: &SnapshotRegionFilter,
                scan_parameters: &ScanParameters,
                scan_filter_parameters: &ScanFilterParameters,
            ) -> Vec<SnapshotRegionFilter> {
                let data_type = scan_filter_parameters.get_data_type();
                let data_type_size = data_type.size_in_bytes();
                let memory_alignment = scan_filter_parameters.get_memory_alignment_or_default();
                let sparse_element_count = snapshot_region_filter.get_element_count(memory_alignment, data_type_size);
                let encoder = ScannerVectorEncoder::<$bit_width>::get_instance();
                let current_value_pointer = snapshot_region.get_current_values_pointer(&snapshot_region_filter);
                let previous_value_pointer = snapshot_region.get_previous_values_pointer(&snapshot_region_filter);

                // This mask automatically captures all in-between elements. For example, scanning for Byte 0 with an alignment of 2-bytes
                // against <0, 24, 0, 43> would all return true, due to this mask of <0, 255, 0, 255>. Scan results will automatically skip
                // over the unwanted elements based on alignment. In fact, we do NOT want to break this into two separate snapshot regions,
                // since this would be incredibly inefficient. So in this example, we would return a single snapshot region of size 4, and the scan results would iterate by 2.
                let sparse_mask = Self::get_sparse_mask(memory_alignment);

                let results = encoder.encode(
                    current_value_pointer,
                    previous_value_pointer,
                    scan_parameters,
                    scan_filter_parameters,
                    snapshot_region_filter.get_base_address(),
                    sparse_element_count,
                    sparse_mask,
                );

                return results;
            }
        }
    };
}

// Create implementations for 128, 256, and 512 SIMD vector widths.
impl_scanner_vector_sparse!(128, u8x16, 16);
impl_scanner_vector_sparse!(256, u8x32, 32);
impl_scanner_vector_sparse!(512, u8x64, 64);

impl_scanner_for_vector_sparse!(128, u8x16);
impl_scanner_for_vector_sparse!(256, u8x32);
impl_scanner_for_vector_sparse!(512, u8x64);
