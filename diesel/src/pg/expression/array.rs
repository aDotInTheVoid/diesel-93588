use crate::expression::{
    AppearsOnTable, AsExpressionList, Expression, SelectableExpression, ValidGrouping,
};
use crate::pg::Pg;
use crate::query_builder::{AstPass, QueryFragment, QueryId};
use crate::sql_types;
use std::marker::PhantomData;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct ArrayLiteral<T, ST> {
    elements: T,
    _marker: PhantomData<ST>,
}
#[cfg(feature = "postgres_backend")]
pub fn array<ST, T>(elements: T) -> ArrayLiteral<T::Expression, ST>
where
    T: AsExpressionList<ST>,
{
    loop {}
}
impl<T, ST> Expression for ArrayLiteral<T, ST>
where
    ST: 'static,
    T: Expression,
{
    type SqlType = sql_types::Array<ST>;
}
impl<T, ST> QueryFragment<Pg> for ArrayLiteral<T, ST>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(
        &'b self,
        mut out: AstPass<'_, 'b, Pg>,
    ) -> crate::result::QueryResult<()> {
        loop {}
    }
}
impl<T, ST, QS> SelectableExpression<QS> for ArrayLiteral<T, ST>
where
    T: SelectableExpression<QS>,
    ArrayLiteral<T, ST>: AppearsOnTable<QS>,
{}
impl<T, ST, QS> AppearsOnTable<QS> for ArrayLiteral<T, ST>
where
    T: AppearsOnTable<QS>,
    ArrayLiteral<T, ST>: Expression,
{}
impl<T, ST, GB> ValidGrouping<GB> for ArrayLiteral<T, ST>
where
    T: ValidGrouping<GB>,
{
    type IsAggregate = T::IsAggregate;
}
