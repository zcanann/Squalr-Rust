use crate::memory_queryer::memory_protection_enum::MemoryProtectionEnum;
use crate::memory_queryer::memory_queryer_trait::IMemoryQueryer;
use crate::memory_queryer::memory_type_enum::MemoryTypeEnum;
use crate::memory_queryer::region_bounds_handling::RegionBoundsHandling;
use olorin_engine_api::structures::memory::normalized_module::NormalizedModule;
use olorin_engine_api::structures::memory::normalized_region::NormalizedRegion;
use olorin_engine_common::logging::log_level::LogLevel;
use olorin_engine_common::logging::logger::Logger;
use olorin_engine_processes::process_info::Bitness;
use olorin_engine_processes::process_info::OpenedProcessInfo;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct AndroidMemoryQueryer;

/// Helper struct to hold a single line of `/proc/<process_id>/maps` data
struct ProcMapRegion {
    start: u64,
    end: u64,
    perms: String,
    offset: u64,
    dev: String,
    inode: u64,
    pathname: String,
}

impl AndroidMemoryQueryer {
    pub fn new() -> Self {
        AndroidMemoryQueryer
    }

    /// Reads `/proc/<process_id>/maps` directly from the filesystem and parses each line.
    fn parse_proc_maps(process_id: i32) -> std::io::Result<Vec<ProcMapRegion>> {
        // Instead of using “cat /proc/<process_id>/maps” via su, read the file directly:
        let path = format!("/proc/{}/maps", process_id);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut regions = vec![];

        // Each line in `/proc/<process_id>/maps` typically looks like:
        // 00400000-00452000 r-xp 00000000 fc:01 1234   /system/bin/app_process32
        for line_result in reader.lines() {
            let line = line_result?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            // We expect at least: address-range, perms, offset, dev, inode, [pathname...]
            if parts.len() < 5 {
                continue;
            }

            let range_part = parts[0];
            let perms_part = parts[1];
            let offset_part = parts[2];
            let dev_part = parts[3];
            let inode_part = parts[4];
            // If there's more, join the rest as the pathname
            let pathname_part = if parts.len() > 5 { parts[5..].join(" ") } else { String::new() };

            // Parse address range
            let mut range_split = range_part.split('-');
            let start_str = range_split.next().unwrap_or("0");
            let end_str = range_split.next().unwrap_or("0");

            let start = u64::from_str_radix(start_str, 16).unwrap_or(0);
            let end = u64::from_str_radix(end_str, 16).unwrap_or(0);
            let offset = u64::from_str_radix(offset_part, 16).unwrap_or(0);
            let inode = inode_part.parse::<u64>().unwrap_or(0);

            let region = ProcMapRegion {
                start,
                end,
                perms: perms_part.to_string(),
                offset,
                dev: dev_part.to_string(),
                inode,
                pathname: pathname_part,
            };

            regions.push(region);
        }

        Ok(regions)
    }

    /// Convert the 4-character perms (e.g. "r-xp") from /proc/<process_id>/maps into `MemoryProtectionEnum`.
    fn to_memory_protection(perms: &str) -> MemoryProtectionEnum {
        let mut prot = MemoryProtectionEnum::empty();

        if perms.len() >= 1 && &perms[0..1] == "r" {
            prot |= MemoryProtectionEnum::READ;
        }
        if perms.len() >= 2 && &perms[1..2] == "w" {
            prot |= MemoryProtectionEnum::WRITE;
        }
        if perms.len() >= 3 && &perms[2..3] == "x" {
            prot |= MemoryProtectionEnum::EXECUTE;
        }

        prot
    }

    /// Convert /proc/maps region to your `MemoryTypeEnum`. This is heuristic only.
    fn to_memory_type(
        perms: &str,
        pathname: &str,
    ) -> MemoryTypeEnum {
        let mut mem_type = MemoryTypeEnum::empty();

        // If 'p' is at index 3 => private
        // If 's' is at index 3 => shared
        if perms.len() >= 4 && &perms[3..4] == "p" {
            mem_type |= MemoryTypeEnum::PRIVATE;
        } else {
            mem_type |= MemoryTypeEnum::MAPPED;
        }

        // Heuristic for libraries, etc.
        if pathname.ends_with(".so") || pathname.contains(".so") {
            mem_type |= MemoryTypeEnum::IMAGE;
        }

        mem_type
    }

    /// Helper to see if a region meets the "required" and "excluded" protection criteria.
    fn match_protection(
        region_prot: &MemoryProtectionEnum,
        required: &MemoryProtectionEnum,
        excluded: &MemoryProtectionEnum,
    ) -> bool {
        // Example logic: require all bits in `required`, disallow any bits in `excluded`.
        // Adjust to your needs.
        /*
        if !required.is_empty() && !region_prot.contains(*required) {
            return false;
        }
        if !(region_prot & *excluded).is_empty() {
            return false;
        }
        */
        true
    }

    /// Helper to see if a region's type meets the "allowed" type flags.
    fn match_type(
        region_type: &MemoryTypeEnum,
        allowed: &MemoryTypeEnum,
    ) -> bool {
        // Example logic: region_type must share at least one bit with `allowed`.
        /*
        if allowed.is_empty() {
            return true; // or false, depending on your semantics
        }
        !(region_type & *allowed).is_empty()
        */
        true
    }

