#[cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")]
#[doc(inline)]
pub use self::private::{PartialRow, RowGatWorkaround};
#[cfg(not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"))]
pub(crate) use self::private::{PartialRow, RowGatWorkaround};
use crate::backend::{self, Backend};
use crate::deserialize;
use deserialize::FromSql;
use std::ops::Range;
pub trait RowIndex<I> {
    fn idx(&self, idx: I) -> Option<usize>;
}
pub type FieldRet<'a, R, DB> = <R as RowGatWorkaround<'a, DB>>::Field;
pub trait Row<
    'a,
    DB: Backend,
>: RowIndex<
        usize,
    > + for<'b> RowIndex<&'b str> + for<'b> RowGatWorkaround<'b, DB> + Sized {
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc(hidden)
    )]
    type InnerPartialRow: Row<'a, DB>;
    fn field_count(&self) -> usize;
    fn get<'b, I>(&'b self, idx: I) -> Option<FieldRet<'b, Self, DB>>
    where
        'a: 'b,
        Self: RowIndex<I>;
    fn get_value<ST, T, I>(&self, idx: I) -> crate::deserialize::Result<T>
    where
        Self: RowIndex<I>,
        T: FromSql<ST, DB>,
    {
        loop {}
    }
    #[cfg_attr(
        not(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"),
        doc(hidden)
    )]
    fn partial_row(&self, range: Range<usize>) -> PartialRow<'_, Self::InnerPartialRow>;
}
pub trait Field<'a, DB: Backend> {
    fn field_name(&self) -> Option<&str>;
    fn value(&self) -> Option<backend::RawValue<'_, DB>>;
    fn is_null(&self) -> bool {
        loop {}
    }
}
impl<'a, 'b, DB, R> RowGatWorkaround<'a, DB> for PartialRow<'b, R>
where
    DB: Backend,
    R: RowGatWorkaround<'a, DB>,
{
    type Field = R::Field;
}
pub trait NamedRow<'a, DB: Backend>: Row<'a, DB> {
    fn get<ST, T>(&self, column_name: &str) -> deserialize::Result<T>
    where
        T: FromSql<ST, DB>;
}
impl<'a, R, DB> NamedRow<'a, DB> for R
where
    R: Row<'a, DB>,
    DB: Backend,
{
    fn get<ST, T>(&self, column_name: &str) -> deserialize::Result<T>
    where
        T: FromSql<ST, DB>,
    {
        loop {}
    }
}
mod private {
    use super::*;
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
    )]
    pub trait RowGatWorkaround<'a, DB: Backend> {
        type Field: Field<'a, DB>;
    }
    #[derive(Debug)]
    #[cfg_attr(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
        cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
    )]
    pub struct PartialRow<'a, R> {
        inner: &'a R,
        range: Range<usize>,
    }
    impl<'a, R> PartialRow<'a, R> {
        pub fn new<'b, DB>(inner: &'a R, range: Range<usize>) -> Self
        where
            R: Row<'b, DB>,
            DB: Backend,
        {
            loop {}
        }
    }
    impl<'a, 'b, DB, R> Row<'a, DB> for PartialRow<'b, R>
    where
        DB: Backend,
        R: Row<'a, DB>,
    {
        type InnerPartialRow = R;
        fn field_count(&self) -> usize {
            loop {}
        }
        fn get<'c, I>(
            &'c self,
            idx: I,
        ) -> Option<<Self as RowGatWorkaround<'c, DB>>::Field>
        where
            'a: 'c,
            Self: RowIndex<I>,
        {
            loop {}
        }
        fn partial_row(&self, range: Range<usize>) -> PartialRow<'_, R> {
            loop {}
        }
    }
    impl<'a, 'b, R> RowIndex<&'a str> for PartialRow<'b, R>
    where
        R: RowIndex<&'a str>,
    {
        fn idx(&self, idx: &'a str) -> Option<usize> {
            loop {}
        }
    }
    impl<'a, R> RowIndex<usize> for PartialRow<'a, R>
    where
        R: RowIndex<usize>,
    {
        fn idx(&self, idx: usize) -> Option<usize> {
            loop {}
        }
    }
}
