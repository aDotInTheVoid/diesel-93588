use crate::backend::sql_dialect;
use crate::backend::Backend;
use crate::backend::SqlDialect;
use crate::expression::subselect::Subselect;
use crate::expression::{
    AppearsOnTable, AsExpression, Expression, SelectableExpression, TypedExpressionType,
    ValidGrouping,
};
use crate::query_builder::{
    AstPass, BoxedSelectStatement, QueryFragment, QueryId, SelectQuery, SelectStatement,
};
use crate::result::QueryResult;
use crate::serialize::ToSql;
use crate::sql_types::HasSqlType;
use crate::sql_types::{Bool, SingleValue, SqlType};
use std::marker::PhantomData;
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
#[non_exhaustive]
pub struct In<T, U> {
    pub left: T,
    pub values: U,
}
#[derive(Debug, Copy, Clone, QueryId, ValidGrouping)]
#[non_exhaustive]
pub struct NotIn<T, U> {
    pub left: T,
    pub values: U,
}
impl<T, U> In<T, U> {
    pub(crate) fn new(left: T, values: U) -> Self {
        loop {}
    }
}
impl<T, U> NotIn<T, U> {
    pub(crate) fn new(left: T, values: U) -> Self {
        loop {}
    }
}
impl<T, U> Expression for In<T, U>
where
    T: Expression,
    U: Expression<SqlType = T::SqlType>,
{
    type SqlType = Bool;
}
impl<T, U> Expression for NotIn<T, U>
where
    T: Expression,
    U: Expression<SqlType = T::SqlType>,
{
    type SqlType = Bool;
}
impl<T, U, DB> QueryFragment<DB> for In<T, U>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::ArrayComparision>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, U, DB> QueryFragment<DB, sql_dialect::array_comparision::AnsiSqlArrayComparison>
for In<T, U>
where
    DB: Backend
        + SqlDialect<
            ArrayComparision = sql_dialect::array_comparision::AnsiSqlArrayComparison,
        >,
    T: QueryFragment<DB>,
    U: QueryFragment<DB> + MaybeEmpty,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, U, DB> QueryFragment<DB> for NotIn<T, U>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::ArrayComparision>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<T, U, DB> QueryFragment<DB, sql_dialect::array_comparision::AnsiSqlArrayComparison>
for NotIn<T, U>
where
    DB: Backend
        + SqlDialect<
            ArrayComparision = sql_dialect::array_comparision::AnsiSqlArrayComparison,
        >,
    T: QueryFragment<DB>,
    U: QueryFragment<DB> + MaybeEmpty,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl_selectable_expression!(In < T, U >);
impl_selectable_expression!(NotIn < T, U >);
pub trait AsInExpression<T: SqlType + TypedExpressionType> {
    type InExpression: MaybeEmpty + Expression<SqlType = T>;
    #[allow(clippy::wrong_self_convention)]
    fn as_in_expression(self) -> Self::InExpression;
}
impl<I, T, ST> AsInExpression<ST> for I
where
    I: IntoIterator<Item = T>,
    T: AsExpression<ST>,
    ST: SqlType + TypedExpressionType,
{
    type InExpression = Many<ST, T>;
    fn as_in_expression(self) -> Self::InExpression {
        loop {}
    }
}
pub trait MaybeEmpty {
    fn is_empty(&self) -> bool;
}
impl<ST, F, S, D, W, O, LOf, G, H, LC> AsInExpression<ST>
for SelectStatement<F, S, D, W, O, LOf, G, H, LC>
where
    ST: SqlType + TypedExpressionType,
    Subselect<Self, ST>: Expression<SqlType = ST>,
    Self: SelectQuery<SqlType = ST>,
{
    type InExpression = Subselect<Self, ST>;
    fn as_in_expression(self) -> Self::InExpression {
        loop {}
    }
}
impl<'a, ST, QS, DB, GB> AsInExpression<ST> for BoxedSelectStatement<'a, ST, QS, DB, GB>
where
    ST: SqlType + TypedExpressionType,
    Subselect<BoxedSelectStatement<'a, ST, QS, DB, GB>, ST>: Expression<SqlType = ST>,
{
    type InExpression = Subselect<Self, ST>;
    fn as_in_expression(self) -> Self::InExpression {
        loop {}
    }
}
#[derive(Debug, Clone)]
pub struct Many<ST, I> {
    pub values: Vec<I>,
    p: PhantomData<ST>,
}
impl<ST, I, GB> ValidGrouping<GB> for Many<ST, I>
where
    ST: SingleValue,
    I: AsExpression<ST>,
    I::Expression: ValidGrouping<GB>,
{
    type IsAggregate = <I::Expression as ValidGrouping<GB>>::IsAggregate;
}
impl<ST, I> Expression for Many<ST, I>
where
    ST: TypedExpressionType,
{
    type SqlType = ST;
}
impl<ST, I> MaybeEmpty for Many<ST, I> {
    fn is_empty(&self) -> bool {
        loop {}
    }
}
impl<ST, I, QS> SelectableExpression<QS> for Many<ST, I>
where
    Many<ST, I>: AppearsOnTable<QS>,
    ST: SingleValue,
    I: AsExpression<ST>,
    <I as AsExpression<ST>>::Expression: SelectableExpression<QS>,
{}
impl<ST, I, QS> AppearsOnTable<QS> for Many<ST, I>
where
    Many<ST, I>: Expression,
    I: AsExpression<ST>,
    ST: SingleValue,
    <I as AsExpression<ST>>::Expression: SelectableExpression<QS>,
{}
impl<ST, I, DB> QueryFragment<DB> for Many<ST, I>
where
    Self: QueryFragment<DB, DB::ArrayComparision>,
    DB: Backend,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<ST, I, DB> QueryFragment<DB, sql_dialect::array_comparision::AnsiSqlArrayComparison>
for Many<ST, I>
where
    DB: Backend + HasSqlType<ST>
        + SqlDialect<
            ArrayComparision = sql_dialect::array_comparision::AnsiSqlArrayComparison,
        >,
    ST: SingleValue,
    I: ToSql<ST, DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<ST, I> QueryId for Many<ST, I> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