    /// Adjust a region to the requested address range, if needed.
    fn clamp_region(
        region: &ProcMapRegion,
        start_address: u64,
        end_address: u64,
        bounds_handling: RegionBoundsHandling,
    ) -> Option<(u64, u64)> {
        let rstart = region.start;
        let rend = region.end;

        // If no overlap, handle exclude/resize/include
        if rstart >= end_address || rend <= start_address {
            return match bounds_handling {
                RegionBoundsHandling::Exclude => None,
                RegionBoundsHandling::Resize => None,
                RegionBoundsHandling::Include => Some((rstart, rend)),
            };
        }

        // We do have overlap, so handle each mode:
        match bounds_handling {
            RegionBoundsHandling::Exclude => {
                // Only include if it's fully within start_address..end_address
                if rstart < start_address || rend > end_address {
                    None
                } else {
                    Some((rstart, rend))
                }
            }
            RegionBoundsHandling::Resize => {
                let clipped_start = rstart.max(start_address);
                let clipped_end = rend.min(end_address);
                if clipped_start < clipped_end {
                    Some((clipped_start, clipped_end))
                } else {
                    None
                }
            }
            RegionBoundsHandling::Include => Some((rstart, rend)),
        }
    }
}

impl IMemoryQueryer for AndroidMemoryQueryer {
    fn get_virtual_pages(
        &self,
        process_info: &OpenedProcessInfo,
        required_protection: MemoryProtectionEnum,
        excluded_protection: MemoryProtectionEnum,
        allowed_types: MemoryTypeEnum,
        start_address: u64,
        end_address: u64,
        region_bounds_handling: RegionBoundsHandling,
    ) -> Vec<NormalizedRegion> {
        let process_id_i32 = process_info.process_id as i32;

        let regions_result = Self::parse_proc_maps(process_id_i32);
        let Ok(regions) = regions_result else {
            Logger::log(LogLevel::Info, "Failed to query memory regions via SU shell.", None);
            return vec![];
        };

        let mut out = vec![];

        for reg in regions {
            let protection = Self::to_memory_protection(&reg.perms);
            if !Self::match_protection(&protection, &required_protection, &excluded_protection) {
                continue;
            }

            let mem_type = Self::to_memory_type(&reg.perms, &reg.pathname);
            if !Self::match_type(&mem_type, &allowed_types) {
                continue;
            }

            if let Some((clamped_start, clamped_end)) = Self::clamp_region(&reg, start_address, end_address, region_bounds_handling) {
                let size = clamped_end.saturating_sub(clamped_start);
                if size > 0 {
                    out.push(NormalizedRegion::new(clamped_start, size));
                }
            }
        }

        out
    }

    fn get_all_virtual_pages(
        &self,
        process_info: &OpenedProcessInfo,
    ) -> Vec<NormalizedRegion> {
        // Just call `get_virtual_pages` with no filtering.
        self.get_virtual_pages(
            process_info,
            MemoryProtectionEnum::empty(),
            MemoryProtectionEnum::empty(),
            MemoryTypeEnum::all(),
            0,
            std::u64::MAX,
            RegionBoundsHandling::Include,
        )
    }

    fn is_address_writable(
        &self,
        process_info: &OpenedProcessInfo,
        address: u64,
    ) -> bool {
        // If you need to check, parse maps again or cache them in a real-world scenario.
        // Then find the region containing 'address' and see if it has WRITE.
        // For demonstration, this is a naive re-parse each call:
        if let Ok(regions) = Self::parse_proc_maps(process_info.process_id as i32) {
            for reg in regions {
                if address >= reg.start && address < reg.end {
                    let protection = Self::to_memory_protection(&reg.perms);
                    return protection.contains(MemoryProtectionEnum::WRITE);
                }
            }
        }

        false
    }

    fn get_maximum_address(
        &self,
        process_info: &OpenedProcessInfo,
    ) -> u64 {
        match process_info.bitness {
            Bitness::Bit64 => 0x7FFFFFFFFFFF,
            Bitness::Bit32 => 0xFFFFFFFF,
        }
    }

    fn get_min_usermode_address(
        &self,
        _process_info: &OpenedProcessInfo,
    ) -> u64 {
        0
    }

    fn get_max_usermode_address(
        &self,
        process_info: &OpenedProcessInfo,
    ) -> u64 {
        self.get_maximum_address(process_info)
    }

    fn get_modules(
        &self,
        process_info: &OpenedProcessInfo,
    ) -> Vec<NormalizedModule> {
        let process_id_i32 = process_info.process_id as i32;
        let regions_result = Self::parse_proc_maps(process_id_i32);
        let Ok(regions) = regions_result else {
            return vec![];
        };

        let mut modules = vec![];
        for reg in regions {
            // If perms has 'x' (execute) and pathname is not empty
            if reg.perms.len() >= 3 && &reg.perms[2..3] == "x" && !reg.pathname.is_empty() {
                let size = reg.end.saturating_sub(reg.start);
                modules.push(NormalizedModule::new(&reg.pathname, reg.start, size));
            }
        }
        modules
    }

    fn address_to_module(
        &self,
        process_info: &OpenedProcessInfo,
        address: u64,
    ) -> Option<(String, u64)> {
        let modules = self.get_modules(process_info);
        for m in modules {
            let start = m.get_base_address();
            let end = start + m.get_region_size();
            if address >= start && address < end {
                *module_name = m.get_name().to_string();
                return start;
            }
        }
        0
    }

    fn resolve_module(
        &self,
        process_info: &OpenedProcessInfo,
        identifier: &str,
    ) -> u64 {
        let modules = self.get_modules(process_info);
        for m in modules {
            if m.get_name().contains(identifier) || m.get_full_path().contains(identifier) {
                return m.get_base_address();
            }
        }
        0
    }
}
