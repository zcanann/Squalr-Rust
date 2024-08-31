pub mod settings_command;
pub mod settings_command_list;
pub mod settings_command_set;
pub use settings_command::SettingsCommand;
pub use settings_command_list::handle_settings_list;
pub use settings_command_set::handle_settings_set;

pub fn handle_settings_command(cmd: &mut SettingsCommand) {
    match cmd {
        SettingsCommand::Set { .. } => handle_settings_set(cmd),
        SettingsCommand::List { .. } => handle_settings_list(cmd),
    }
}
