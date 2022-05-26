use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Oid, Pg> for u32 {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Oid, Pg> for u32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::SmallInt, Pg> for i16 {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Integer, Pg> for i32 {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::BigInt, Pg> for i64 {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::SmallInt, Pg> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Integer, Pg> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::BigInt, Pg> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_builder::bind_collector::ByteWrapper;
    #[test]
    fn i16_to_sql() {
        loop {}
    }
    #[test]
    fn i32_to_sql() {
        loop {}
    }
    #[test]
    fn i64_to_sql() {
        loop {}
    }
}
