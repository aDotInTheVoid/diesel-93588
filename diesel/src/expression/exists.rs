use crate::backend::{sql_dialect, Backend, SqlDialect};
use crate::expression::subselect::Subselect;
use crate::expression::{AppearsOnTable, Expression, SelectableExpression, ValidGrouping};
use crate::helper_types::exists;
use crate::query_builder::*;
use crate::result::QueryResult;
use crate::sql_types::Bool;
pub fn exists<T>(query: T) -> exists<T> {
    loop {}
}
#[derive(Clone, Copy, QueryId, Debug)]
#[non_exhaustive]
pub struct Exists<T> {
    pub subselect: Subselect<T, Bool>,
}
impl<T> Expression for Exists<T>
where
    Subselect<T, Bool>: Expression,
{
    type SqlType = Bool;
}
impl<T, GB> ValidGrouping<GB> for Exists<T>
where
    Subselect<T, Bool>: ValidGrouping<GB>,
{
    type IsAggregate = <Subselect<T, Bool> as ValidGrouping<GB>>::IsAggregate;
}
impl<T, DB> QueryFragment<DB> for Exists<T>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::ExistsSyntax>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, DB> QueryFragment<DB, sql_dialect::exists_syntax::AnsiSqlExistsSyntax> for Exists<T>
where
    DB: Backend + SqlDialect<ExistsSyntax = sql_dialect::exists_syntax::AnsiSqlExistsSyntax>,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, QS> SelectableExpression<QS> for Exists<T>
where
    Self: AppearsOnTable<QS>,
    Subselect<T, Bool>: SelectableExpression<QS>,
{
}
impl<T, QS> AppearsOnTable<QS> for Exists<T>
where
    Self: Expression,
    Subselect<T, Bool>: AppearsOnTable<QS>,
{
}
