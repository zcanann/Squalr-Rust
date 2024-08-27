use crate::command_handlers::memory::memory_command::MemoryCommand;
use squalr_engine::session_manager::SessionManager;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_memory::memory_reader::memory_reader_trait::IMemoryReader;
use squalr_engine_memory::memory_reader::MemoryReader;

pub fn handle_memory_read(cmd: &mut MemoryCommand) {
    if let MemoryCommand::Read { address, ref mut value } = cmd {
        let session_manager_lock = SessionManager::get_instance();
        let session_manager = session_manager_lock.read().unwrap();
        if let Some(process_info) = session_manager.get_opened_process() {
            Logger::get_instance().log(LogLevel::Info, &format!("Reading value from address {}", address), None);

            match MemoryReader::get_instance().read(process_info.handle, *address, value) {
                true => {
                    Logger::get_instance().log(LogLevel::Info, &format!("Read value {:?} from address {}", value, address), None);
                }
                false => {
                    Logger::get_instance().log(LogLevel::Error, &format!("Failed to read memory"), None);
                }
            }
        } else {
            Logger::get_instance().log(LogLevel::Error, "No opened process available.", None);
        }
    }
}
