use crate::dsl::{Filter, OrFilter};
use crate::expression_methods::*;
use crate::query_source::*;
pub trait FilterDsl<Predicate> {
        type Output;
        fn filter(self, predicate: Predicate) -> Self::Output;
}
impl<T, Predicate> FilterDsl<Predicate> for T
where
    T: Table,
    T::Query: FilterDsl<Predicate>,
{
    type Output = Filter<T::Query, Predicate>;
    fn filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
pub trait FindDsl<PK> {
        type Output;
        fn find(self, id: PK) -> Self::Output;
}
impl<T, PK> FindDsl<PK> for T
where
    T: Table + FilterDsl<<<T as Table>::PrimaryKey as EqAll<PK>>::Output>,
    T::PrimaryKey: EqAll<PK>,
{
    type Output = Filter<Self, <T::PrimaryKey as EqAll<PK>>::Output>;
    fn find(self, id: PK) -> Self::Output {
        loop {}
    }
}
pub trait OrFilterDsl<Predicate> {
        type Output;
        fn or_filter(self, predicate: Predicate) -> Self::Output;
}
impl<T, Predicate> OrFilterDsl<Predicate> for T
where
    T: Table,
    T::Query: OrFilterDsl<Predicate>,
{
    type Output = OrFilter<T::Query, Predicate>;
    fn or_filter(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
