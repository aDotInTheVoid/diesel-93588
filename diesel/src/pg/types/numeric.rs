#[cfg(feature = "bigdecimal")]
mod bigdecimal {
    extern crate bigdecimal;
    extern crate num_bigint;
    extern crate num_integer;
    extern crate num_traits;
    use self::bigdecimal::BigDecimal;
    use self::num_bigint::{BigInt, BigUint, Sign};
    use self::num_integer::Integer;
    use self::num_traits::{Signed, ToPrimitive, Zero};
    use crate::deserialize::{self, FromSql};
    use crate::pg::data_types::PgNumeric;
    use crate::pg::{Pg, PgValue};
    use crate::serialize::{self, Output, ToSql};
    use crate::sql_types::Numeric;
    use std::convert::{TryFrom, TryInto};
    use std::error::Error;
    struct ToBase10000(Option<BigUint>);
    impl Iterator for ToBase10000 {
        type Item = i16;
        fn next(&mut self) -> Option<Self::Item> {
            loop {}
        }
    }
    #[cfg(all(feature = "postgres_backend", feature = "bigdecimal"))]
    impl<'a> TryFrom<&'a PgNumeric> for BigDecimal {
        type Error = Box<dyn Error + Send + Sync>;
        fn try_from(numeric: &'a PgNumeric) -> deserialize::Result<Self> {
            loop {}
        }
    }
    #[cfg(all(feature = "postgres_backend", feature = "bigdecimal"))]
    impl TryFrom<PgNumeric> for BigDecimal {
        type Error = Box<dyn Error + Send + Sync>;
        fn try_from(numeric: PgNumeric) -> deserialize::Result<Self> {
            loop {}
        }
    }
    #[cfg(all(feature = "postgres_backend", feature = "bigdecimal"))]
    impl<'a> From<&'a BigDecimal> for PgNumeric {
        #[allow(clippy::assign_op_pattern, clippy::redundant_closure)]
        fn from(decimal: &'a BigDecimal) -> Self {
            loop {}
        }
    }
    #[cfg(all(feature = "postgres_backend", feature = "bigdecimal"))]
    impl From<BigDecimal> for PgNumeric {
        fn from(bigdecimal: BigDecimal) -> Self {
            loop {}
        }
    }
    #[cfg(all(feature = "postgres_backend", feature = "bigdecimal"))]
    impl ToSql<Numeric, Pg> for BigDecimal {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
            loop {}
        }
    }
    #[cfg(all(feature = "postgres_backend", feature = "bigdecimal"))]
    impl FromSql<Numeric, Pg> for BigDecimal {
        fn from_sql(numeric: PgValue<'_>) -> deserialize::Result<Self> {
            loop {}
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::str::FromStr;
        #[test]
        fn bigdecimal_to_pgnumeric_converts_digits_to_base_10000() {
            loop {}
        }
        #[test]
        fn bigdecimal_to_pg_numeric_properly_adjusts_scale() {
            loop {}
        }
        #[test]
        fn bigdecimal_to_pg_numeric_retains_sign() {
            loop {}
        }
        #[test]
        fn bigdecimal_with_negative_scale_to_pg_numeric_works() {
            loop {}
        }
        #[test]
        fn bigdecimal_with_negative_weight_to_pg_numeric_works() {
            loop {}
        }
        #[test]
        fn pg_numeric_to_bigdecimal_works() {
            loop {}
        }
    }
}
