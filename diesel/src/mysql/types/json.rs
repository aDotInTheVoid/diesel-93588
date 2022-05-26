use crate::deserialize::{self, FromSql};
use crate::mysql::{Mysql, MysqlValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types;
#[cfg(all(feature = "mysql_backend", feature = "serde_json"))]
impl FromSql<sql_types::Json, Mysql> for serde_json::Value {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "mysql_backend", feature = "serde_json"))]
impl ToSql<sql_types::Json, Mysql> for serde_json::Value {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
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
