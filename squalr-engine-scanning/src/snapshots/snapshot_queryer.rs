use crate::scan_settings::ScanSettings;
use crate::snapshots::snapshot::Snapshot;
use crate::snapshots::snapshot_region::SnapshotRegion;

use squalr_engine_memory::memory_queryer::memory_protection_enum::MemoryProtectionEnum;
use squalr_engine_memory::memory_queryer::MemoryQueryer;
use squalr_engine_memory::memory_queryer::memory_queryer_trait::IMemoryQueryer;
use squalr_engine_memory::memory_queryer::memory_type_enum::MemoryTypeEnum;
use squalr_engine_memory::memory_queryer::region_bounds_handling::RegionBoundsHandling;
use sysinfo::Pid;
use std::collections::HashSet;

bitflags::bitflags! {
    pub struct SnapshotRetrievalMode: u32 {
        const FROM_SETTINGS = 0b0001;
        const FROM_USER_MODE_MEMORY = 0b0010;
        const FROM_HEAPS = 0b0100;
        const FROM_STACK = 0b1000;
        const FROM_MODULES = 0b10000;
    }
}

pub struct SnapshotQueryer;

impl SnapshotQueryer {

    // TODO: Support middle-ware for emulator types to filter down the address space
    pub fn get_snapshot(
        process_id: &Pid,
        snapshot_creation_mode: SnapshotRetrievalMode,
    ) -> Snapshot {
        match snapshot_creation_mode {
            SnapshotRetrievalMode::FROM_SETTINGS => {
                SnapshotQueryer::create_snapshot_from_settings(process_id)
            }
            SnapshotRetrievalMode::FROM_USER_MODE_MEMORY => {
                SnapshotQueryer::create_snapshot_from_usermode_memory(process_id)
            }
            SnapshotRetrievalMode::FROM_MODULES => {
                SnapshotQueryer::create_snapshot_from_modules(process_id)
            }
            SnapshotRetrievalMode::FROM_HEAPS => {
                SnapshotQueryer::create_snapshot_from_heaps(process_id)
            }
            SnapshotRetrievalMode::FROM_STACK => unimplemented!(),
            _ => {
                eprintln!("Unknown snapshot retrieval mode");
                Snapshot::new(String::from(""), vec![])
            }
        }
    }

    pub fn create_snapshot_by_address_range(
        process_id: &Pid,
        start_address: u64,
        end_address: u64,
    ) -> Snapshot {
        let required_page_flags = MemoryProtectionEnum::empty();
        let excluded_page_flags = MemoryProtectionEnum::empty();
        let allowed_type_flags = MemoryTypeEnum::NONE | MemoryTypeEnum::PRIVATE | MemoryTypeEnum::IMAGE | MemoryTypeEnum::MAPPED;
        let bounds_handling = RegionBoundsHandling::Resize;
    
        let normalized_regions = MemoryQueryer::instance().get_virtual_pages(
            process_id,
            required_page_flags,
            excluded_page_flags,
            allowed_type_flags,
            start_address,
            end_address,
            bounds_handling,
        );
    
        let mut snapshot_regions = Vec::new();
        for normalized_region in normalized_regions {
            let mut snapshot_region = SnapshotRegion::new_from_normalized_region(normalized_region);
            snapshot_region.align(ScanSettings::instance().get_alignment() as u32);
            snapshot_regions.push(snapshot_region);
        }
    
        Snapshot::new(String::from(""), snapshot_regions)
    }
    

    fn create_snapshot_from_usermode_memory(process_id: &Pid) -> Snapshot {
        let required_page_flags = MemoryProtectionEnum::empty();
        let excluded_page_flags = MemoryProtectionEnum::empty();
        let allowed_type_flags = MemoryTypeEnum::NONE | MemoryTypeEnum::PRIVATE | MemoryTypeEnum::IMAGE;
        let start_address = 0;
        let end_address = MemoryQueryer::instance().get_max_usermode_address(process_id);
    
        let normalized_regions = MemoryQueryer::instance().get_virtual_pages(
            process_id,
            required_page_flags,
            excluded_page_flags,
            allowed_type_flags,
            start_address,
            end_address,
            RegionBoundsHandling::Exclude
        );
    
        let mut snapshot_regions = Vec::new();
        for normalized_region in normalized_regions {
            let mut snapshot_region = SnapshotRegion::new_from_normalized_region(normalized_region);
            snapshot_region.align(ScanSettings::instance().get_alignment() as u32);
            snapshot_regions.push(snapshot_region);
        }
    
        Snapshot::new(String::from(""), snapshot_regions)
    }
    

