use super::types::date_and_time::MysqlTime;
use super::MysqlType;
use crate::deserialize;
use std::error::Error;
/// Raw mysql value as received from the database
#[derive(Clone, Debug)]
pub struct MysqlValue<'a> {
    raw: &'a [u8],
    tpe: MysqlType,
}
impl<'a> MysqlValue<'a> {
    /// Create a new instance of [MysqlValue] based on a byte buffer
    /// and information about the type of the value represented by the
    /// given buffer
    #[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
    pub fn new(raw: &'a [u8], tpe: MysqlType) -> Self {
        loop {}
    }
    pub(in crate::mysql) fn new_internal(raw: &'a [u8], tpe: MysqlType) -> Self {
        loop {}
    }
    /// Get the underlying raw byte representation
    pub fn as_bytes(&self) -> &[u8] {
        loop {}
    }
    /// Get the mysql type of the current value
    pub fn value_type(&self) -> MysqlType {
        loop {}
    }
    /// Checks that the type code is valid, and interprets the data as a
    /// `MysqlTime` pointer
    #[allow(dead_code, clippy::cast_ptr_alignment)]
    pub(crate) fn time_value(&self) -> deserialize::Result<MysqlTime> {
        loop {}
    }
    /// Returns the numeric representation of this value, based on the type code.
    /// Returns an error if the type code is not numeric.
    pub(crate) fn numeric_value(
        &self,
    ) -> deserialize::Result<NumericRepresentation<'_>> {
        loop {}
    }
    fn invalid_type_code(&self, expected: &str) -> Box<dyn Error + Send + Sync> {
        loop {}
    }
}
/// Represents all possible forms MySQL transmits integers
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum NumericRepresentation<'a> {
    /// Correponds to `MYSQL_TYPE_TINY`
    Tiny(i8),
    /// Correponds to `MYSQL_TYPE_SHORT`
    Small(i16),
    /// Correponds to `MYSQL_TYPE_INT24` and `MYSQL_TYPE_LONG`
    Medium(i32),
    /// Correponds to `MYSQL_TYPE_LONGLONG`
    Big(i64),
    /// Correponds to `MYSQL_TYPE_FLOAT`
    Float(f32),
    /// Correponds to `MYSQL_TYPE_DOUBLE`
    Double(f64),
    /// Correponds to `MYSQL_TYPE_DECIMAL` and `MYSQL_TYPE_NEWDECIMAL`
    Decimal(&'a [u8]),
}
