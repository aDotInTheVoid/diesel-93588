//! Support for JSON and `jsonb` values under PostgreSQL.
extern crate serde_json;
use std::io::prelude::*;
use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types;
#[cfg(all(feature = "postgres_backend", feature = "serde_json"))]
impl FromSql<sql_types::Json, Pg> for serde_json::Value {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "postgres_backend", feature = "serde_json"))]
impl ToSql<sql_types::Json, Pg> for serde_json::Value {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "postgres_backend", feature = "serde_json"))]
impl FromSql<sql_types::Jsonb, Pg> for serde_json::Value {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "postgres_backend", feature = "serde_json"))]
impl ToSql<sql_types::Jsonb, Pg> for serde_json::Value {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[cfg(tests)]
mod tests {
    use crate::query_builder::bind_types::ByteWrapper;
    #[test]
    fn json_to_sql() {
        loop {}
    }
    #[test]
    fn some_json_from_sql() {
        loop {}
    }
    #[test]
    fn bad_json_from_sql() {
        loop {}
    }
    #[test]
    fn no_json_from_sql() {
        loop {}
    }
    #[test]
    fn jsonb_to_sql() {
        loop {}
    }
    #[test]
    fn some_jsonb_from_sql() {
        loop {}
    }
    #[test]
    fn bad_jsonb_from_sql() {
        loop {}
    }
    #[test]
    fn bad_jsonb_version_from_sql() {
        loop {}
    }
    #[test]
    fn no_jsonb_from_sql() {
        loop {}
    }
}
