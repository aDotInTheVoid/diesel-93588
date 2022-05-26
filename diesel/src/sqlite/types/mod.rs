mod date_and_time;
mod numeric;
use super::connection::SqliteValue;
use super::Sqlite;
use crate::deserialize::{self, FromSql};
use crate::query_builder::QueryId;
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types;
use crate::sql_types::SqlType;
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::VarChar, Sqlite> for *const str {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Binary, Sqlite> for *const [u8] {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::SmallInt, Sqlite> for i16 {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Integer, Sqlite> for i32 {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Bool, Sqlite> for bool {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::BigInt, Sqlite> for i64 {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Float, Sqlite> for f32 {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Double, Sqlite> for f64 {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Bool, Sqlite> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Text, Sqlite> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Binary, Sqlite> for [u8] {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::SmallInt, Sqlite> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Integer, Sqlite> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::BigInt, Sqlite> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Float, Sqlite> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Double, Sqlite> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
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
#[cfg_attr(feature = "chrono", doc = " [`chrono::DateTime`]: chrono::DateTime")]
#[cfg_attr(
    not(feature = "chrono"),
    doc = " [`chrono::DateTime`]: https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html"
)]
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(sqlite_type(name = "Text"))]
#[cfg(feature = "sqlite")]
pub struct Timestamptz;
