#[cfg(feature = "bigdecimal")]
mod bigdecimal {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use std::io::prelude::*;
    use crate::deserialize::{self, FromSql};
    use crate::mysql::{Mysql, MysqlValue, NumericRepresentation};
    use crate::serialize::{self, IsNull, Output, ToSql};
    use crate::sql_types::Numeric;
    #[cfg(all(feature = "mysql_backend", feature = "bigdecimal"))]
    impl ToSql<Numeric, Mysql> for BigDecimal {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
            loop {}
        }
    }
    #[cfg(all(feature = "mysql_backend", feature = "bigdecimal"))]
    impl FromSql<Numeric, Mysql> for BigDecimal {
        fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
            loop {}
        }
    }
}
