use super::from_clause::AsQuerySource;
use super::*;
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::expression::grouped::Grouped;
use crate::expression::operators::{And, Or};
use crate::expression::*;
use crate::result::QueryResult;
use crate::sql_types::BoolOrNullableBool;
/// Add `Predicate` to the current `WHERE` clause, joining with `AND` if
/// applicable.
pub trait WhereAnd<Predicate> {
    /// What is the type of the resulting `WHERE` clause?
    type Output;
    /// See the trait-level docs.
    fn and(self, predicate: Predicate) -> Self::Output;
}
/// Add `Predicate` to the current `WHERE` clause, joining with `OR` if
/// applicable.
pub trait WhereOr<Predicate> {
    /// What is the type of the resulting `WHERE` clause?
    type Output;
    /// See the trait-level docs.
    fn or(self, predicate: Predicate) -> Self::Output;
}
/// Represents that a query has no `WHERE` clause.
#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoWhereClause;
impl<DB> QueryFragment<DB> for NoWhereClause
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, _: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Predicate> WhereAnd<Predicate> for NoWhereClause
where
    Predicate: Expression,
    Predicate::SqlType: BoolOrNullableBool,
{
    type Output = WhereClause<Predicate>;
    fn and(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<Predicate> WhereOr<Predicate> for NoWhereClause
where
    Predicate: Expression,
    Predicate::SqlType: BoolOrNullableBool,
{
    type Output = WhereClause<Predicate>;
    fn or(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<'a, DB> From<NoWhereClause> for BoxedWhereClause<'a, DB> {
    fn from(_: NoWhereClause) -> Self {
        loop {}
    }
}
/// The `WHERE` clause of a query.
#[derive(Debug, Clone, Copy, QueryId)]
pub struct WhereClause<Expr>(Expr);
impl<DB, Expr> QueryFragment<DB> for WhereClause<Expr>
where
    DB: Backend + DieselReserveSpecialization,
    Expr: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Expr, Predicate> WhereAnd<Predicate> for WhereClause<Expr>
where
    Expr: Expression,
    Expr::SqlType: BoolOrNullableBool,
    Predicate: Expression,
    Predicate::SqlType: BoolOrNullableBool,
{
    type Output = WhereClause<Grouped<And<Expr, Predicate>>>;
    fn and(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<Expr, Predicate> WhereOr<Predicate> for WhereClause<Expr>
where
    Expr: Expression,
    Expr::SqlType: BoolOrNullableBool,
    Predicate: Expression,
    Predicate::SqlType: BoolOrNullableBool,
{
    type Output = WhereClause<Grouped<Or<Expr, Predicate>>>;
    fn or(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<'a, DB, Predicate> From<WhereClause<Predicate>> for BoxedWhereClause<'a, DB>
where
    DB: Backend,
    Predicate: QueryFragment<DB> + Send + 'a,
{
    fn from(where_clause: WhereClause<Predicate>) -> Self {
        loop {}
    }
}
/// Marker trait indicating that a `WHERE` clause is valid for a given query
/// source.
pub trait ValidWhereClause<QS> {}
impl<QS> ValidWhereClause<QS> for NoWhereClause {}
impl<QS, Expr> ValidWhereClause<QS> for WhereClause<Expr>
where
    Expr: AppearsOnTable<QS::QuerySource>,
    QS: AsQuerySource,
{}
impl<Expr> ValidWhereClause<NoFromClause> for WhereClause<Expr>
where
    Expr: AppearsOnTable<NoFromClause>,
{}
#[allow(missing_debug_implementations)]
pub enum BoxedWhereClause<'a, DB> {
    Where(Box<dyn QueryFragment<DB> + Send + 'a>),
    None,
}
impl<'a, DB> QueryFragment<DB> for BoxedWhereClause<'a, DB>
where
    DB: Backend + DieselReserveSpecialization,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<'a, DB> QueryId for BoxedWhereClause<'a, DB> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<'a, DB, Predicate> WhereAnd<Predicate> for BoxedWhereClause<'a, DB>
where
    DB: Backend + 'a,
    Predicate: QueryFragment<DB> + Send + 'a,
    Grouped<And<Box<dyn QueryFragment<DB> + Send + 'a>, Predicate>>: QueryFragment<DB>,
{
    type Output = Self;
    fn and(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
impl<'a, DB, Predicate> WhereOr<Predicate> for BoxedWhereClause<'a, DB>
where
    DB: Backend + 'a,
    Predicate: QueryFragment<DB> + Send + 'a,
    Grouped<Or<Box<dyn QueryFragment<DB> + Send + 'a>, Predicate>>: QueryFragment<DB>,
{
    type Output = Self;
    fn or(self, predicate: Predicate) -> Self::Output {
        loop {}
    }
}
