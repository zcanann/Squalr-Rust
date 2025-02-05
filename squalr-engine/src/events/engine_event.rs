use serde::{Deserialize, Serialize};
use squalr_engine_processes::process_info::OpenedProcessInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngineEvent {
    ProcessOpened(OpenedProcessInfo),
}
