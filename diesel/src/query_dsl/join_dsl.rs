use crate::helper_types;
use crate::query_builder::AsQuery;
use crate::query_source::joins::OnClauseWrapper;
use crate::query_source::{JoinTo, QuerySource, Table};
#[doc(hidden)]
pub trait InternalJoinDsl<Rhs, Kind, On> {
    type Output;
    fn join(self, rhs: Rhs, kind: Kind, on: On) -> Self::Output;
}
impl<T, Rhs, Kind, On> InternalJoinDsl<Rhs, Kind, On> for T
where
    T: Table + AsQuery,
    T::Query: InternalJoinDsl<Rhs, Kind, On>,
{
    type Output = <T::Query as InternalJoinDsl<Rhs, Kind, On>>::Output;
    fn join(self, rhs: Rhs, kind: Kind, on: On) -> Self::Output {
        loop {}
    }
}
#[doc(hidden)]
pub trait JoinWithImplicitOnClause<Rhs, Kind> {
    type Output;
    fn join_with_implicit_on_clause(self, rhs: Rhs, kind: Kind) -> Self::Output;
}
impl<Lhs, Rhs, Kind> JoinWithImplicitOnClause<Rhs, Kind> for Lhs
where
    Lhs: JoinTo<Rhs>,
    Lhs: InternalJoinDsl<
        <Lhs as JoinTo<Rhs>>::FromClause,
        Kind,
        <Lhs as JoinTo<Rhs>>::OnClause,
    >,
{
    type Output = <Lhs as InternalJoinDsl<Lhs::FromClause, Kind, Lhs::OnClause>>::Output;
    fn join_with_implicit_on_clause(self, rhs: Rhs, kind: Kind) -> Self::Output {
        loop {}
    }
}
pub trait JoinOnDsl: Sized {
    fn on<On>(self, on: On) -> helper_types::On<Self, On> {
        OnClauseWrapper::new(self, on)
    }
}
impl<T: QuerySource> JoinOnDsl for T {}
