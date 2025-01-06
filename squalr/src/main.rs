// Disable terminal from spawning. All relevant output is routed to the output view anyways.
#![windows_subsystem = "windows"]

pub mod cli_log_listener;
pub mod docking;
pub mod models;
pub mod view_models;

use crate::view_models::main_window::main_window_view_model::MainWindowViewModel;
use squalr_engine::session_manager::SessionManager;
use squalr_engine_architecture::vectors::vectors;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;

// Makes the code generated by compiled .slint files available to our Rust code.
slint::include_modules!();

pub fn main() {
    // Override the default backend (femtovg => software). I wont want to rely on devs having QT installed,
    // and femtovg is shit at rendering font. Skia doesn't install properly on nightly, so software renderer it is.
    std::env::set_var("SLINT_BACKEND", "winit-software");

    // Create and show the main window, which in turn will instantiate all other windows and panels.
    let main_window_view = MainWindowViewModel::new();

    Logger::get_instance().log(LogLevel::Info, "Squalr started", None);
    vectors::log_vector_architecture();

    if let Ok(session_manager) = SessionManager::get_instance().read() {
        session_manager.initialize();
    } else {
        Logger::get_instance().log(LogLevel::Error, "Fatal error initializing session manager.", None);
    }

    main_window_view.initialize();

    // Run the slint window event loop until slint::quit_event_loop() is called.
    match slint::run_event_loop() {
        Ok(_) => {}
        Err(e) => {
            Logger::get_instance().log(LogLevel::Error, "Fatal error starting Squalr.", Some(e.to_string().as_str()));
        }
    }
}
