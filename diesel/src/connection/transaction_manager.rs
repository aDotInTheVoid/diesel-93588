use crate::connection::commit_error_processor::{
    CommitErrorOutcome, CommitErrorProcessor,
};
use crate::connection::Connection;
use crate::result::{DatabaseErrorKind, Error, QueryResult};
use std::borrow::Cow;
use std::num::NonZeroU32;
pub trait TransactionManager<Conn: Connection> {
    type TransactionStateData;
    fn begin_transaction(conn: &mut Conn) -> QueryResult<()>;
    fn rollback_transaction(conn: &mut Conn) -> QueryResult<()>;
    fn commit_transaction(conn: &mut Conn) -> QueryResult<()>;
    fn transaction_manager_status_mut(conn: &mut Conn) -> &mut TransactionManagerStatus;
    fn transaction<F, R, E>(conn: &mut Conn, callback: F) -> Result<R, E>
    where
        F: FnOnce(&mut Conn) -> Result<R, E>,
        E: From<Error>,
    {
        loop {}
    }
}
#[allow(missing_debug_implementations, missing_copy_implementations)]
#[derive(Default)]
pub struct AnsiTransactionManager {
    pub(crate) status: TransactionManagerStatus,
}
#[derive(Debug)]
pub enum TransactionManagerStatus {
    Valid(ValidTransactionManagerStatus),
    InError,
}
impl Default for TransactionManagerStatus {
    fn default() -> Self {
        loop {}
    }
}
impl TransactionManagerStatus {
    pub fn transaction_depth(&self) -> QueryResult<Option<NonZeroU32>> {
        loop {}
    }
    fn transaction_state(&mut self) -> QueryResult<&mut ValidTransactionManagerStatus> {
        loop {}
    }
}
#[allow(missing_copy_implementations)]
#[derive(Debug, Default)]
pub struct ValidTransactionManagerStatus {
    pub(super) transaction_depth: Option<NonZeroU32>,
    pub(crate) previous_error_relevant_for_rollback: Option<(DatabaseErrorKind, String)>,
}
impl ValidTransactionManagerStatus {
    pub fn transaction_depth(&self) -> Option<NonZeroU32> {
        loop {}
    }
    pub fn change_transaction_depth(
        &mut self,
        transaction_depth_change: TransactionDepthChange,
        query: QueryResult<()>,
    ) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TransactionDepthChange {
    IncreaseDepth,
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
                <Self as ConnectionGatWorkaround<'conn, 'query, Self::Backend>>::Cursor,
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
