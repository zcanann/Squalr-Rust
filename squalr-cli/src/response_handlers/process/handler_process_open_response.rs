use squalr_engine::commands::process::process_response::ProcessResponse;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;

pub fn handle_process_open_response(process_response: ProcessResponse) {
    if let ProcessResponse::Open { process_open_response } = process_response {
        let process_info = process_open_response.opened_process_info;

        Logger::get_instance().log(
            LogLevel::Info,
            &format!("Opened process_id: {}, Name: {}", process_info.process_id, process_info.name),
            None,
        );
    }
}
