use crate::structures::data_types::data_type_error::DataTypeError;
use crate::structures::data_types::data_type_meta_data::DataTypeMetaData;
use crate::structures::data_types::data_type_ref::DataTypeRef;
use crate::structures::data_values::anonymous_value::{AnonymousValue, AnonymousValueContainer};
use crate::structures::memory::endian::Endian;
use crate::structures::{data_types::data_type::DataType, data_values::data_value::DataValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataTypeByteArray {}

impl DataTypeByteArray {
    pub const DATA_TYPE_ID: &str = "byte_array";

    pub fn get_data_type_id() -> &'static str {
        Self::DATA_TYPE_ID
    }
}

impl DataType for DataTypeByteArray {
    fn get_data_type_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_icon_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_default_size_in_bytes(&self) -> u64 {
        1
    }

    fn validate_value(
        &self,
        anonymous_value: &AnonymousValue,
    ) -> bool {
        let data_type_ref = DataTypeRef::new(self.get_data_type_id(), DataTypeMetaData::SizedContainer(0));

        match self.deanonymize_value(anonymous_value, data_type_ref) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn deanonymize_value(
        &self,
        anonymous_value: &AnonymousValue,
        data_type_ref: DataTypeRef,
    ) -> Result<DataValue, DataTypeError> {
        match anonymous_value.get_value() {
            AnonymousValueContainer::StringValue(value_string, is_value_hex) => {
                if *is_value_hex {
                    // Clean input: remove whitespace, commas, and any 0x prefixes.
                    let mut cleaned = value_string
                        .replace(|next_char: char| next_char.is_whitespace() || next_char == ',', "")
                        .replace("0x", "")
                        .replace("0X", "");

                    // Zero-pad odd numbered length to force the hex to be groups of two digits per byte.
                    if cleaned.len() % 2 != 0 {
                        cleaned = format!("0{}", cleaned);
                    }

                    let value_bytes = cleaned
                        .as_bytes()
                        .chunks(2)
                        .map(|chunk| {
                            let hex_str = std::str::from_utf8(chunk).unwrap_or_default();
                            u8::from_str_radix(hex_str, 16).map_err(|err| DataTypeError::ValueParseError {
                                value: hex_str.to_string(),
                                is_value_hex: true,
                                source: err,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    // Group into bytes (2 hex digits each).
                    Ok(DataValue::new(data_type_ref, value_bytes))
                } else {
                    // For decimal, allow space or comma separation.
                    let value_bytes = value_string
                        .split(|next_char: char| next_char.is_whitespace() || next_char == ',')
                        .filter(|next_value| !next_value.is_empty())
                        .map(|next_value| {
                            u8::from_str_radix(next_value, 10).map_err(|err| DataTypeError::ValueParseError {
                                value: next_value.to_string(),
                                is_value_hex: false,
                                source: err,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    Ok(DataValue::new(data_type_ref, value_bytes))
                }
            }
            AnonymousValueContainer::ByteArray(value_bytes) => Ok(DataValue::new(data_type_ref, value_bytes.clone())),
        }
    }

    fn create_display_value(
        &self,
        value_bytes: &[u8],
        _data_type_meta_data: &DataTypeMetaData,
    ) -> Result<String, DataTypeError> {
        if !value_bytes.is_empty() {
            Ok(value_bytes
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(" "))
        } else {
            Err(DataTypeError::NoBytes)
        }
    }

    fn is_discrete(&self) -> bool {
        true
    }

    fn get_endian(&self) -> Endian {
        Endian::Little
    }

    fn get_default_value(
        &self,
        data_type_ref: DataTypeRef,
    ) -> DataValue {
        let array_size = match data_type_ref.get_meta_data() {
            DataTypeMetaData::SizedContainer(size) => *size as usize,
            _ => {
                log::error!("Invalid metadata provided to byte array data type.");
                0usize
            }
        };

        DataValue::new(data_type_ref, vec![0u8; array_size])
    }

    fn get_default_meta_data(&self) -> DataTypeMetaData {
        DataTypeMetaData::SizedContainer(1)
    }
}
