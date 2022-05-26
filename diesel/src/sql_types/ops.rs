use super::*;
pub trait Add {
    type Rhs: SqlType;
    type Output: SqlType;
}
pub trait Sub {
    type Rhs: SqlType;
    type Output: SqlType;
}
pub trait Mul {
    type Rhs: SqlType;
    type Output: SqlType;
}
pub trait Div {
    type Rhs: SqlType;
    type Output: SqlType;
}
macro_rules! numeric_type {
    ($($tpe:ident),*) => {
        $(impl Add for $tpe { type Rhs = $tpe; type Output = $tpe; } impl Sub for $tpe {
        type Rhs = $tpe; type Output = $tpe; } impl Mul for $tpe { type Rhs = $tpe; type
        Output = $tpe; } impl Div for $tpe { type Rhs = $tpe; type Output = $tpe; })*
    };
}
numeric_type!(SmallInt, Integer, BigInt, Float, Double, Numeric);
impl Add for Time {
    type Rhs = Interval;
    type Output = Time;
}
impl Sub for Time {
    type Rhs = Interval;
    type Output = Time;
}
impl Add for Date {
    type Rhs = Interval;
    type Output = Timestamp;
}
impl Sub for Date {
    type Rhs = Interval;
    type Output = Timestamp;
}
impl Add for Timestamp {
    type Rhs = Interval;
    type Output = Timestamp;
}
impl Sub for Timestamp {
    type Rhs = Interval;
    type Output = Timestamp;
}
impl Add for Interval {
    type Rhs = Interval;
    type Output = Interval;
}
impl Sub for Interval {
    type Rhs = Interval;
    type Output = Interval;
}
impl Mul for Interval {
    type Rhs = Integer;
    type Output = Interval;
}
impl Div for Interval {
    type Rhs = Integer;
    type Output = Interval;
}
impl<T> Add for Nullable<T>
where
    T: Add + SqlType<IsNull = is_nullable::NotNull>,
    T::Rhs: SqlType<IsNull = is_nullable::NotNull>,
    T::Output: SqlType<IsNull = is_nullable::NotNull>,
{
    type Rhs = Nullable<T::Rhs>;
    type Output = Nullable<T::Output>;
}
impl<T> Sub for Nullable<T>
where
    T: Sub + SqlType<IsNull = is_nullable::NotNull>,
    T::Rhs: SqlType<IsNull = is_nullable::NotNull>,
    T::Output: SqlType<IsNull = is_nullable::NotNull>,
{
    type Rhs = Nullable<T::Rhs>;
    type Output = Nullable<T::Output>;
}
impl<T> Mul for Nullable<T>
where
    T: Mul + SqlType<IsNull = is_nullable::NotNull>,
    T::Rhs: SqlType<IsNull = is_nullable::NotNull>,
    T::Output: SqlType<IsNull = is_nullable::NotNull>,
{
    type Rhs = Nullable<T::Rhs>;
    type Output = Nullable<T::Output>;
}
impl<T> Div for Nullable<T>
where
    T: Div + SqlType<IsNull = is_nullable::NotNull>,
    T::Rhs: SqlType<IsNull = is_nullable::NotNull>,
    T::Output: SqlType<IsNull = is_nullable::NotNull>,
{
    type Rhs = Nullable<T::Rhs>;
    type Output = Nullable<T::Output>;
}
