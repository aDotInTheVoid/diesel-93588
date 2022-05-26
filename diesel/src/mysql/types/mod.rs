pub(super) mod date_and_time;
#[cfg(feature = "serde_json")]
mod json;
mod numeric;
mod primitives;
use crate::deserialize::{self, FromSql};
use crate::mysql::{Mysql, MysqlType, MysqlValue};
use crate::query_builder::QueryId;
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::*;
use crate::sql_types::{self, ops::*};
use byteorder::{NativeEndian, WriteBytesExt};
#[cfg(feature = "mysql_backend")]
impl ToSql<TinyInt, Mysql> for i8 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<TinyInt, Mysql> for i8 {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy, Default, SqlType, QueryId)]
#[cfg(feature = "mysql_backend")]
pub struct Unsigned<ST: 'static>(ST);
impl<T> Add for Unsigned<T>
where
    T: Add,
{
    type Rhs = Unsigned<T::Rhs>;
    type Output = Unsigned<T::Output>;
}
impl<T> Sub for Unsigned<T>
where
    T: Sub,
{
    type Rhs = Unsigned<T::Rhs>;
    type Output = Unsigned<T::Output>;
}
impl<T> Mul for Unsigned<T>
where
    T: Mul,
{
    type Rhs = Unsigned<T::Rhs>;
    type Output = Unsigned<T::Output>;
}
impl<T> Div for Unsigned<T>
where
    T: Div,
{
    type Rhs = Unsigned<T::Rhs>;
    type Output = Unsigned<T::Output>;
}
#[cfg(feature = "mysql_backend")]
impl ToSql<Unsigned<TinyInt>, Mysql> for u8 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Unsigned<TinyInt>, Mysql> for u8 {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<Unsigned<SmallInt>, Mysql> for u16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Unsigned<SmallInt>, Mysql> for u16 {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<Unsigned<Integer>, Mysql> for u32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Unsigned<Integer>, Mysql> for u32 {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<Unsigned<BigInt>, Mysql> for u64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Unsigned<BigInt>, Mysql> for u64 {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<Bool, Mysql> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl FromSql<Bool, Mysql> for bool {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<sql_types::SmallInt, Mysql> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<sql_types::Integer, Mysql> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<sql_types::BigInt, Mysql> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<sql_types::Double, Mysql> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl ToSql<sql_types::Float, Mysql> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl HasSqlType<Unsigned<TinyInt>> for Mysql {
    fn metadata(_lookup: &mut ()) -> MysqlType {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl HasSqlType<Unsigned<SmallInt>> for Mysql {
    fn metadata(_lookup: &mut ()) -> MysqlType {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl HasSqlType<Unsigned<Integer>> for Mysql {
    fn metadata(_lookup: &mut ()) -> MysqlType {
        loop {}
    }
}
#[cfg(feature = "mysql_backend")]
impl HasSqlType<Unsigned<BigInt>> for Mysql {
    fn metadata(_lookup: &mut ()) -> MysqlType {
        loop {}
    }
}
#[cfg_attr(
    feature = "chrono",
    doc = " [`chrono::NaiveDateTime`]: chrono::naive::NaiveDateTime"
)]
#[cfg_attr(
    not(feature = "chrono"),
    doc = " [`chrono::NaiveDateTime`]: https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveDateTime.html"
)]
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(mysql_type(name = "DateTime"))]
#[cfg(feature = "mysql_backend")]
pub struct Datetime;
