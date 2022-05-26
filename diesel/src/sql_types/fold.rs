use crate::sql_types::{self, is_nullable, SingleValue, SqlType};
pub trait Foldable: SingleValue {
        type Sum: SqlType + SingleValue;
        type Avg: SqlType + SingleValue;
}
impl<T> Foldable for sql_types::Nullable<T>
where
    T: Foldable + SqlType<IsNull = is_nullable::NotNull>,
{
    type Sum = T::Sum;
    type Avg = T::Avg;
}
macro_rules! foldable_impls {
    ($($Source:ty => ($SumType:ty, $AvgType:ty)),+,) => {
        $(impl Foldable for $Source { type Sum = sql_types::Nullable <$SumType >; type
        Avg = sql_types::Nullable <$AvgType >; })+
    };
}
foldable_impls! {
    sql_types::SmallInt => (sql_types::BigInt, sql_types::Numeric), sql_types::Integer =>
    (sql_types::BigInt, sql_types::Numeric), sql_types::BigInt => (sql_types::Numeric,
    sql_types::Numeric), sql_types::Float => (sql_types::Float, sql_types::Double),
    sql_types::Double => (sql_types::Double, sql_types::Double), sql_types::Numeric =>
    (sql_types::Numeric, sql_types::Numeric), sql_types::Interval =>
    (sql_types::Interval, sql_types::Interval),
}
#[cfg(feature = "mysql")]
foldable_impls! {
    sql_types::Unsigned < sql_types::SmallInt > => (sql_types::Unsigned <
    sql_types::BigInt >, sql_types::Numeric), sql_types::Unsigned < sql_types::Integer >
    => (sql_types::Unsigned < sql_types::BigInt >, sql_types::Numeric),
    sql_types::Unsigned < sql_types::BigInt > => (sql_types::Numeric,
    sql_types::Numeric),
}
