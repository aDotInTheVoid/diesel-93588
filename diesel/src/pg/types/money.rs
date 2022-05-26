use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::deserialize::{self, FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::pg::{Pg, PgValue};
use crate::serialize::{self, Output, ToSql};
use crate::sql_types::{BigInt, Money};
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
                fn add(self, rhs: PgMoney) -> Self::Output {
        loop {}
    }
}
impl AddAssign for PgMoney {
                fn add_assign(&mut self, rhs: PgMoney) {
        loop {}
    }
}
impl Sub for PgMoney {
    type Output = Self;
                fn sub(self, rhs: PgMoney) -> Self::Output {
        loop {}
    }
}
impl SubAssign for PgMoney {
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
