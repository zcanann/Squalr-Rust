use crate::imemory_queryer::IMemoryQueryer;
use crate::normalized_region::NormalizedRegion;
use crate::normalized_module::NormalizedModule;
use crate::memory_protection_enum::MemoryProtectionEnum;
use crate::memory_type_enum::MemoryTypeEnum;
use crate::region_bounds_handling::RegionBoundsHandling;
use crate::windows::memory_basic_information_64::MemoryBasicInformation64;
use core::mem::size_of;
use std::collections::HashSet;
use std::ptr::null_mut;
use sysinfo::Pid;
use winapi::shared::minwindef::{ DWORD, HMODULE, LPVOID };
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModulesEx, GetModuleFileNameExA, GetModuleInformation, MODULEINFO, LIST_MODULES_ALL};
use winapi::um::memoryapi::VirtualQueryEx;
use winapi::um::winnt::{HANDLE, PAGE_READWRITE, PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_WRITECOPY, PAGE_EXECUTE_WRITECOPY,
    PROCESS_QUERY_INFORMATION, MEMORY_BASIC_INFORMATION, PROCESS_VM_READ};

pub struct WindowsMemoryQuery;

impl WindowsMemoryQuery {
    pub fn new() -> Self {
        Self
    }

    fn open_process(&self, process_id:  &Pid) -> HANDLE {
        unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id.as_u32()) }
    }

    fn virtual_pages(
        &self,
        process_handle: HANDLE,
        start_address: u64,
        end_address: u64,
        required_protection: u32,
        excluded_protection: u32,
        allowed_types: MemoryTypeEnum,
        region_bounds_handling: RegionBoundsHandling,
    ) -> Vec<MemoryBasicInformation64> {
        let mut regions = Vec::new();
        let mut address = start_address;
        let mut wrapped_around = false;

        if start_address >= end_address {
            return regions;
        }

        if region_bounds_handling == RegionBoundsHandling::Include || region_bounds_handling == RegionBoundsHandling::Resize {
            address = 0;
        }

        while address < end_address && !wrapped_around {
            let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
            let result = unsafe {
                VirtualQueryEx(
                    process_handle,
                    address as LPVOID,
                    &mut mbi,
                    size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };

            if result == 0 {
                break;
            }

            address += mbi.RegionSize as u64;
            if address < start_address {
                wrapped_around = true;
            }

            if mbi.State == winapi::um::winnt::MEM_FREE {
                continue;
            }

            if (mbi.Protect & required_protection == 0) || (mbi.Protect & excluded_protection != 0) {
                continue;
            }

            if (mbi.Protect & PAGE_EXECUTE == 0)
                && (mbi.Protect & PAGE_EXECUTE_READ == 0)
                && (mbi.Protect & PAGE_EXECUTE_READWRITE == 0)
                && (mbi.Protect & PAGE_EXECUTE_WRITECOPY == 0)
                && (mbi.Protect & PAGE_READWRITE == 0)
                && (mbi.Protect & PAGE_WRITECOPY == 0) {
                continue;
            }

            if (mbi.Protect & winapi::um::winnt::PAGE_NOACCESS != 0)
                || (mbi.Protect & winapi::um::winnt::PAGE_GUARD != 0) {
                continue;
            }

            match allowed_types {
                MemoryTypeEnum::NONE => {
                    if mbi.Type != winapi::um::winnt::MEM_PRIVATE {
                        continue;
                    }
                }
                MemoryTypeEnum::PRIVATE => {
                    if mbi.Type != winapi::um::winnt::MEM_PRIVATE {
                        continue;
                    }
                }
                MemoryTypeEnum::IMAGE => {
                    if mbi.Type != winapi::um::winnt::MEM_IMAGE {
                        continue;
                    }
                }
                MemoryTypeEnum::MAPPED => {
                    if mbi.Type != winapi::um::winnt::MEM_MAPPED {
                        continue;
                    }
                }
                _ => {
                    continue;
                }
            }

            let region_start_address = mbi.BaseAddress as u64;
            let region_end_address = region_start_address + mbi.RegionSize as u64;

            if region_start_address < start_address || region_end_address > end_address {
                match region_bounds_handling {
                    RegionBoundsHandling::Exclude => continue,
                    RegionBoundsHandling::Include => {}
                    RegionBoundsHandling::Resize => {
                        let new_start_address = start_address.max(region_start_address);
                        let new_end_address = end_address.min(region_end_address);
                        mbi.BaseAddress = new_start_address as LPVOID;
                        mbi.RegionSize = (new_end_address - new_start_address) as usize;
                    }
                }
            }

            regions.push(MemoryBasicInformation64 {
                BaseAddress: mbi.BaseAddress as u64,
                AllocationBase: mbi.AllocationBase as u64,
                AllocationProtect: mbi.AllocationProtect,
                RegionSize: mbi.RegionSize as u64,
                State: mbi.State,
                Protect: mbi.Protect,
                Type: mbi.Type,
            });
        }

        return regions;
    }

    fn get_protection_flags(&self, protection: &MemoryProtectionEnum) -> DWORD {
        let mut flags = 0;

        if protection.contains(MemoryProtectionEnum::WRITE) {
            flags |= PAGE_READWRITE | PAGE_EXECUTE_READWRITE;
        }

        if protection.contains(MemoryProtectionEnum::EXECUTE) {
            flags |= PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY;
        }

        if protection.contains(MemoryProtectionEnum::COPY_ON_WRITE) {
            flags |= PAGE_WRITECOPY | PAGE_EXECUTE_WRITECOPY;
        }

        return flags;
    }
}

