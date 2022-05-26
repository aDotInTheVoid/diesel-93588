#![cfg(feature = "bigdecimal")]
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::deserialize::{self, FromSql};
use crate::sql_types::{Double, Numeric};
use crate::sqlite::connection::SqliteValue;
use crate::sqlite::Sqlite;
#[cfg(all(feature = "sqlite", feature = "bigdecimal"))]
impl FromSql<Numeric, Sqlite> for BigDecimal {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> deserialize::Result<Self> {
        loop {}
    }
}
