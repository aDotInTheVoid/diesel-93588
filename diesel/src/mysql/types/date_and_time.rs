#[cfg(feature = "chrono")]
use chrono::*;
use std::io::Write;
use std::os::raw as libc;
use std::{mem, slice};
use crate::deserialize::{self, FromSql, FromSqlRow};
use crate::expression::AsExpression;
use crate::mysql::{Mysql, MysqlValue};
use crate::serialize::{self, IsNull, Output, ToSql};
use crate::sql_types::{Date, Datetime, Time, Timestamp};
#[repr(C)]
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[non_exhaustive]
#[diesel(sql_type = Timestamp)]
#[diesel(sql_type = Time)]
#[diesel(sql_type = Date)]
#[diesel(sql_type = Datetime)]
pub struct MysqlTime {
        pub year: libc::c_uint,
        pub month: libc::c_uint,
        pub day: libc::c_uint,
        pub hour: libc::c_uint,
        pub minute: libc::c_uint,
        pub second: libc::c_uint,
        pub second_part: libc::c_ulong,
        pub neg: bool,
        pub time_type: MysqlTimestampType,
        pub time_zone_displacement: libc::c_int,
}
impl MysqlTime {
        #[allow(clippy::too_many_arguments)]
    pub fn new(
        year: libc::c_uint,
        month: libc::c_uint,
        day: libc::c_uint,
        hour: libc::c_uint,
        minute: libc::c_uint,
        second: libc::c_uint,
        second_part: libc::c_ulong,
        neg: bool,
        time_type: MysqlTimestampType,
        time_zone_displacement: libc::c_int,
    ) -> Self {
        loop {}
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct MysqlTimestampType(libc::c_int);
impl MysqlTimestampType {
            pub const MYSQL_TIMESTAMP_NONE: MysqlTimestampType = MysqlTimestampType(-2);
            pub const MYSQL_TIMESTAMP_ERROR: MysqlTimestampType = MysqlTimestampType(-1);
            pub const MYSQL_TIMESTAMP_DATE: MysqlTimestampType = MysqlTimestampType(0);
            pub const MYSQL_TIMESTAMP_DATETIME: MysqlTimestampType = MysqlTimestampType(1);
            pub const MYSQL_TIMESTAMP_TIME: MysqlTimestampType = MysqlTimestampType(2);
            pub const MYSQL_TIMESTAMP_DATETIME_TZ: MysqlTimestampType = MysqlTimestampType(3);
}
macro_rules! mysql_time_impls {
    ($ty:ty) => {
        #[cfg(feature = "mysql_backend")] impl ToSql <$ty, Mysql > for MysqlTime { fn
        to_sql <'b > (&'b self, out : & mut Output <'b, '_, Mysql >) -> serialize::Result
        { let bytes = unsafe { let bytes_ptr = self as * const MysqlTime as * const u8;
        slice::from_raw_parts(bytes_ptr, mem::size_of::< MysqlTime > ()) }; out
        .write_all(bytes) ?; Ok(IsNull::No) } } #[cfg(feature = "mysql_backend")] impl
        FromSql <$ty, Mysql > for MysqlTime { fn from_sql(value : MysqlValue <'_ >) ->
        deserialize::Result < Self > { value.time_value() } }
    };
}
mysql_time_impls!(Datetime);
mysql_time_impls!(Timestamp);
mysql_time_impls!(Time);
mysql_time_impls!(Date);
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl ToSql<Datetime, Mysql> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl FromSql<Datetime, Mysql> for NaiveDateTime {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl ToSql<Timestamp, Mysql> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl FromSql<Timestamp, Mysql> for NaiveDateTime {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl ToSql<Time, Mysql> for NaiveTime {
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, Mysql>,
    ) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl FromSql<Time, Mysql> for NaiveTime {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl ToSql<Date, Mysql> for NaiveDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        loop {}
    }
}
#[cfg(all(feature = "chrono", feature = "mysql_backend"))]
impl FromSql<Date, Mysql> for NaiveDate {
    fn from_sql(bytes: MysqlValue<'_>) -> deserialize::Result<Self> {
        loop {}
    }
}
#[cfg(all(test, feature = "chrono"))]
mod tests {
    extern crate chrono;
    extern crate dotenvy;
    use self::chrono::{Duration, NaiveDate, NaiveTime, Utc};
    use crate::dsl::{now, sql};
    use crate::prelude::*;
    use crate::select;
    use crate::sql_types::{Date, Datetime, Time, Timestamp};
    use crate::test_helpers::connection;
    #[test]
    fn unix_epoch_encodes_correctly() {
        loop {}
    }
    #[test]
    fn unix_epoch_decodes_correctly() {
        loop {}
    }
    #[test]
    fn times_relative_to_now_encode_correctly() {
        loop {}
    }
    #[test]
    fn times_of_day_encode_correctly() {
        loop {}
    }
    #[test]
    fn times_of_day_decode_correctly() {
        loop {}
    }
    #[test]
    fn dates_encode_correctly() {
        loop {}
    }
    #[test]
    fn dates_decode_correctly() {
        loop {}
    }
}
