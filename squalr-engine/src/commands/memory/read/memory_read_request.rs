use std::sync::Arc;

use crate::commands::engine_command::EngineCommand;
use crate::commands::engine_request::EngineRequest;
use crate::commands::memory::memory_command::MemoryCommand;
use crate::commands::memory::memory_response::MemoryResponse;
use crate::commands::memory::read::memory_read_response::MemoryReadResponse;
use crate::engine_execution_context::EngineExecutionContext;
use serde::Deserialize;
use serde::Serialize;
use squalr_engine_common::conversions::Conversions;
use squalr_engine_common::dynamic_struct::dynamic_struct::DynamicStruct;
use squalr_engine_common::logging::log_level::LogLevel;
use squalr_engine_common::logging::logger::Logger;
use squalr_engine_memory::memory_reader::MemoryReader;
use squalr_engine_memory::memory_reader::memory_reader_trait::IMemoryReader;
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug, Serialize, Deserialize)]
pub struct MemoryReadRequest {
    #[structopt(short = "a", long, parse(try_from_str = Conversions::parse_hex_or_int))]
    pub address: u64,
    #[structopt(short = "v", long)]
    pub value: DynamicStruct,
}

impl EngineRequest for MemoryReadRequest {
    type ResponseType = MemoryReadResponse;

    fn execute(
        &self,
        execution_context: &Arc<EngineExecutionContext>,
    ) -> Self::ResponseType {
        if let Some(process_info) = execution_context.get_opened_process() {
            Logger::log(LogLevel::Info, &format!("Reading value from address {}", self.address), None);

            let mut out_value = self.value.clone();
            let success = MemoryReader::get_instance().read_struct(&process_info, self.address, &mut out_value);

            MemoryReadResponse {
                value: out_value,
                address: self.address,
                success: success,
            }
        } else {
            Logger::log(LogLevel::Error, "No opened process available.", None);

            MemoryReadResponse {
                value: DynamicStruct::new(),
                address: self.address,
                success: false,
            }
        }
    }

    fn to_engine_command(&self) -> EngineCommand {
        EngineCommand::Memory(MemoryCommand::Read {
            memory_read_request: self.clone(),
        })
    }
}

impl From<MemoryReadResponse> for MemoryResponse {
    fn from(memory_read_response: MemoryReadResponse) -> Self {
        MemoryResponse::Read { memory_read_response }
    }
}
