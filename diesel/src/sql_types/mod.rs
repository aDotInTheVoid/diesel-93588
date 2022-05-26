mod fold;
pub mod ops;
mod ord;
pub use self::fold::Foldable;
pub use self::ord::SqlOrd;
use crate::expression::TypedExpressionType;
use crate::query_builder::QueryId;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 16, array_oid = 1000))]
#[diesel(sqlite_type(name = "Integer"))]
#[diesel(mysql_type(name = "Tiny"))]
pub struct Bool;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(mysql_type(name = "Tiny"))]
pub struct TinyInt;
#[doc(hidden)]
pub type Tinyint = TinyInt;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 21, array_oid = 1005))]
#[diesel(sqlite_type(name = "SmallInt"))]
#[diesel(mysql_type(name = "Short"))]
pub struct SmallInt;
#[doc(hidden)]
pub type Int2 = SmallInt;
#[doc(hidden)]
pub type Smallint = SmallInt;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 23, array_oid = 1007))]
#[diesel(sqlite_type(name = "Integer"))]
#[diesel(mysql_type(name = "Long"))]
pub struct Integer;
#[doc(hidden)]
pub type Int4 = Integer;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 20, array_oid = 1016))]
#[diesel(sqlite_type(name = "Long"))]
#[diesel(mysql_type(name = "LongLong"))]
pub struct BigInt;
#[doc(hidden)]
pub type Int8 = BigInt;
#[doc(hidden)]
pub type Bigint = BigInt;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 700, array_oid = 1021))]
#[diesel(sqlite_type(name = "Float"))]
#[diesel(mysql_type(name = "Float"))]
pub struct Float;
#[doc(hidden)]
pub type Float4 = Float;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 701, array_oid = 1022))]
#[diesel(sqlite_type(name = "Double"))]
#[diesel(mysql_type(name = "Double"))]
pub struct Double;
#[doc(hidden)]
pub type Float8 = Double;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 1700, array_oid = 1231))]
#[diesel(mysql_type(name = "Numeric"))]
#[diesel(sqlite_type(name = "Double"))]
pub struct Numeric;
pub type Decimal = Numeric;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 25, array_oid = 1009))]
#[diesel(sqlite_type(name = "Text"))]
#[diesel(mysql_type(name = "String"))]
pub struct Text;
pub type VarChar = Text;
#[doc(hidden)]
pub type Varchar = VarChar;
#[doc(hidden)]
pub type Char = Text;
#[doc(hidden)]
pub type Tinytext = Text;
#[doc(hidden)]
pub type Mediumtext = Text;
#[doc(hidden)]
pub type Longtext = Text;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 17, array_oid = 1001))]
#[diesel(sqlite_type(name = "Binary"))]
#[diesel(mysql_type(name = "Blob"))]
pub struct Binary;
#[doc(hidden)]
pub type Tinyblob = Binary;
#[doc(hidden)]
pub type Blob = Binary;
#[doc(hidden)]
pub type Mediumblob = Binary;
#[doc(hidden)]
pub type Longblob = Binary;
#[doc(hidden)]
pub type Varbinary = Binary;
#[doc(hidden)]
pub type Bit = Binary;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 1082, array_oid = 1182))]
#[diesel(sqlite_type(name = "Text"))]
#[diesel(mysql_type(name = "Date"))]
pub struct Date;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 1186, array_oid = 1187))]
pub struct Interval;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 1083, array_oid = 1183))]
#[diesel(sqlite_type(name = "Text"))]
#[diesel(mysql_type(name = "Time"))]
pub struct Time;
#[cfg_attr(feature = "chrono", doc = " [NaiveDateTime]: chrono::naive::NaiveDateTime")]
#[cfg_attr(
    not(feature = "chrono"),
    doc = " [NaiveDateTime]: https://docs.rs/chrono/*/chrono/naive/struct.NaiveDateTime.html"
)]
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 1114, array_oid = 1115))]
#[diesel(sqlite_type(name = "Text"))]
#[diesel(mysql_type(name = "Timestamp"))]
pub struct Timestamp;
#[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
#[diesel(postgres_type(oid = 114, array_oid = 199))]
#[diesel(mysql_type(name = "String"))]
pub struct Json;
#[derive(Debug, Clone, Copy, Default)]
pub struct Nullable<ST>(ST);
impl<ST> SqlType for Nullable<ST>
where
    ST: SqlType,
{
    type IsNull = is_nullable::IsNullable;
}
#[doc(inline)]
#[cfg(feature = "postgres_backend")]
pub use crate::pg::sql_types::*;
#[doc(inline)]
#[cfg(feature = "mysql_backend")]
pub use crate::mysql::sql_types::{Datetime, Unsigned};
#[doc(inline)]
#[cfg(feature = "sqlite")]
pub use crate::sqlite::sql_types::Timestamptz as TimestamptzSqlite;
pub trait HasSqlType<ST>: TypeMetadata {
                    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata;
}
pub trait TypeMetadata {
                        type TypeMetadata;
                    type MetadataLookup: ?Sized;
}
pub trait IntoNullable {
                type Nullable;
}
impl<T> IntoNullable for T
where
    T: SqlType<IsNull = is_nullable::NotNull> + SingleValue,
{
    type Nullable = Nullable<T>;
}
impl<T> IntoNullable for Nullable<T>
where
    T: SqlType,
{
    type Nullable = Self;
}
pub trait IntoNotNullable {
                type NotNullable;
}
impl<T> IntoNotNullable for T
where
    T: SqlType<IsNull = is_nullable::NotNull>,
{
    type NotNullable = T;
}
impl<T> IntoNotNullable for Nullable<T>
where
    T: SqlType,
{
    type NotNullable = T;
}
pub trait SingleValue: SqlType {}
impl<T: SqlType + SingleValue> SingleValue for Nullable<T> {}
#[doc(inline)]
pub use diesel_derives::DieselNumericOps;
#[doc(inline)]
pub use diesel_derives::SqlType;
pub trait SqlType: 'static {
                            type IsNull: OneIsNullable<is_nullable::IsNullable>
        + OneIsNullable<is_nullable::NotNull>;
}
pub trait OneIsNullable<Other> {
        type Out: OneIsNullable<is_nullable::IsNullable>
        + OneIsNullable<is_nullable::NotNull>;
}
pub trait AllAreNullable<Other> {
        type Out: AllAreNullable<is_nullable::NotNull>
        + AllAreNullable<is_nullable::IsNullable>;
}
pub trait MaybeNullableType<O> {
        type Out: SqlType + TypedExpressionType;
}
pub mod is_nullable {
    use super::*;
                    #[derive(Debug, Clone, Copy)]
    pub struct NotNull;
                        #[derive(Debug, Clone, Copy)]
    pub struct IsNullable;
    impl OneIsNullable<NotNull> for NotNull {
        type Out = NotNull;
    }
    impl OneIsNullable<IsNullable> for NotNull {
        type Out = IsNullable;
    }
    impl OneIsNullable<NotNull> for IsNullable {
        type Out = IsNullable;
    }
    impl OneIsNullable<IsNullable> for IsNullable {
        type Out = IsNullable;
    }
    impl AllAreNullable<NotNull> for NotNull {
        type Out = NotNull;
    }
    impl AllAreNullable<IsNullable> for NotNull {
        type Out = NotNull;
    }
    impl AllAreNullable<NotNull> for IsNullable {
        type Out = NotNull;
    }
    impl AllAreNullable<IsNullable> for IsNullable {
        type Out = IsNullable;
    }
    impl<O> MaybeNullableType<O> for NotNull
    where
        O: SqlType + TypedExpressionType,
    {
        type Out = O;
    }
    impl<O> MaybeNullableType<O> for IsNullable
    where
        O: SqlType,
        Nullable<O>: TypedExpressionType,
    {
        type Out = Nullable<O>;
    }
        pub type MaybeNullable<N, T> = <N as MaybeNullableType<T>>::Out;
            pub type IsOneNullable<S1, S2> = <IsSqlTypeNullable<
        S1,
    > as OneIsNullable<IsSqlTypeNullable<S2>>>::Out;
            pub type AreAllNullable<S1, S2> = <IsSqlTypeNullable<
        S1,
    > as AllAreNullable<IsSqlTypeNullable<S2>>>::Out;
        pub type IsSqlTypeNullable<T> = <T as SqlType>::IsNull;
}
pub trait BoolOrNullableBool {}
impl BoolOrNullableBool for Bool {}
impl BoolOrNullableBool for Nullable<Bool> {}
#[doc(inline)]
pub use crate::expression::expression_types::Untyped;
