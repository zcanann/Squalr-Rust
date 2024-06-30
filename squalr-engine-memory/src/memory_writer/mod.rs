pub mod memory_writer_trait;
use std::sync::Once;

#[cfg(any(target_os = "linux"))]
mod linux;

#[cfg(any(target_os = "macos"))]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use crate::memory_writer::linux::linux_memory_writer::LinuxMemoryWriter as MemoryWriterImpl;

#[cfg(target_os = "macos")]
pub use crate::memory_writer::macos::macos_memory_writer::MacOsMemoryWriter as MemoryWriterImpl;

#[cfg(target_os = "windows")]
pub use crate::memory_writer::windows::windows_memory_writer::WindowsMemoryWriter as MemoryWriterImpl;

pub struct MemoryWriter;

impl MemoryWriter {
    pub fn instance() -> &'static MemoryWriterImpl {
        static mut SINGLETON: Option<MemoryWriterImpl> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = MemoryWriterImpl::new();
                SINGLETON = Some(instance);
            });

            SINGLETON.as_ref().unwrap()
        }
    }
}
