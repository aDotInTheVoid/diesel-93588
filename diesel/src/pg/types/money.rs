//! Support for Money values under PostgreSQL.
use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::deserialize::{self, FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, Output, ToSql};
use crate::sql_types::{BigInt, Money};
/// Money is represented in Postgres as a 64 bit signed integer.  This struct is a dumb wrapper
/// type, meant only to indicate the integer's meaning.  The fractional precision of the value is
/// determined by the [`lc_monetary` setting of the database](https://www.postgresql.org/docs/9.6/static/datatype-money.html).
/// This struct is re-exported as `Cents` as a convenient and conventional expression of a typical
/// unit of 1/100th of currency. For other names or precisions, users might consider a differently
/// named `use` of the `PgMoney` struct.
///
/// ```rust
/// use diesel::data_types::PgMoney as Pence; // 1/100th unit of Pound
/// use diesel::data_types::PgMoney as Fils;  // 1/1000th unit of Dinar
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AsExpression, FromSqlRow)]
#[diesel(sql_type = Money)]
pub struct PgMoney(pub i64);
#[cfg(feature = "postgres_backend")]
impl FromSql<Money, Pg> for PgMoney {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
impl ToSql<Money, Pg> for PgMoney {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        loop {}
    }
}
impl Add for PgMoney {
    type Output = Self;
    /// # Panics
    ///
    /// Performs a checked addition, and will `panic!` on overflow in both `debug` and `release`.
    fn add(self, rhs: PgMoney) -> Self::Output {
        loop {}
    }
}
impl AddAssign for PgMoney {
    /// # Panics
    ///
    /// Performs a checked addition, and will `panic!` on overflow in both `debug` and `release`.
    fn add_assign(&mut self, rhs: PgMoney) {
        loop {}
    }
}
impl Sub for PgMoney {
    type Output = Self;
    /// # Panics
    ///
    /// Performs a checked subtraction, and will `panic!` on underflow in both `debug` and `release`.
    fn sub(self, rhs: PgMoney) -> Self::Output {
        loop {}
    }
}
impl SubAssign for PgMoney {
    /// # Panics
    ///
    /// Performs a checked subtraction, and will `panic!` on underflow in both `debug` and `release`.
    fn sub_assign(&mut self, rhs: PgMoney) {
        loop {}
    }
}
#[cfg(feature = "quickcheck")]
mod quickcheck_impls {
    extern crate quickcheck;
    use self::quickcheck::{Arbitrary, Gen};
    use super::PgMoney;
    impl Arbitrary for PgMoney {
        fn arbitrary(g: &mut Gen) -> Self {
            loop {}
        }
    }
}
#[test]
fn add_money() {
    loop {}
}
#[test]
fn add_assign_money() {
    loop {}
}
#[test]
#[should_panic(expected = "overflow adding money amounts")]
fn add_money_overflow() {
    loop {}
}
#[test]
#[should_panic(expected = "overflow adding money amounts")]
fn add_assign_money_overflow() {
    loop {}
}
#[test]
fn sub_money() {
    loop {}
}
#[test]
fn sub_assign_money() {
    loop {}
}
#[test]
#[should_panic(expected = "underflow subtracting money amounts")]
fn sub_money_underflow() {
    loop {}
}
#[test]
#[should_panic(expected = "underflow subtracting money amounts")]
fn sub_assign_money_underflow() {
    loop {}
}
