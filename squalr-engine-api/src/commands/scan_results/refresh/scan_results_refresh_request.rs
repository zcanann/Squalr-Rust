use crate::commands::engine_command_request::EngineCommandRequest;
use crate::commands::scan_results::refresh::scan_results_refresh_response::ScanResultsRefreshResponse;
use crate::commands::scan_results::scan_results_command::ScanResultsCommand;
use crate::commands::scan_results::scan_results_response::ScanResultsResponse;
use crate::{commands::engine_command::EngineCommand, structures::scan_results::scan_result_valued::ScanResultValued};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug, Serialize, Deserialize)]
pub struct ScanResultsRefreshRequest {
    #[structopt(short = "r", long)]
    pub scan_results: Vec<ScanResultValued>,
}

impl EngineCommandRequest for ScanResultsRefreshRequest {
    type ResponseType = ScanResultsRefreshResponse;

    fn to_engine_command(&self) -> EngineCommand {
        EngineCommand::Results(ScanResultsCommand::Refresh {
            results_refresh_request: self.clone(),
        })
    }
}

impl From<ScanResultsRefreshResponse> for ScanResultsResponse {
    fn from(scan_results_refresh_response: ScanResultsRefreshResponse) -> Self {
        ScanResultsResponse::Refresh { scan_results_refresh_response }
    }
}
