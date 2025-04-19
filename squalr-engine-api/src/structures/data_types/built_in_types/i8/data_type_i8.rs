use crate::structures::data_types::built_in_types::primitive_type::PrimitiveDataType;
use crate::structures::data_types::data_type_error::DataTypeError;
use crate::structures::data_types::data_type_meta_data::DataTypeMetaData;
use crate::structures::data_types::data_type_ref::DataTypeRef;
use crate::structures::data_values::anonymous_value::AnonymousValue;
use crate::structures::memory::endian::Endian;
use crate::structures::{data_types::data_type::DataType, data_values::data_value::DataValue};
use serde::{Deserialize, Serialize};

type PrimitiveType = i8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataTypeI8 {}

impl DataTypeI8 {
    pub const DATA_TYPE_ID: &str = "i8";

    pub fn get_data_type_id() -> &'static str {
        Self::DATA_TYPE_ID
    }

    fn to_vec(value: PrimitiveType) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }
}

impl DataType for DataTypeI8 {
    fn get_data_type_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_icon_id(&self) -> &str {
        Self::get_data_type_id()
    }

    fn get_default_size_in_bytes(&self) -> u64 {
        size_of::<PrimitiveType>() as u64
    }

    fn validate_value(
        &self,
        anonymous_value: &AnonymousValue,
    ) -> bool {
        let data_type_ref = DataTypeRef::new_from_anonymous_value(self.get_data_type_id(), anonymous_value);

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
        PrimitiveDataType::deanonymize_primitive::<PrimitiveType>(anonymous_value, data_type_ref, false)
    }

    fn create_display_value(
        &self,
        value_bytes: &[u8],
        _data_type_meta_data: &DataTypeMetaData,
    ) -> Result<String, DataTypeError> {
        let expected = self.get_default_size_in_bytes() as usize;
        let actual = value_bytes.len();

        if actual == expected {
            Ok(PrimitiveType::from_le_bytes([value_bytes[0]]).to_string())
        } else {
            Err(DataTypeError::InvalidByteCount { expected, actual })
        }
    }

    fn get_endian(&self) -> Endian {
        Endian::Little
    }

    fn is_discrete(&self) -> bool {
        true
    }

    fn get_default_value(
        &self,
        data_type_ref: DataTypeRef,
    ) -> DataValue {
        DataValue::new(data_type_ref, Self::to_vec(0))
    }

    fn get_default_meta_data(&self) -> DataTypeMetaData {
        DataTypeMetaData::None
    }

    fn get_meta_data_for_anonymous_value(
        &self,
        _anonymous_value: &AnonymousValue,
    ) -> DataTypeMetaData {
        DataTypeMetaData::None
    }

    fn get_meta_data_from_string(
        &self,
        _string: &str,
    ) -> Result<DataTypeMetaData, String> {
        Ok(DataTypeMetaData::None)
    }
}
