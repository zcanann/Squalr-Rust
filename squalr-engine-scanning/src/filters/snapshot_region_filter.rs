use squalr_engine_memory::memory_alignment::MemoryAlignment;
use squalr_engine_memory::normalized_region::NormalizedRegion;
use std::cmp::max;

/// Defines a range of filtered memory within a snapshot region. These filters are created by
/// scans to narrow down on the desired addresses.
#[derive(Debug)]
pub struct SnapshotRegionFilter {
    filter_range: NormalizedRegion,
}

impl SnapshotRegionFilter {
    pub fn new(
        base_address: u64,
        get_size_in_bytes: u64
    ) -> Self {
        Self {
            filter_range: NormalizedRegion::new(base_address, get_size_in_bytes),
        }
    }
    
    pub fn get_base_address(
        &self,
    ) -> u64 {
        return self.filter_range.get_base_address();
    }

    pub fn set_base_address(
        &mut self,
        base_address: u64,
    ) {
        self.filter_range.set_base_address(base_address);
    }

    pub fn set_end_address(
        &mut self,
        end_address: u64,
    ) {
        self.filter_range.set_end_address(end_address);
    }

    pub fn get_end_address(
        &self,
    ) -> u64 {
        return self.filter_range.get_end_address();
    }
    
    pub fn get_region_size(
        &self,
    ) -> u64 {
        return self.filter_range.get_region_size();
    }

    pub fn get_element_count(
        &self,
        alignment: MemoryAlignment,
        data_type_size: u64
    ) -> u64 {
        
        let get_size_in_bytes = self.get_region_size();
        let misalignment = self.get_misaligned_starting_byte_count(alignment);
        let alignment: u64 = max(alignment as u64, 1);
        
        // If a filter is misaligned or an invalid size, something has gone horribly wrong and we want to debug it.
        debug_assert!(alignment > 0);
        debug_assert!(misalignment == 0);
        debug_assert!(get_size_in_bytes >= data_type_size);

        return get_size_in_bytes / alignment;
    }

    fn get_misaligned_starting_byte_count(
        &self,
        alignment: MemoryAlignment
    ) -> u64 {
        let alignment = max(alignment as u64, 1);
        let base_address = self.get_base_address();
        let misalignment = base_address % alignment;

        // Additional modulo to handle the case where misalignment is 0.
        return (alignment - misalignment) % alignment
    }
}
