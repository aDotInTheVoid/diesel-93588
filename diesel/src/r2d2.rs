//! Connection pooling via r2d2.
//!
//! Note: This module requires enabling the `r2d2` feature
//!
//! # Example
//!
//! The below snippet is a contrived example emulating a web application,
//! where one would first initialize the pool in the `main()` function
//! (at the start of a long-running process). One would then pass this
//! pool struct around as shared state, which, here, we've emulated using
//! threads instead of routes.
//!
//! ```rust
//! # include!("doctest_setup.rs");
//! use diesel::prelude::*;
//! use diesel::r2d2::ConnectionManager;
//! # use diesel::r2d2::CustomizeConnection;
//! # use diesel::r2d2::Error as R2D2Error;
//! use diesel::r2d2::Pool;
//! use diesel::result::Error;
//! use std::thread;
//!
//! # #[derive(Copy, Clone, Debug)]
//! # pub struct SetupUserTableCustomizer;
//! #
//! # impl CustomizeConnection<DbConnection, R2D2Error> for SetupUserTableCustomizer
//! # {
//! #     fn on_acquire(&self, conn: &mut DbConnection) -> Result<(), R2D2Error> {
//! #         setup_database(conn);
//! #         Ok(())
//! #     }
//! # }
//!
//! pub fn get_connection_pool() -> Pool<ConnectionManager<DbConnection>> {
//!     let url = database_url_for_env();
//!     let manager = ConnectionManager::<DbConnection>::new(url);
//!     // Refer to the `r2d2` documentation for more methods to use
//!     // when building a connection pool
//!     Pool::builder()
//! #         .max_size(1)
//!         .test_on_check_out(true)
//! #         .connection_customizer(Box::new(SetupUserTableCustomizer))
//!         .build(manager)
//!         .expect("Could not build connection pool")
//! }
//!
//! pub fn create_user(conn: &mut DbConnection, user_name: &str) -> Result<usize, Error> {
//!     use schema::users::dsl::*;
//!
//!     diesel::insert_into(users)
//!         .values(name.eq(user_name))
//!         .execute(conn)
//! }
//!
//! fn main() {
//!     let pool = get_connection_pool();
//!     let mut threads = vec![];
//!     let max_users_to_create = 1;
//!
//!     for i in 0..max_users_to_create {
//!         let pool = pool.clone();
//!         threads.push(thread::spawn({
//!             move || {
//!                 let conn = &mut pool.get().unwrap();
//!                 let name = format!("Person {}", i);
//!                 create_user(conn, &name).unwrap();
//!             }
//!         }))
//!     }
//!
//!     for handle in threads {
//!         handle.join().unwrap();
//!     }
//! }
//! ```
//!
//! # A note on error handling
//!
//! When used inside a pool, if an individual connection becomes
//! broken (as determined by the [R2D2Connection::is_broken] method)
//! then, when the connection goes out of scope, `r2d2` will close
//! and return the connection to the DB.
//!
//! `diesel` determines broken connections by whether or not the current
//! thread is panicking or if individual `Connection` structs are
//! broken (determined by the `is_broken()` method). Generically, these
//! are left to individual backends to implement themselves.
//!
//! For SQLite, PG, and MySQL backends `is_broken()` is determined
//! by whether or not the `TransactionManagerStatus` (as a part
//! of the `AnsiTransactionManager` struct) is in an `InError` state
//! or contains an open transaction when the connection goes out of scope.
//!
pub use r2d2::*;
/// A re-export of [`r2d2::Error`], which is only used by methods on [`r2d2::Pool`].
///
/// [`r2d2::Error`]: r2d2::Error
/// [`r2d2::Pool`]: r2d2::Pool
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
/// An r2d2 connection manager for use with Diesel.
///
/// See the [r2d2 documentation] for usage examples.
///
/// [r2d2 documentation]: r2d2
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
    /// Returns a new connection manager,
    /// which establishes connections to the given database URL.
    pub fn new<S: Into<String>>(database_url: S) -> Self {
        loop {}
    }
}
/// The error used when managing connections with `r2d2`.
#[derive(Debug)]
pub enum Error {
    /// An error occurred establishing the connection
    ConnectionError(ConnectionError),
    /// An error occurred pinging the database
    QueryError(crate::result::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        loop {}
    }
}
impl ::std::error::Error for Error {}
/// A trait indicating a connection could be used inside a r2d2 pool
pub trait R2D2Connection: Connection {
    /// Check if a connection is still valid
    fn ping(&mut self) -> QueryResult<()>;
    /// Checks if the connection is broken and should not be reused
    ///
    /// This method should return only contain a fast non-blocking check
    /// if the connection is considered to be broken or not. See
    /// [ManageConnection::has_broken] for details.
    ///
    /// The default implementation does not consider any connection as broken
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
