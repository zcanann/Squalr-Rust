use crate::commands::command_handlers::inter_process_command_handler::InterProcessCommandHandler;
use crate::commands::command_handlers::memory;
use crate::commands::command_handlers::process;
use crate::commands::command_handlers::project;
use crate::commands::command_handlers::results;
use crate::commands::command_handlers::scan;
use crate::commands::command_handlers::settings;
use crate::commands::engine_command::EngineCommand;

pub enum CommandHandlerType {
    Standard(),
    InterProcess(InterProcessCommandHandler),
}

pub struct CommandHandler {}

impl CommandHandler {
    pub fn handle_command(command: &mut EngineCommand) {
        match command {
            EngineCommand::Memory(cmd) => memory::handle_memory_command(cmd),
            EngineCommand::Process(cmd) => process::handle_process_command(cmd),
            EngineCommand::Project(cmd) => project::handle_project_command(cmd),
            EngineCommand::Results(cmd) => results::handle_results_command(cmd),
            EngineCommand::Scan(cmd) => scan::handle_scan_command(cmd),
            EngineCommand::Settings(cmd) => settings::handle_settings_command(cmd),
        }
    }
}
