use std::io::prelude::*;
use crate::deserialize::{self, FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::Uuid;
#[derive(AsExpression, FromSqlRow)]
#[diesel(foreign_derive)]
#[diesel(sql_type = Uuid)]
#[allow(dead_code)]
struct UuidProxy(uuid::Uuid);
#[cfg(all(feature = "postgres_backend", feature = "uuid"))]
impl FromSql<Uuid, Pg> for uuid::Uuid {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "postgres_backend", feature = "uuid"))]
impl ToSql<Uuid, Pg> for uuid::Uuid {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[test]
fn uuid_to_sql() {
    loop {}
}
#[test]
fn some_uuid_from_sql() {
    loop {}
}
#[test]
fn bad_uuid_from_sql() {
    loop {}
}
#[test]
fn no_uuid_from_sql() {
    loop {}
}
