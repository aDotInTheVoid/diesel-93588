use std::convert::TryInto;
use std::io::prelude::*;
use crate::deserialize::{self, FromSql};
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::MacAddr;
#[allow(dead_code)]
mod foreign_derives {
    use super::*;
    use crate::deserialize::FromSqlRow;
    use crate::expression::AsExpression;
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(foreign_derive)]
    #[diesel(sql_type = MacAddr)]
    struct ByteArrayProxy([u8; 6]);
}
#[cfg(feature = "postgres_backend")]
impl FromSql<MacAddr, Pg> for [u8; 6] {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<MacAddr, Pg> for [u8; 6] {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
#[test]
fn macaddr_roundtrip() {
    loop {}
}
