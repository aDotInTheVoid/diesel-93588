pub use r2d2::*;
pub type PoolError = r2d2::Error;
use std::convert::Into;
use std::fmt;
use std::marker::PhantomData;
use crate::backend::Backend;
use crate::connection::commit_error_processor::{
    CommitErrorOutcome, CommitErrorProcessor,
};
use crate::connection::{
    ConnectionGatWorkaround, SimpleConnection, TransactionManager,
    TransactionManagerStatus,
};
use crate::expression::QueryMetadata;
use crate::prelude::*;
use crate::query_builder::{Query, QueryFragment, QueryId};
#[derive(Clone)]
pub struct ConnectionManager<T> {
    database_url: String,
    _marker: PhantomData<T>,
}
impl<T> fmt::Debug for ConnectionManager<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
unsafe impl<T: Send + 'static> Sync for ConnectionManager<T> {}
impl<T> ConnectionManager<T> {
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        loop {}
    }
}
#[derive(Debug)]
pub enum Error {
    ConnectionError(ConnectionError),
    QueryError(crate::result::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl ::std::error::Error for Error {}
pub trait R2D2Connection: Connection {
    fn ping(&mut self) -> QueryResult<()>;
    fn is_broken(&mut self) -> bool {
        false
    }
}
impl<T> ManageConnection for ConnectionManager<T>
where
    T: R2D2Connection + Send + 'static,
{
    type Connection = T;
    type Error = Error;
    fn connect(&self) -> Result<T, Error> {
        loop {}
    }
    fn is_valid(&self, conn: &mut T) -> Result<(), Error> {
        loop {}
    }
    fn has_broken(&self, conn: &mut T) -> bool {
        loop {}
    }
}
impl<M> SimpleConnection for PooledConnection<M>
where
    M: ManageConnection,
    M::Connection: R2D2Connection + Send + 'static,
{
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        loop {}
    }
}
impl<'conn, 'query, DB, M> ConnectionGatWorkaround<'conn, 'query, DB>
for PooledConnection<M>
where
    M: ManageConnection,
    M::Connection: Connection<Backend = DB>,
    DB: Backend,
{
    type Cursor = <M::Connection as ConnectionGatWorkaround<'conn, 'query, DB>>::Cursor;
    type Row = <M::Connection as ConnectionGatWorkaround<'conn, 'query, DB>>::Row;
}
impl<M> CommitErrorProcessor for PooledConnection<M>
where
    M: ManageConnection,
    M::Connection: R2D2Connection + CommitErrorProcessor + Send + 'static,
{
    fn process_commit_error(&self, error: crate::result::Error) -> CommitErrorOutcome {
        loop {}
    }
}
impl<M> Connection for PooledConnection<M>
where
    M: ManageConnection,
    M::Connection: Connection + R2D2Connection + Send + 'static,
{
    type Backend = <M::Connection as Connection>::Backend;
    type TransactionManager = PoolTransactionManager<
        <M::Connection as Connection>::TransactionManager,
    >;
    fn establish(_: &str) -> ConnectionResult<Self> {
        loop {}
    }
    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> QueryResult<
        <Self as ConnectionGatWorkaround<'conn, 'query, Self::Backend>>::Cursor,
    >
    where
        T: Query + QueryFragment<Self::Backend> + QueryId + 'query,
        Self::Backend: QueryMetadata<T::SqlType>,
    {
        loop {}
    }
    fn execute_returning_count<T>(&mut self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId,
    {
        loop {}
    }
    fn transaction_state(
        &mut self,
    ) -> &mut <Self::TransactionManager as TransactionManager<
        Self,
    >>::TransactionStateData {
        loop {}
    }
    fn begin_test_transaction(&mut self) -> QueryResult<()> {
        loop {}
    }
}
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct PoolTransactionManager<T>(std::marker::PhantomData<T>);
impl<M, T> TransactionManager<PooledConnection<M>> for PoolTransactionManager<T>
where
    M: ManageConnection,
    M::Connection: Connection<TransactionManager = T> + R2D2Connection,
    T: TransactionManager<M::Connection>,
{
    type TransactionStateData = T::TransactionStateData;
    fn begin_transaction(conn: &mut PooledConnection<M>) -> QueryResult<()> {
        loop {}
    }
    fn rollback_transaction(conn: &mut PooledConnection<M>) -> QueryResult<()> {
        loop {}
    }
    fn commit_transaction(conn: &mut PooledConnection<M>) -> QueryResult<()> {
        loop {}
    }
    fn transaction_manager_status_mut(
        conn: &mut PooledConnection<M>,
    ) -> &mut TransactionManagerStatus {
        loop {}
    }
}
impl<M> crate::migration::MigrationConnection for PooledConnection<M>
where
    M: ManageConnection,
    M::Connection: crate::migration::MigrationConnection,
    Self: Connection,
{
    fn setup(&mut self) -> QueryResult<usize> {
        loop {}
    }
}
impl<Changes, Output, M> crate::query_dsl::UpdateAndFetchResults<Changes, Output>
for PooledConnection<M>
where
    M: ManageConnection,
    M::Connection: crate::query_dsl::UpdateAndFetchResults<Changes, Output>,
    Self: Connection,
{
    fn update_and_fetch(&mut self, changeset: Changes) -> QueryResult<Output> {
        loop {}
    }
}
#[derive(QueryId)]
pub(crate) struct CheckConnectionQuery;
impl<DB> QueryFragment<DB> for CheckConnectionQuery
where
    DB: Backend,
{
    fn walk_ast<'b>(
        &'b self,
        mut pass: crate::query_builder::AstPass<'_, 'b, DB>,
    ) -> QueryResult<()> {
        loop {}
    }
}
impl Query for CheckConnectionQuery {
    type SqlType = crate::sql_types::Integer;
}
impl<C> RunQueryDsl<C> for CheckConnectionQuery {}
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::thread;
    use crate::r2d2::*;
    use crate::test_helpers::*;
    #[test]
    fn establish_basic_connection() {
        loop {}
    }
    #[test]
    fn is_valid() {
        loop {}
    }
    #[test]
    fn pooled_connection_impls_connection() {
        loop {}
    }
    #[test]
    fn check_pool_does_actually_hold_connections() {
        loop {}
    }
}
