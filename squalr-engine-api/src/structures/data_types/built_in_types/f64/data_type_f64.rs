use crate::structures::data_types::data_type_meta_data::DataTypeMetaData;
use crate::structures::data_values::anonymous_value::AnonymousValue;
use crate::structures::memory::endian::Endian;
use crate::structures::{data_types::data_type::DataType, data_values::data_value::DataValue};
use serde::{Deserialize, Serialize};

type PrimitiveType = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataTypeF64 {}

impl DataTypeF64 {
    pub fn get_data_type_id() -> &'static str {
        &"f64"
    }

    fn to_vec(value: PrimitiveType) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }
}

impl DataType for DataTypeF64 {
    fn get_data_type_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_icon_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_default_size_in_bytes(&self) -> u64 {
        size_of::<PrimitiveType>() as u64
    }

    fn deanonymize_value(
        &self,
        anonymous_value: &AnonymousValue,
    ) -> Vec<u8> {
        let value_string = anonymous_value.to_string();

        match value_string.parse::<PrimitiveType>() {
            Ok(value) => Self::to_vec(value),
            Err(_) => vec![],
        }
    }

    fn create_display_value(
        &self,
        value_bytes: &[u8],
    ) -> Option<String> {
        if value_bytes.len() == self.get_default_size_in_bytes() as usize {
            Some(
                PrimitiveType::from_le_bytes([
                    value_bytes[0],
                    value_bytes[1],
                    value_bytes[2],
                    value_bytes[3],
                    value_bytes[4],
                    value_bytes[5],
                    value_bytes[6],
                    value_bytes[7],
                ])
                .to_string(),
            )
        } else {
            None
        }
    }

    fn get_endian(&self) -> Endian {
        Endian::Little
    }

    fn get_default_value(&self) -> DataValue {
        DataValue::new(self.get_ref(), Self::to_vec(0.0))
    }

    fn get_default_meta_data(&self) -> DataTypeMetaData {
        DataTypeMetaData::None
    }
}
