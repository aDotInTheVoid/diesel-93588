use crate::deserialize::{self, FromSql};
use crate::mysql::{Mysql, MysqlValue, NumericRepresentation};
use crate::result::Error::DeserializationError;
use crate::sql_types::{BigInt, Binary, Double, Float, Integer, SmallInt, Text};
use std::convert::TryInto;
use std::error::Error;
use std::str::{self, FromStr};
fn decimal_to_integer<T>(bytes: &[u8]) -> deserialize::Result<T>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    loop {}
}
fn f32_to_i64(f: f32) -> deserialize::Result<i64> {
    loop {}
}
fn f64_to_i64(f: f64) -> deserialize::Result<i64> {
    loop {}
}
#[cfg(feature = "mysql_backend")]
impl FromSql<SmallInt, Mysql> for i16 {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Integer, Mysql> for i32 {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<BigInt, Mysql> for i64 {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Float, Mysql> for f32 {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Double, Mysql> for f64 {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Text, Mysql> for String {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Binary, Mysql> for Vec<u8> {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
