use crate::backend::{sql_dialect, Backend};
use crate::expression::{AppearsOnTable, Expression};
use crate::query_builder::*;
use crate::query_source::*;
use crate::result::QueryResult;
#[doc(hidden)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct DoNothing;
impl<DB> QueryFragment<DB> for DoNothing
where
    DB: Backend,
    Self: QueryFragment<DB, DB::OnConflictClause>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB, T> QueryFragment<DB, T> for DoNothing
where
    DB: Backend,
    T: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[doc(hidden)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct DoUpdate<T> {
    changeset: T,
}
impl<T> DoUpdate<T> {
    pub(crate) fn new(changeset: T) -> Self {
        loop {}
    }
}
impl<DB, T> QueryFragment<DB> for DoUpdate<T>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::OnConflictClause>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB, T, SP> QueryFragment<DB, SP> for DoUpdate<T>
where
    DB: Backend,
    SP: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
#[doc(hidden)]
#[derive(Debug, Clone, Copy, QueryId)]
pub struct Excluded<T>(T);
impl<T> Excluded<T> {
    pub(crate) fn new(t: T) -> Self {
        loop {}
    }
}
impl<DB, T> QueryFragment<DB> for Excluded<T>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::OnConflictClause>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB, T, SP> QueryFragment<DB, SP> for Excluded<T>
where
    DB: Backend,
    SP: sql_dialect::on_conflict_clause::SupportsOnConflictClause,
    T: Column,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T> Expression for Excluded<T>
where
    T: Expression,
{
    type SqlType = T::SqlType;
}
impl<T> AppearsOnTable<T::Table> for Excluded<T>
where
    T: Column,
    Excluded<T>: Expression,
{}
