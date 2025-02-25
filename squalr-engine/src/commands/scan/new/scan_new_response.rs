use crate::commands::engine_response::EngineResponse;
use crate::commands::engine_response::TypedEngineResponse;
use crate::commands::scan::scan_response::ScanResponse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanNewResponse {}

impl TypedEngineResponse for ScanNewResponse {
    fn to_engine_response(&self) -> EngineResponse {
        EngineResponse::Scan(ScanResponse::New {
            scan_new_response: self.clone(),
        })
    }

    fn from_engine_response(response: EngineResponse) -> Result<Self, EngineResponse> {
        if let EngineResponse::Scan(ScanResponse::New { scan_new_response }) = response {
            Ok(scan_new_response)
        } else {
            Err(response)
        }
    }
}
