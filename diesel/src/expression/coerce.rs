use std::marker::PhantomData;
use crate::backend::Backend;
use crate::expression::*;
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::{DieselNumericOps, SqlType};
#[derive(Debug, Copy, Clone, QueryId, DieselNumericOps)]
#[doc(hidden)]
pub struct Coerce<T, ST> {
    expr: T,
    _marker: PhantomData<ST>,
}
impl<T, ST> Coerce<T, ST> {
    pub fn new(expr: T) -> Self {
        loop {}
    }
}
impl<T, ST> Expression for Coerce<T, ST>
where
    T: Expression,
    ST: SqlType + TypedExpressionType,
{
    type SqlType = ST;
}
impl<T, ST, QS> SelectableExpression<QS> for Coerce<T, ST>
where
    T: SelectableExpression<QS>,
    Self: Expression,
{}
impl<T, ST, QS> AppearsOnTable<QS> for Coerce<T, ST>
where
    T: AppearsOnTable<QS>,
    Self: Expression,
{}
impl<T, ST, DB> QueryFragment<DB> for Coerce<T, ST>
where
    T: QueryFragment<DB>,
    DB: Backend,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, ST, GB> ValidGrouping<GB> for Coerce<T, ST>
where
    T: ValidGrouping<GB>,
{
    type IsAggregate = T::IsAggregate;
}
