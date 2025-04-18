use crate::structures::memory::memory_alignment::MemoryAlignment;
use std::cmp::{Ord, Ordering};
use std::hash::{Hash, Hasher};

/// Defines a generic range of addresses, with no extra information.
/// This is the base type for many more specialized regions.
pub struct NormalizedRegion {
    base_address: u64,
    region_size: u64,
}

impl NormalizedRegion {
    pub fn new(
        base_address: u64,
        region_size: u64,
    ) -> Self {
        Self { base_address, region_size }
    }

    /// Gets the base/start address of this region.
    pub fn get_base_address(&self) -> u64 {
        self.base_address
    }

    /// Sets the base/start address of this region.
    pub fn set_base_address(
        &mut self,
        base_address: u64,
    ) {
        self.base_address = base_address;
    }

    /// Sets the base/start address of this region.
    pub fn set_base_address_retain_end_address(
        &mut self,
        base_address: u64,
    ) {
        let end_address = self.get_end_address();
        self.base_address = base_address;
        self.set_end_address(end_address);
    }

    /// Gets the size of this region.
    pub fn get_region_size(&self) -> u64 {
        self.region_size
    }

    /// Sets the size of this region.
    pub fn set_region_size(
        &mut self,
        region_size: u64,
    ) {
        self.region_size = region_size;
    }

    /// Gets the end address of this region.
    pub fn get_end_address(&self) -> u64 {
        self.base_address.saturating_add(self.region_size as u64)
    }

    /// Sets the end address of this region.
    pub fn set_end_address(
        &mut self,
        end_address: u64,
    ) {
        self.region_size = end_address.saturating_sub(self.base_address) as u64;
    }

    pub fn set_alignment(
        &mut self,
        alignment: MemoryAlignment,
    ) {
        let alignment_value = alignment as u64;

        if alignment_value <= 0 || self.base_address % alignment as u64 == 0 {
            return;
        }

        let end_address = self.get_end_address();
        self.base_address -= self.base_address % alignment as u64;
        self.base_address += alignment as u64;
        self.set_end_address(end_address);
    }

    /// Gets a value indicating whether the provided address is contained inclusively by this region.
    pub fn contains_address(
        &self,
        address: u64,
    ) -> bool {
        address >= self.base_address && address <= self.get_end_address()
    }

    /// Expands this snapshot region in both directions, saturating, in both directions. For example,
    /// expanding by 1 will cause the base address to decrease by 1, and the end address to increase by 1.
    pub fn expand(
        &mut self,
        expand_size: u64,
    ) {
        self.base_address = self.base_address.saturating_sub(expand_size);
        self.region_size = self.region_size.saturating_add(expand_size * 2);
    }
}

impl PartialEq for NormalizedRegion {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.base_address == other.base_address && self.region_size == other.region_size
    }
}

impl Eq for NormalizedRegion {}

impl PartialOrd for NormalizedRegion {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NormalizedRegion {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.base_address.cmp(&other.base_address)
    }
}

impl Hash for NormalizedRegion {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.base_address.hash(state);
        self.region_size.hash(state);
    }
}
