use crate::query_source::Table;
pub trait LimitDsl {
        type Output;
        fn limit(self, limit: i64) -> Self::Output;
}
impl<T> LimitDsl for T
where
    T: Table,
    T::Query: LimitDsl,
{
    type Output = <T::Query as LimitDsl>::Output;
    fn limit(self, limit: i64) -> Self::Output {
        loop {}
    }
}
