use crate::backend::{self, Backend};
use crate::expression::select_by::SelectBy;
use crate::row::{NamedRow, Row};
use crate::sql_types::{SingleValue, SqlType, Untyped};
use crate::Selectable;
use std::error::Error;
use std::result;
pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;
pub trait Queryable<ST, DB>: Sized
where
    DB: Backend,
{
    type Row: FromStaticSqlRow<ST, DB>;
    fn build(row: Self::Row) -> Result<Self>;
}
#[doc(inline)]
pub use diesel_derives::Queryable;
pub trait QueryableByName<DB>
where
    Self: Sized,
    DB: Backend,
{
    fn build<'a>(row: &impl NamedRow<'a, DB>) -> Result<Self>;
}
#[doc(inline)]
pub use diesel_derives::QueryableByName;
pub trait FromSql<A, DB: Backend>: Sized {
    fn from_sql(bytes: backend::RawValue<'_, DB>) -> Result<Self>;
    #[inline(always)]
    fn from_nullable_sql(bytes: Option<backend::RawValue<'_, DB>>) -> Result<Self> {
        match bytes {
            Some(bytes) => Self::from_sql(bytes),
            None => Err(Box::new(crate::result::UnexpectedNullError)),
        }
    }
}
pub trait FromSqlRow<ST, DB: Backend>: Sized {
    fn build_from_row<'a>(row: &impl Row<'a, DB>) -> Result<Self>;
}
#[doc(inline)]
pub use diesel_derives::FromSqlRow;
pub trait StaticallySizedRow<ST, DB: Backend>: FromSqlRow<ST, DB> {
    const FIELD_COUNT: usize;
}
impl<DB, T> FromSqlRow<Untyped, DB> for T
where
    DB: Backend,
    T: QueryableByName<DB>,
{
    fn build_from_row<'a>(row: &impl Row<'a, DB>) -> Result<Self> {
        loop {}
    }
}
pub trait FromStaticSqlRow<ST, DB: Backend>: Sized {
    fn build_from_row<'a>(row: &impl Row<'a, DB>) -> Result<Self>;
}
#[doc(hidden)]
pub trait SqlTypeOrSelectable {}
impl<ST> SqlTypeOrSelectable for ST where ST: SqlType + SingleValue {}
impl<U, DB> SqlTypeOrSelectable for SelectBy<U, DB>
where
    U: Selectable<DB>,
    DB: Backend,
{
}
impl<T, ST, DB> FromSqlRow<ST, DB> for T
where
    T: Queryable<ST, DB>,
    ST: SqlTypeOrSelectable,
    DB: Backend,
    T::Row: FromStaticSqlRow<ST, DB>,
{
    #[inline(always)]
    fn build_from_row<'a>(row: &impl Row<'a, DB>) -> Result<Self> {
        loop {}
    }
}
impl<T, ST, DB> FromStaticSqlRow<ST, DB> for T
where
    DB: Backend,
    T: FromSql<ST, DB>,
    ST: SingleValue,
{
    fn build_from_row<'a>(row: &impl Row<'a, DB>) -> Result<Self> {
        loop {}
    }
}
impl<T, ST, DB> StaticallySizedRow<ST, DB> for T
where
    ST: SqlTypeOrSelectable + crate::util::TupleSize,
    T: Queryable<ST, DB>,
    DB: Backend,
{
    const FIELD_COUNT: usize = <ST as crate::util::TupleSize>::SIZE;
}
