use crate::values::data_type::DataType;
use crate::values::data_value::DataValue;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents a value as a string that can potentially be converted to an explicit type later.
/// This is particularly useful when scannining for a value that may later be interpreted
/// as many data types, and supporting values passed via command line.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AnonymousValue {
    value_str: String,
}

impl AnonymousValue {
    pub fn new(value: &str) -> Self {
        AnonymousValue { value_str: value.to_string() }
    }

    pub fn deanonymize_type(
        &self,
        target_type: &DataType,
    ) -> Result<DataValue, String> {
        let value_and_type_str = format!("{}={}", self.value_str, target_type);

        return value_and_type_str.parse::<DataValue>();
    }
}

impl ToString for AnonymousValue {
    fn to_string(&self) -> String {
        self.value_str.clone()
    }
}

impl FromStr for AnonymousValue {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        return Ok(AnonymousValue::new(string));
    }
}
