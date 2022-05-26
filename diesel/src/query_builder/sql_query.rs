use super::Query;
use crate::backend::{Backend, DieselReserveSpecialization};
use crate::connection::Connection;
use crate::query_builder::{AstPass, QueryFragment, QueryId};
use crate::query_dsl::RunQueryDsl;
use crate::result::QueryResult;
use crate::serialize::ToSql;
use crate::sql_types::{HasSqlType, Untyped};
use std::marker::PhantomData;
#[derive(Debug, Clone)]
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct SqlQuery<Inner = self::private::Empty> {
    inner: Inner,
    query: String,
}
impl<Inner> SqlQuery<Inner> {
    pub(crate) fn new(inner: Inner, query: String) -> Self {
        loop {}
    }
    pub fn bind<ST, Value>(self, value: Value) -> UncheckedBind<Self, Value, ST> {
        loop {}
    }
    pub fn into_boxed<'f, DB: Backend>(self) -> BoxedSqlQuery<'f, DB, Self> {
        loop {}
    }
    pub fn sql<T: AsRef<str>>(mut self, sql: T) -> Self {
        loop {}
    }
}
impl SqlQuery {
    pub(crate) fn from_sql(query: String) -> SqlQuery {
        loop {}
    }
}
impl<DB, Inner> QueryFragment<DB> for SqlQuery<Inner>
where
    DB: Backend + DieselReserveSpecialization,
    Inner: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Inner> QueryId for SqlQuery<Inner> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<Inner> Query for SqlQuery<Inner> {
    type SqlType = Untyped;
}
impl<Inner, Conn> RunQueryDsl<Conn> for SqlQuery<Inner> {}
#[derive(Debug, Clone, Copy)]
#[must_use = "Queries are only executed when calling `load`, `get_result` or similar."]
pub struct UncheckedBind<Query, Value, ST> {
    query: Query,
    value: Value,
    _marker: PhantomData<ST>,
}
impl<Query, Value, ST> UncheckedBind<Query, Value, ST> {
    pub fn new(query: Query, value: Value) -> Self {
        loop {}
    }
    pub fn bind<ST2, Value2>(self, value: Value2) -> UncheckedBind<Self, Value2, ST2> {
        loop {}
    }
    pub fn into_boxed<'f, DB: Backend>(self) -> BoxedSqlQuery<'f, DB, Self> {
        loop {}
    }
    pub fn sql<T: Into<String>>(self, sql: T) -> SqlQuery<Self> {
        loop {}
    }
}
impl<Query, Value, ST> QueryId for UncheckedBind<Query, Value, ST>
where
    Query: QueryId,
    ST: QueryId,
{
    type QueryId = UncheckedBind<Query::QueryId, (), ST::QueryId>;
    const HAS_STATIC_QUERY_ID: bool = Query::HAS_STATIC_QUERY_ID && ST::HAS_STATIC_QUERY_ID;
}
impl<Query, Value, ST, DB> QueryFragment<DB> for UncheckedBind<Query, Value, ST>
where
    DB: Backend + HasSqlType<ST> + DieselReserveSpecialization,
    Query: QueryFragment<DB>,
    Value: ToSql<ST, DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Q, Value, ST> Query for UncheckedBind<Q, Value, ST> {
    type SqlType = Untyped;
}
impl<Conn, Query, Value, ST> RunQueryDsl<Conn> for UncheckedBind<Query, Value, ST> {}
#[must_use = "Queries are only executed when calling `load`, `get_result`, or similar."]
#[allow(missing_debug_implementations)]
pub struct BoxedSqlQuery<'f, DB: Backend, Query> {
    query: Query,
    sql: String,
    binds: Vec<Box<dyn QueryFragment<DB> + 'f>>,
}
struct RawBind<ST, U> {
    value: U,
    p: PhantomData<ST>,
}
impl<ST, U, DB> QueryFragment<DB> for RawBind<ST, U>
where
    DB: Backend + HasSqlType<ST>,
    U: ToSql<ST, DB>,
{
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<'f, DB: Backend, Query> BoxedSqlQuery<'f, DB, Query> {
    pub(crate) fn new(query: Query) -> Self {
        loop {}
    }
    pub fn bind<BindSt, Value>(mut self, b: Value) -> Self
    where
        DB: HasSqlType<BindSt>,
        Value: ToSql<BindSt, DB> + 'f,
        BindSt: 'f,
    {
        loop {}
    }
    pub fn sql<T: AsRef<str>>(mut self, sql: T) -> Self {
        loop {}
    }
}
impl<DB, Query> QueryFragment<DB> for BoxedSqlQuery<'_, DB, Query>
where
    DB: Backend + DieselReserveSpecialization,
    Query: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<DB: Backend, Query> QueryId for BoxedSqlQuery<'_, DB, Query> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
impl<DB, Q> Query for BoxedSqlQuery<'_, DB, Q>
where
    DB: Backend,
{
    type SqlType = Untyped;
}
impl<Conn: Connection, Query> RunQueryDsl<Conn> for BoxedSqlQuery<'_, Conn::Backend, Query> {}
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
