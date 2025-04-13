use crate::structures::{data_types::data_type_ref::DataTypeRef, data_values::data_value::DataValue};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents a base scan result containing only the address and data type.
/// This will later need to be processed to determine modules, offsets, current values, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanResultBase {
    address: u64,
    data_type: DataTypeRef,
}

impl ScanResultBase {
    pub fn new(
        address: u64,
        data_type: DataTypeRef,
    ) -> Self {
        Self { address, data_type }
    }

    pub fn get_address(&self) -> u64 {
        self.address
    }

    pub fn get_data_type(&self) -> &DataTypeRef {
        &self.data_type
    }
}

impl FromStr for ScanResultBase {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = string.split(',').collect();
        if parts.len() < 2 {
            return Err("Input string must contain at least an address and data type".to_string());
        }

        let address = match parts[0].trim().parse::<u64>() {
            Ok(address) => address,
            Err(err) => {
                return Err(format!("Failed to parse address: {}", err));
            }
        };

        let data_type = parts[1].trim().parse::<DataTypeRef>()?;

        let current_value = if parts.len() > 2 && !parts[2].trim().is_empty() {
            Some(parts[2].trim().parse::<DataValue>()?)
        } else {
            None
        };

        let previous_value = if parts.len() > 3 && !parts[3].trim().is_empty() {
            Some(parts[3].trim().parse::<DataValue>()?)
        } else {
            None
        };

        Ok(ScanResultBase { address, data_type })
    }
}
