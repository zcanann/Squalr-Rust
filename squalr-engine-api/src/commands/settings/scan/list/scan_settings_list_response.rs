use crate::commands::engine_command_response::EngineCommandResponse;
use crate::commands::engine_command_response::TypedEngineCommandResponse;
use crate::commands::settings::scan::scan_settings_response::ScanSettingsResponse;
use crate::commands::settings::settings_response::SettingsResponse;
use crate::structures::settings::scan_settings::ScanSettings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSettingsListResponse {
    pub scan_settings: Result<ScanSettings, String>,
}

impl TypedEngineCommandResponse for ScanSettingsListResponse {
    fn to_engine_response(&self) -> EngineCommandResponse {
        EngineCommandResponse::Settings(SettingsResponse::Scan {
            scan_settings_response: ScanSettingsResponse::List {
                scan_settings_list_response: self.clone(),
            },
        })
    }

    fn from_engine_response(response: EngineCommandResponse) -> Result<Self, EngineCommandResponse> {
        if let EngineCommandResponse::Settings(SettingsResponse::Scan {
            scan_settings_response: ScanSettingsResponse::List { scan_settings_list_response },
        }) = response
        {
            Ok(scan_settings_list_response)
        } else {
            Err(response)
        }
    }
}
