use super::types::date_and_time::MysqlTime;
use super::MysqlType;
use crate::deserialize;
use std::error::Error;
#[derive(Clone, Debug)]
pub struct MysqlValue<'a> {
    raw: &'a [u8],
    tpe: MysqlType,
}
impl<'a> MysqlValue<'a> {
                #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub fn new(raw: &'a [u8], tpe: MysqlType) -> Self {
        loop {}
    }
    pub(in crate::mysql) fn new_internal(raw: &'a [u8], tpe: MysqlType) -> Self {
        loop {}
    }
        pub fn as_bytes(&self) -> &[u8] {
        loop {}
    }
        pub fn value_type(&self) -> MysqlType {
        loop {}
    }
            #[allow(dead_code, clippy::cast_ptr_alignment)]
    pub(crate) fn time_value(&self) -> deserialize::Result<MysqlTime> {
        loop {}
    }
            pub(crate) fn numeric_value(
        &self,
    ) -> deserialize::Result<NumericRepresentation<'_>> {
        loop {}
    }
    fn invalid_type_code(&self, expected: &str) -> Box<dyn Error + Send + Sync> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum NumericRepresentation<'a> {
        Tiny(i8),
        Small(i16),
        Medium(i32),
        Big(i64),
        Float(f32),
        Double(f64),
        Decimal(&'a [u8]),
}
