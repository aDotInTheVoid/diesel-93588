use std::io::prelude::*;
use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types;
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Bool, Pg> for bool {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<sql_types::Bool, Pg> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
/// The returned pointer is *only* valid for the lifetime to the argument of
/// `from_sql`. This impl is intended for uses where you want to write a new
/// impl in terms of `String`, but don't want to allocate. We have to return a
/// raw pointer instead of a reference with a lifetime due to the structure of
/// `FromSql`
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Text, Pg> for *const str {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
/// The returned pointer is *only* valid for the lifetime to the argument of
/// `from_sql`. This impl is intended for uses where you want to write a new
/// impl in terms of `Vec<u8>`, but don't want to allocate. We have to return a
/// raw pointer instead of a reference with a lifetime due to the structure of
/// `FromSql`
#[cfg(feature = "postgres_backend")]
impl FromSql<sql_types::Binary, Pg> for *const [u8] {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[test]
fn bool_to_sql() {
    loop {}
}
#[test]
fn no_bool_from_sql() {
    loop {}
}
