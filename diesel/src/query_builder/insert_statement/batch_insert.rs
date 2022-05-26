use super::ValuesClause;
use crate::backend::{sql_dialect, Backend, SqlDialect};
use crate::insertable::CanInsertInSingleQuery;
use crate::query_builder::{AstPass, QueryFragment, QueryId};
use crate::QueryResult;
use std::marker::PhantomData;
/// This type represents a batch insert clause, which allows
/// to insert multiple rows at once.
///
/// Custom backends can specialize the [`QueryFragment`]
/// implementation via [`SqlDialect::BatchInsertSupport`]
/// or provide fully custom [`ExecuteDsl`](crate::query_dsl::methods::ExecuteDsl)
/// and [`LoadQuery`](crate::query_dsl::methods::LoadQuery) implementations
#[cfg_attr(
    feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes",
    cfg(feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")
)]
#[derive(Debug)]
pub struct BatchInsert<V, Tab, QId, const STABLE_QUERY_ID: bool> {
    /// List of values that should be inserted
    pub values: V,
    _marker: PhantomData<(QId, Tab)>,
}
impl<
    V,
    Tab,
    QId,
    const STABLE_QUERY_ID: bool,
> BatchInsert<V, Tab, QId, STABLE_QUERY_ID> {
    pub(crate) fn new(values: V) -> Self {
        loop {}
    }
}
impl<V, QId: 'static, Tab: 'static, const STABLE_QUERY_ID: bool> QueryId
for BatchInsert<V, Tab, QId, STABLE_QUERY_ID> {
    type QueryId = QId;
    const HAS_STATIC_QUERY_ID: bool = STABLE_QUERY_ID;
}
impl<T, Table, QId, DB, const HAS_STATIC_QUERY_ID: bool> CanInsertInSingleQuery<DB>
for BatchInsert<T, Table, QId, HAS_STATIC_QUERY_ID>
where
    T: CanInsertInSingleQuery<DB>,
    DB: Backend
        + SqlDialect<
            InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword,
        >,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<T, DB, const N: usize> CanInsertInSingleQuery<DB> for [T; N]
where
    DB: Backend
        + SqlDialect<
            InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword,
        >,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<T, DB, const N: usize> CanInsertInSingleQuery<DB> for Box<[T; N]>
where
    DB: Backend
        + SqlDialect<
            InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword,
        >,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<T, DB> CanInsertInSingleQuery<DB> for [T]
where
    DB: Backend
        + SqlDialect<
            InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword,
        >,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<T, DB> CanInsertInSingleQuery<DB> for Vec<T>
where
    DB: Backend
        + SqlDialect<
            InsertWithDefaultKeyword = sql_dialect::default_keyword_for_insert::IsoSqlDefaultKeyword,
        >,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<Tab, DB, V, QId, const HAS_STATIC_QUERY_ID: bool> QueryFragment<DB>
for BatchInsert<V, Tab, QId, HAS_STATIC_QUERY_ID>
where
    DB: Backend,
    Self: QueryFragment<DB, DB::BatchInsertSupport>,
{
    fn walk_ast<'b>(&'b self, pass: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<
    Tab,
    DB,
    V,
    QId,
    const HAS_STATIC_QUERY_ID: bool,
> QueryFragment<DB, sql_dialect::batch_insert_support::PostgresLikeBatchInsertSupport>
for BatchInsert<Vec<ValuesClause<V, Tab>>, Tab, QId, HAS_STATIC_QUERY_ID>
where
    DB: Backend
        + SqlDialect<
            BatchInsertSupport = sql_dialect::batch_insert_support::PostgresLikeBatchInsertSupport,
        >,
    DB::InsertWithDefaultKeyword: sql_dialect::default_keyword_for_insert::SupportsDefaultKeyword,
    ValuesClause<V, Tab>: QueryFragment<DB>,
    V: QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
