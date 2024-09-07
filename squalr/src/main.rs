pub mod callback;
pub mod cli_log_listener;
pub mod docking;
pub mod view_models;

use crate::view_models::main_window::main_window_view_model::MainWindowViewModel;
use squalr_engine_architecture::vectors::vectors;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;

// Makes the code generated by compiled .slint files available to our Rust code.
slint::include_modules!();

pub fn main() {
    // Override the default backend (femtovg => software). I wont want to rely on devs having QT installed,
    // and femtovg is shit at rendering font. Skia doesn't install properly on nightly, so software renderer it is.
    std::env::set_var("SLINT_BACKEND", "winit-software");

    // Run the slint window event loop until slint::quit_event_loop() is called.
    MainWindowViewModel::new().show();

    vectors::log_vector_architecture();
    Logger::get_instance().log(LogLevel::Info, "Squalr started", None);

    match slint::run_event_loop() {
        Ok(_) => {}
        Err(e) => {
            Logger::get_instance().log(LogLevel::Error, "Fatal error starting Squalr.", Some(e.to_string().as_str()));
        }
    }
}
