use crate::expression::array_comparison::MaybeEmpty;
use crate::expression::*;
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::SqlType;
use std::marker::PhantomData;
#[derive(Debug, Copy, Clone, QueryId)]
pub struct Subselect<T, ST> {
    values: T,
    _sql_type: PhantomData<ST>,
}
impl<T, ST> Subselect<T, ST> {
    pub(crate) fn new(values: T) -> Self {
        loop {}
    }
}
impl<T: SelectQuery, ST> Expression for Subselect<T, ST>
where
    ST: SqlType + TypedExpressionType,
{
    type SqlType = ST;
}
impl<T, ST> MaybeEmpty for Subselect<T, ST> {
    fn is_empty(&self) -> bool {
        loop {}
    }
}
impl<T, ST, QS> SelectableExpression<QS> for Subselect<T, ST>
where
    Subselect<T, ST>: AppearsOnTable<QS>,
    T: ValidSubselect<QS>,
{}
impl<T, ST, QS> AppearsOnTable<QS> for Subselect<T, ST>
where
    Subselect<T, ST>: Expression,
    T: ValidSubselect<QS>,
{}
impl<T, ST, GB> ValidGrouping<GB> for Subselect<T, ST> {
    type IsAggregate = is_aggregate::Never;
}
impl<T, ST, DB> QueryFragment<DB> for Subselect<T, ST>
where
    DB: Backend,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
pub trait ValidSubselect<QS> {}
