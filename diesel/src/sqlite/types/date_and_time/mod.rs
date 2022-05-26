use crate::deserialize::{self, FromSql};
use crate::serialize::{self, Output, ToSql};
use crate::sql_types;
use crate::sqlite::connection::SqliteValue;
use crate::sqlite::Sqlite;
#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Date, Sqlite> for String {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Date, Sqlite> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Date, Sqlite> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Time, Sqlite> for String {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Time, Sqlite> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Time, Sqlite> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::Timestamp, Sqlite> for String {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Timestamp, Sqlite> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::Timestamp, Sqlite> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl FromSql<sql_types::TimestamptzSqlite, Sqlite> for String {
    fn from_sql(value: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::TimestamptzSqlite, Sqlite> for str {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "sqlite")]
impl ToSql<sql_types::TimestamptzSqlite, Sqlite> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        loop {}
    }
}
