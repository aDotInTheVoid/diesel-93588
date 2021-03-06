use std::ops::Mul;
use crate::data_types::PgInterval;
#[cfg(feature = "postgres_backend")]
pub trait IntervalDsl: Sized + From<i32> + Mul<Self, Output = Self> {
    fn microseconds(self) -> PgInterval;
    fn days(self) -> PgInterval;
    fn months(self) -> PgInterval;
    fn milliseconds(self) -> PgInterval {
        loop {}
    }
    fn seconds(self) -> PgInterval {
        loop {}
    }
    fn minutes(self) -> PgInterval {
        loop {}
    }
    fn hours(self) -> PgInterval {
        loop {}
    }
    fn weeks(self) -> PgInterval {
        loop {}
    }
    fn years(self) -> PgInterval {
        loop {}
    }
    fn microsecond(self) -> PgInterval {
        loop {}
    }
    fn millisecond(self) -> PgInterval {
        loop {}
    }
    fn second(self) -> PgInterval {
        loop {}
    }
    fn minute(self) -> PgInterval {
        loop {}
    }
    fn hour(self) -> PgInterval {
        loop {}
    }
    fn day(self) -> PgInterval {
        loop {}
    }
    fn week(self) -> PgInterval {
        loop {}
    }
    fn month(self) -> PgInterval {
        loop {}
    }
    fn year(self) -> PgInterval {
        loop {}
    }
}
impl IntervalDsl for i32 {
    fn microseconds(self) -> PgInterval {
        loop {}
    }
    fn days(self) -> PgInterval {
        loop {}
    }
    fn months(self) -> PgInterval {
        loop {}
    }
    fn milliseconds(self) -> PgInterval {
        loop {}
    }
    fn seconds(self) -> PgInterval {
        loop {}
    }
    fn minutes(self) -> PgInterval {
        loop {}
    }
    fn hours(self) -> PgInterval {
        loop {}
    }
}
impl IntervalDsl for i64 {
    fn microseconds(self) -> PgInterval {
        loop {}
    }
    fn days(self) -> PgInterval {
        loop {}
    }
    fn months(self) -> PgInterval {
        loop {}
    }
}
impl IntervalDsl for f64 {
    fn microseconds(self) -> PgInterval {
        loop {}
    }
    fn days(self) -> PgInterval {
        loop {}
    }
    fn months(self) -> PgInterval {
        loop {}
    }
    fn years(self) -> PgInterval {
        loop {}
    }
}
#[cfg(test)]
#[allow(clippy::items_after_statements)]
mod tests {
    extern crate dotenvy;
    extern crate quickcheck;
    use self::quickcheck::quickcheck;
    use super::*;
    use crate::data_types::PgInterval;
    use crate::dsl::sql;
    use crate::prelude::*;
    use crate::test_helpers::*;
    use crate::{select, sql_types};
    macro_rules! test_fn {
        ($tpe:ty, $test_name:ident, $units:ident, $max_range:expr) => {
            test_fn!($tpe, $test_name, $units, $max_range, 1);
        };
        ($tpe:ty, $test_name:ident, $units:ident, $max_range:expr, $max_diff:expr) => {
            fn $test_name (val : $tpe) -> bool { if val > $max_range || val < (- 1 as
            $tpe) * $max_range || (val as f64).is_nan() { return true; } let conn = & mut
            pg_connection(); let sql_str = format!(concat!("'{} ", stringify!($units),
            "'::interval"), val); let query = select(sql::< sql_types::Interval > (&
            sql_str)); let value = val.$units (); query.get_result::< PgInterval > (conn)
            .map(| res | { value.months == res.months && value.days == res.days && (value
            .microseconds - res.microseconds).abs() <= $max_diff }).unwrap_or(false) }
            quickcheck($test_name as fn ($tpe) -> bool);
        };
    }
    #[test]
    fn intervals_match_pg_values_i32() {
        loop {}
    }
    #[test]
    fn intervals_match_pg_values_i64() {
        loop {}
    }
    #[test]
    fn intervals_match_pg_values_f64() {
        loop {}
    }
}