    fn create_snapshot_from_settings(process_id: &Pid) -> Snapshot {
        let required_page_flags = SnapshotQueryer::get_required_protection_settings();
        let excluded_page_flags = SnapshotQueryer::get_excluded_protection_settings();
        let allowed_type_flags = SnapshotQueryer::get_allowed_type_settings();
    
        let (start_address, end_address) = if ScanSettings::instance().is_usermode() {
            (0, MemoryQueryer::instance().get_max_usermode_address(process_id))
        } else {
            (
                ScanSettings::instance().get_start_address(),
                ScanSettings::instance().get_end_address(),
            )
        };
    
        let normalized_regions = MemoryQueryer::instance().get_virtual_pages(
            process_id,
            required_page_flags,
            excluded_page_flags,
            allowed_type_flags,
            start_address,
            end_address,
            RegionBoundsHandling::Exclude,
        );
    
        let mut snapshot_regions = Vec::new();
        for normalized_region in normalized_regions {
            let mut snapshot_region = SnapshotRegion::new_from_normalized_region(normalized_region);
            snapshot_region.align(ScanSettings::instance().get_alignment() as u32);
            snapshot_regions.push(snapshot_region);
        }
    
        Snapshot::new(String::from(""), snapshot_regions)
    }
    
    fn create_snapshot_from_modules(process_id: &Pid) -> Snapshot {
        // Note that we use into_base_region to extract the base region without copying, instead taking ownership
        let module_regions: Vec<SnapshotRegion> = MemoryQueryer::instance()
            .get_modules(process_id)
            .into_iter()
            .map(|module| SnapshotRegion::new_from_normalized_region(module.into_base_region()))
            .collect();
    
        Snapshot::new(String::from(""), module_regions)
    }    

    fn create_snapshot_from_heaps(process_id: &Pid) -> Snapshot {
        let snapshot = SnapshotQueryer::create_snapshot_from_usermode_memory(process_id);
        let modules: HashSet<u64> = MemoryQueryer::instance()
            .get_modules(process_id)
            .into_iter()
            .map(|module| module.get_base_address())
            .collect();
    
        let required_page_flags = MemoryProtectionEnum::empty();
        let excluded_page_flags = MemoryProtectionEnum::empty();
        let allowed_type_flags = MemoryTypeEnum::NONE | MemoryTypeEnum::PRIVATE | MemoryTypeEnum::IMAGE;
        let start_address = 0;
        let end_address = MemoryQueryer::instance().get_max_usermode_address(process_id);
    
        let virtual_pages = MemoryQueryer::instance().get_virtual_pages(
            process_id,
            required_page_flags,
            excluded_page_flags,
            allowed_type_flags,
            start_address,
            end_address,
            RegionBoundsHandling::Exclude,
        );
    
        let memory_regions: Vec<SnapshotRegion> = virtual_pages
            .into_iter()
            .filter(|page| !modules.contains(&page.get_base_address()))
            .map(|page| {
                let mut snapshot_region = SnapshotRegion::new_from_normalized_region(page);
                snapshot_region.align(ScanSettings::instance().get_alignment() as u32);
                snapshot_region
            })
            .collect();
    
        Snapshot::new(String::from(""), memory_regions)
    }
    

    fn get_allowed_type_settings() -> MemoryTypeEnum {
        let mut result = MemoryTypeEnum::empty();

        if ScanSettings::instance().get_memory_type_none() {
            result |= MemoryTypeEnum::NONE;
        }

        if ScanSettings::instance().get_memory_type_private() {
            result |= MemoryTypeEnum::PRIVATE;
        }

        if ScanSettings::instance().get_memory_type_image() {
            result |= MemoryTypeEnum::IMAGE;
        }

        if ScanSettings::instance().get_memory_type_mapped() {
            result |= MemoryTypeEnum::MAPPED;
        }

        return result;
    }

    fn get_required_protection_settings() -> MemoryProtectionEnum {
        let mut result = MemoryProtectionEnum::empty();

        if ScanSettings::instance().get_required_write() {
            result |= MemoryProtectionEnum::WRITE;
        }

        if ScanSettings::instance().get_required_execute() {
            result |= MemoryProtectionEnum::EXECUTE;
        }

        if ScanSettings::instance().get_required_copy_on_write() {
            result |= MemoryProtectionEnum::COPY_ON_WRITE;
        }

        return result;
    }

    fn get_excluded_protection_settings() -> MemoryProtectionEnum {
        let mut result = MemoryProtectionEnum::empty();

        if ScanSettings::instance().get_excluded_write() {
            result |= MemoryProtectionEnum::WRITE;
        }

        if ScanSettings::instance().get_excluded_execute() {
            result |= MemoryProtectionEnum::EXECUTE;
        }

        if ScanSettings::instance().get_excluded_copy_on_write() {
            result |= MemoryProtectionEnum::COPY_ON_WRITE;
        }

        return result;
    }
}
