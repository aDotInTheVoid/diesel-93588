use std::marker::PhantomData;
use crate::backend::Backend;
use crate::expression::*;
use crate::query_builder::*;
use crate::query_dsl::RunQueryDsl;
use crate::result::QueryResult;
use crate::sql_types::{DieselNumericOps, SqlType};
#[derive(Debug, Clone, DieselNumericOps)]
#[must_use = "Queries are only executed when calling `load`, `get_result`, or similar."]
pub struct SqlLiteral<ST, T = self::private::Empty> {
    sql: String,
    inner: T,
    _marker: PhantomData<ST>,
}
impl<ST, T> SqlLiteral<ST, T>
where
    ST: TypedExpressionType,
{
    pub(crate) fn new(sql: String, inner: T) -> Self {
        loop {}
    }
                                                                                                                                                                                                                                                                                            pub fn bind<BindST, U>(self, bind_value: U) -> UncheckedBind<Self, U::Expression>
    where
        BindST: SqlType + TypedExpressionType,
        U: AsExpression<BindST>,
    {
        loop {}
    }
                                                                                                                                                                            pub fn sql(self, sql: &str) -> SqlLiteral<ST, Self> {
        loop {}
    }
}
impl<ST, T> Expression for SqlLiteral<ST, T>
where
    ST: TypedExpressionType,
{
    type SqlType = ST;
}
impl<ST, T, DB> QueryFragment<DB> for SqlLiteral<ST, T>
where
    DB: Backend,
    T: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<ST, T> QueryId for SqlLiteral<ST, T> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<ST, T> Query for SqlLiteral<ST, T>
where
    Self: Expression,
{
    type SqlType = ST;
}
impl<ST, T, Conn> RunQueryDsl<Conn> for SqlLiteral<ST, T> {}
impl<QS, ST, T> SelectableExpression<QS> for SqlLiteral<ST, T>
where
    Self: Expression,
{}
impl<QS, ST, T> AppearsOnTable<QS> for SqlLiteral<ST, T>
where
    Self: Expression,
{}
impl<ST, T, GB> ValidGrouping<GB> for SqlLiteral<ST, T> {
    type IsAggregate = is_aggregate::Never;
}
pub fn sql<ST>(sql: &str) -> SqlLiteral<ST>
where
    ST: TypedExpressionType,
{
    loop {}
}
#[derive(QueryId, Debug, Clone, Copy)]
#[must_use = "Queries are only executed when calling `load`, `get_result`, or similar."]
pub struct UncheckedBind<Query, Value> {
    query: Query,
    value: Value,
}
impl<Query, Value> UncheckedBind<Query, Value>
where
    Query: Expression,
{
    pub(crate) fn new(query: Query, value: Value) -> Self {
        loop {}
    }
                                                                                                                                                                                pub fn sql(self, sql: &str) -> SqlLiteral<Query::SqlType, Self> {
        loop {}
    }
}
impl<Query, Value> Expression for UncheckedBind<Query, Value>
where
    Query: Expression,
{
    type SqlType = Query::SqlType;
}
impl<Query, Value, DB> QueryFragment<DB> for UncheckedBind<Query, Value>
where
    DB: Backend,
    Query: QueryFragment<DB>,
    Value: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Q, Value> Query for UncheckedBind<Q, Value>
where
    Q: Query,
{
    type SqlType = Q::SqlType;
}
impl<Query, Value, GB> ValidGrouping<GB> for UncheckedBind<Query, Value> {
    type IsAggregate = is_aggregate::Never;
}
impl<QS, Query, Value> SelectableExpression<QS> for UncheckedBind<Query, Value>
where
    Self: AppearsOnTable<QS>,
{}
impl<QS, Query, Value> AppearsOnTable<QS> for UncheckedBind<Query, Value>
where
    Self: Expression,
{}
impl<Query, Value, Conn> RunQueryDsl<Conn> for UncheckedBind<Query, Value> {}
mod private {
    use crate::backend::{Backend, DieselReserveSpecialization};
    use crate::query_builder::{QueryFragment, QueryId};
    #[derive(Debug, Clone, Copy, QueryId)]
    pub struct Empty;
    impl<DB> QueryFragment<DB> for Empty
    where
        DB: Backend + DieselReserveSpecialization,
    {
        fn walk_ast<'b>(
            &'b self,
            _pass: crate::query_builder::AstPass<'_, 'b, DB>,
        ) -> crate::QueryResult<()> {
            loop {}
        }
    }
}
