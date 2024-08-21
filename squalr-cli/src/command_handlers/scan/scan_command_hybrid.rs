use crate::command_handlers::scan::ScanCommand;
use squalr_engine::session_manager::SessionManager;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_memory::memory_alignment::MemoryAlignment;
use squalr_engine_scanning::scanners::constraints::scan_constraint::ScanConstraint;
use squalr_engine_scanning::scanners::hybrid_scanner::HybridScanner;
use std::thread;

pub fn handle_hybrid_scan_command(
    cmd: &mut ScanCommand,
) {
    if let ScanCommand::Hybrid { value_and_type, constraint_type} = cmd {
        let session_manager_lock = SessionManager::get_instance();
        let process_info = {
            let session_manager = session_manager_lock.read().unwrap();
            session_manager.get_opened_process().cloned()
        };

        if let Some(process_info) = process_info {
            let session_manager = session_manager_lock.write().unwrap();
            let snapshot = session_manager.get_snapshot();

            let data_types = vec![value_and_type.data_type.to_owned()];

            let constraint = ScanConstraint::new_with_value(
                constraint_type.to_owned(),
                None, // TODO
            );
            
            let task = HybridScanner::scan(
                process_info.clone(),
                snapshot,
                &constraint,
                None,
                true
            );

            // Spawn a thread to listen to progress updates
            let progress_receiver = task.add_listener();
            thread::spawn(move || {
                while let Ok(progress) = progress_receiver.recv() {
                    Logger::get_instance().log(LogLevel::Info, &format!("Progress: {:.2}%", progress), None);
                }
            });

            // Wait for completion synchronously
            task.wait_for_completion();
        } else {
            Logger::get_instance().log(LogLevel::Info, "No opened process", None);
        }
    }
}