impl IMemoryQueryer for WindowsMemoryQuery {
    fn get_virtual_pages(
        &self,
        process_id: &Pid,
        required_protection: MemoryProtectionEnum,
        excluded_protection: MemoryProtectionEnum,
        allowed_types: MemoryTypeEnum,
        start_address: u64,
        end_address: u64,
        region_bounds_handling: RegionBoundsHandling,
    ) -> HashSet<NormalizedRegion> {
        let process_handle = self.open_process(process_id);
        let required_flags = self.get_protection_flags(&required_protection);
        let excluded_flags = self.get_protection_flags(&excluded_protection);
        let mut regions = HashSet::new();

        let memory_info = self.virtual_pages(
            process_handle,
            start_address,
            end_address,
            required_flags,
            excluded_flags,
            allowed_types,
            region_bounds_handling,
        );

        for mbi in memory_info {
            regions.insert(NormalizedRegion::new(mbi.BaseAddress as u64, mbi.RegionSize as u64));
        }

        return regions;
    }

    fn get_all_virtual_pages(
        &self,
        process_id: &Pid,
    ) -> HashSet<NormalizedRegion> {
        let start_address = 0;
        let end_address = self.get_maximum_address(process_id);
        self.get_virtual_pages(
            process_id,
            MemoryProtectionEnum::NONE,
            MemoryProtectionEnum::NONE,
            MemoryTypeEnum::PRIVATE | MemoryTypeEnum::IMAGE | MemoryTypeEnum::MAPPED,
            start_address,
            end_address,
            RegionBoundsHandling::Exclude,
        )
    }

    fn is_address_writable(&self, process_id: &Pid, address: u64) -> bool {
        return false;
    }

    fn get_maximum_address(&self, process_id: &Pid) -> u64 {
        // Implement actual functionality here
        // TODO: Determine the maximum address based on the target architecture (x86 or x64)
        u64::MAX
    }

    fn get_min_usermode_address(&self, process_id: &Pid) -> u64 {
        // In windows, anything below this is not addressable by a normal program
        return 0x10000;
    }

    fn get_max_usermode_address(&self, process_id: &Pid) -> u64 {
        // Implement actual functionality here
        // TODO: Determine the maximum address based on the target architecture (x86 or x64)
        0x7FFFFFFF_FFFF // Example value for 64-bit Windows
    }

    fn get_modules(
        &self,
        process_id: &Pid,
    ) -> HashSet<NormalizedModule> {
        // Implement actual functionality here
        let process_handle = self.open_process(process_id);
        let mut modules = HashSet::new();

        let mut module_handles: [HMODULE; 1024] = [null_mut(); 1024];
        let mut cb_needed = 0;

        let result = unsafe {
            EnumProcessModulesEx(
                process_handle,
                module_handles.as_mut_ptr(),
                std::mem::size_of_val(&module_handles) as u32,
                &mut cb_needed,
                LIST_MODULES_ALL,
            )
        };

        if result == 0 {
            return modules;
        }

        let num_modules = cb_needed / std::mem::size_of::<HMODULE>() as u32;

        for i in 0..num_modules as usize {
            let mut module_name = vec![0u8; 1024];
            let result = unsafe {
                GetModuleFileNameExA(
                    process_handle,
                    module_handles[i],
                    module_name.as_mut_ptr() as *mut i8,
                    module_name.len() as u32,
                )
            };

            if result == 0 {
                continue;
            }

            let module_name = String::from_utf8_lossy(&module_name).to_string();
            let mut module_info: MODULEINFO = unsafe { std::mem::zeroed() };

            let result = unsafe {
                GetModuleInformation(
                    process_handle,
                    module_handles[i],
                    &mut module_info,
                    std::mem::size_of::<MODULEINFO>() as u32,
                )
            };

            if result == 0 {
                continue;
            }
            
            modules.insert(NormalizedModule::new(
                &module_name,
                module_info.lpBaseOfDll as u64,
                module_info.SizeOfImage as u64,
            ));
        }

        return modules;
    }

    fn get_stack_addresses(
        &self,
        process_id: &Pid,
    ) -> HashSet<NormalizedRegion> {
        unimplemented!()
    }

    fn get_heap_addresses(
        &self,
        process_id: &Pid,
    ) -> HashSet<NormalizedRegion> {
        unimplemented!()
    }

    fn address_to_module(
        &self,
        process_id: &Pid,
        address: u64,
        module_name: &mut String,
    ) -> u64 {
        let modules = self.get_modules(process_id);
        
        /*
        for module in modules {
            if module.contains_address(address) {
                *module_name = module.get_name();
                return address - module.get_base_address();
            }
        }

        *module_name = String::new(); */
        return address;
    }

    fn resolve_module(
        &self,
        process_id: &Pid,
        identifier: &str,
    ) -> u64 {
        let modules = self.get_modules(process_id);

        for module in modules {
            if module.get_name().eq_ignore_ascii_case(identifier) {
                return module.get_base_address();
            }
        }

        return 0;
    }
}
