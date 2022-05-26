use crate::query_source::Table;
pub trait OffsetDsl {
        type Output;
        fn offset(self, offset: i64) -> Self::Output;
}
impl<T> OffsetDsl for T
where
    T: Table,
    T::Query: OffsetDsl,
{
    type Output = <T::Query as OffsetDsl>::Output;
    fn offset(self, offset: i64) -> Self::Output {
        loop {}
    }
}
