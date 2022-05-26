use crate::connection::commit_error_processor::{
    CommitErrorOutcome, CommitErrorProcessor,
};
use crate::connection::Connection;
use crate::result::{DatabaseErrorKind, Error, QueryResult};
use std::borrow::Cow;
use std::num::NonZeroU32;
/// Manages the internal transaction state for a connection.
///
/// You will not need to interact with this trait, unless you are writing an
/// implementation of [`Connection`].
pub trait TransactionManager<Conn: Connection> {
    /// Data stored as part of the connection implementation
    /// to track the current transaction state of a connection
    type TransactionStateData;
    /// Begin a new transaction or savepoint
    ///
    /// If the transaction depth is greater than 0,
    /// this should create a savepoint instead.
    /// This function is expected to increment the transaction depth by 1.
    fn begin_transaction(conn: &mut Conn) -> QueryResult<()>;
    /// Rollback the inner-most transaction or savepoint
    ///
    /// If the transaction depth is greater than 1,
    /// this should rollback to the most recent savepoint.
    /// This function is expected to decrement the transaction depth by 1.
    fn rollback_transaction(conn: &mut Conn) -> QueryResult<()>;
    /// Commit the inner-most transaction or savepoint
    ///
    /// If the transaction depth is greater than 1,
    /// this should release the most recent savepoint.
    /// This function is expected to decrement the transaction depth by 1.
    fn commit_transaction(conn: &mut Conn) -> QueryResult<()>;
    /// Fetch the current transaction status as mutable
    ///
    /// Used to ensure that `begin_test_transaction` is not called when already
    /// inside of a transaction, and that operations are not run in a `InError`
    /// transaction manager.
    fn transaction_manager_status_mut(conn: &mut Conn) -> &mut TransactionManagerStatus;
    /// Executes the given function inside of a database transaction
    ///
    /// Each implementation of this function needs to fullfill the documented
    /// behaviour of [`Connection::transaction`]
    fn transaction<F, R, E>(conn: &mut Conn, callback: F) -> Result<R, E>
    where
        F: FnOnce(&mut Conn) -> Result<R, E>,
        E: From<Error>,
    {
        Self::begin_transaction(conn)?;
        match callback(&mut *conn) {
            Ok(value) => {
                Self::commit_transaction(conn)?;
                Ok(value)
            }
            Err(e) => {
                Self::rollback_transaction(conn)
                    .map_err(|e| Error::RollbackError(Box::new(e)))?;
                Err(e)
            }
        }
    }
}
/// An implementation of `TransactionManager` which can be used for backends
/// which use ANSI standard syntax for savepoints such as SQLite and PostgreSQL.
#[allow(missing_debug_implementations, missing_copy_implementations)]
#[derive(Default)]
pub struct AnsiTransactionManager {
    pub(crate) status: TransactionManagerStatus,
}
/// Status of the transaction manager
#[derive(Debug)]
pub enum TransactionManagerStatus {
    /// Valid status, the manager can run operations
    Valid(ValidTransactionManagerStatus),
    /// Error status, probably following a broken connection. The manager will no longer run operations
    InError,
}
impl Default for TransactionManagerStatus {
    fn default() -> Self {
        loop {}
    }
}
impl TransactionManagerStatus {
    /// Returns the transaction depth if the transaction manager's status is valid, or returns
    /// [`Error::BrokenTransaction`] if the transaction manager is in error.
    pub fn transaction_depth(&self) -> QueryResult<Option<NonZeroU32>> {
        loop {}
    }
    fn transaction_state(&mut self) -> QueryResult<&mut ValidTransactionManagerStatus> {
        loop {}
    }
}
/// Valid transaction status for the manager. Can return the current transaction depth
#[allow(missing_copy_implementations)]
#[derive(Debug, Default)]
pub struct ValidTransactionManagerStatus {
    pub(super) transaction_depth: Option<NonZeroU32>,
    pub(crate) previous_error_relevant_for_rollback: Option<(DatabaseErrorKind, String)>,
}
impl ValidTransactionManagerStatus {
    /// Return the current transaction depth
    ///
    /// This value is `None` if no current transaction is running
    /// otherwise the number of nested transactions is returned.
    pub fn transaction_depth(&self) -> Option<NonZeroU32> {
        loop {}
    }
    /// Update the transaction depth by adding the value of the `transaction_depth_change` parameter if the `query` is
    /// `Ok(())`
    pub fn change_transaction_depth(
        &mut self,
        transaction_depth_change: TransactionDepthChange,
        query: QueryResult<()>,
    ) -> QueryResult<()> {
        loop {}
    }
}
/// Represents a change to apply to the depth of a transaction
#[derive(Debug, Clone, Copy)]
pub enum TransactionDepthChange {
    /// Increase the depth of the transaction (corresponds to `BEGIN` or `SAVEPOINT`)
    IncreaseDepth,
    /// Decreases the depth of the transaction (corresponds to `COMMIT`/`RELEASE SAVEPOINT` or `ROLLBACK`)
    DecreaseDepth,
}
impl AnsiTransactionManager {
    fn get_transaction_state<Conn>(
        conn: &mut Conn,
    ) -> QueryResult<&mut ValidTransactionManagerStatus>
    where
        Conn: Connection<TransactionManager = Self> + CommitErrorProcessor,
    {
        loop {}
    }
    /// Begin a transaction with custom SQL
    ///
    /// This is used by connections to implement more complex transaction APIs
    /// to set things such as isolation levels.
    /// Returns an error if already inside of a transaction.
    pub fn begin_transaction_sql<Conn>(conn: &mut Conn, sql: &str) -> QueryResult<()>
    where
        Conn: Connection<TransactionManager = Self> + CommitErrorProcessor,
    {
        loop {}
    }
}
impl<Conn> TransactionManager<Conn> for AnsiTransactionManager
where
    Conn: Connection<TransactionManager = Self> + CommitErrorProcessor,
{
    type TransactionStateData = Self;
    fn begin_transaction(conn: &mut Conn) -> QueryResult<()> {
        loop {}
    }
    fn rollback_transaction(conn: &mut Conn) -> QueryResult<()> {
        loop {}
    }
    /// If the transaction fails to commit due to a `SerializationFailure` or a
    /// `ReadOnlyTransaction` a rollback will be attempted. If the rollback succeeds,
    /// the original error will be returned, otherwise the error generated by the rollback
    /// will be returned. In the second case the connection should be considered broken
    /// as it contains a uncommitted unabortable open transaction.
    fn commit_transaction(conn: &mut Conn) -> QueryResult<()> {
        loop {}
    }
    fn transaction_manager_status_mut(conn: &mut Conn) -> &mut TransactionManagerStatus {
        loop {}
    }
    fn transaction<F, R, E>(conn: &mut Conn, f: F) -> Result<R, E>
    where
        F: FnOnce(&mut Conn) -> Result<R, E>,
        E: From<Error>,
    {
        loop {}
    }
}
fn process_commit_error<Conn>(
    conn: &mut Conn,
    error: Error,
    rollback_sql: Cow<'_, str>,
) -> QueryResult<()>
where
    Conn: Connection<TransactionManager = AnsiTransactionManager> + CommitErrorProcessor,
{
    loop {}
}
#[cfg(test)]
mod test {
    mod mock {
        use crate::connection::commit_error_processor::{
            CommitErrorOutcome, CommitErrorProcessor,
        };
        use crate::connection::transaction_manager::AnsiTransactionManager;
        use crate::connection::{
            Connection, ConnectionGatWorkaround, SimpleConnection, TransactionManager,
        };
        use crate::expression::QueryMetadata;
        use crate::query_builder::{AsQuery, QueryFragment, QueryId};
        use crate::result::{Error, QueryResult};
        use crate::test_helpers::TestConnection;
        pub(crate) struct MockConnection {
            pub(crate) next_result: Option<QueryResult<usize>>,
            pub(crate) next_batch_execute_result: Option<QueryResult<()>>,
            pub(crate) broken: bool,
            transaction_state: AnsiTransactionManager,
        }
        impl SimpleConnection for MockConnection {
            fn batch_execute(&mut self, _query: &str) -> QueryResult<()> {
                loop {}
            }
        }
        impl<
            'conn,
            'query,
        > ConnectionGatWorkaround<'conn, 'query, <TestConnection as Connection>::Backend>
        for MockConnection {
            type Cursor = <TestConnection as ConnectionGatWorkaround<
                'conn,
                'query,
                <TestConnection as Connection>::Backend,
            >>::Cursor;
            type Row = <TestConnection as ConnectionGatWorkaround<
                'conn,
                'query,
                <TestConnection as Connection>::Backend,
            >>::Row;
        }
        impl CommitErrorProcessor for MockConnection {
            fn process_commit_error(&self, error: Error) -> CommitErrorOutcome {
                loop {}
            }
        }
        impl Connection for MockConnection {
            type Backend = <TestConnection as Connection>::Backend;
            type TransactionManager = AnsiTransactionManager;
            fn establish(_database_url: &str) -> crate::ConnectionResult<Self> {
                loop {}
            }
            fn load<'conn, 'query, T>(
                &'conn mut self,
                _source: T,
            ) -> QueryResult<
                    <Self as ConnectionGatWorkaround<
                        'conn,
                        'query,
                        Self::Backend,
                    >>::Cursor,
                >
            where
                T: AsQuery,
                T::Query: QueryFragment<Self::Backend> + QueryId + 'query,
                Self::Backend: QueryMetadata<T::SqlType>,
            {
                loop {}
            }
            fn execute_returning_count<T>(&mut self, _source: &T) -> QueryResult<usize>
            where
                T: crate::query_builder::QueryFragment<Self::Backend>
                    + crate::query_builder::QueryId,
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
        }
    }
    #[test]
    #[cfg(feature = "postgres")]
    fn transaction_manager_returns_an_error_when_attempting_to_commit_outside_of_a_transaction() {
        loop {}
    }
    #[test]
    #[cfg(feature = "postgres")]
    fn transaction_manager_returns_an_error_when_attempting_to_rollback_outside_of_a_transaction() {
        loop {}
    }
    #[test]
    fn transaction_manager_enters_broken_state_when_connection_is_broken() {
        loop {}
    }
    #[test]
    #[cfg(feature = "mysql")]
    fn mysql_transaction_is_rolled_back_upon_syntax_error() {
        loop {}
    }
    #[test]
    #[cfg(feature = "sqlite")]
    fn sqlite_transaction_is_rolled_back_upon_syntax_error() {
        loop {}
    }
    #[test]
    #[cfg(feature = "mysql")]
    fn nested_mysql_transaction_is_rolled_back_upon_syntax_error() {
        loop {}
    }
    #[test]
    #[cfg(feature = "mysql")]
    #[allow(clippy::needless_collect)]
    fn mysql_transaction_depth_commits_tracked_properly_on_serialization_failure() {
        loop {}
    }
    #[test]
    #[cfg(feature = "mysql")]
    #[allow(clippy::needless_collect)]
    fn mysql_nested_transaction_depth_commits_tracked_properly_on_serialization_failure() {
        loop {}
    }
    #[test]
    #[cfg(feature = "sqlite")]
    fn sqlite_transaction_is_rolled_back_upon_deferred_constraint_failure() {
        loop {}
    }
}
