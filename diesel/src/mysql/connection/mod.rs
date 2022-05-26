mod bind;
mod raw;
mod stmt;
mod url;
use self::raw::RawConnection;
use self::stmt::iterator::StatementIterator;
use self::stmt::Statement;
use self::url::ConnectionOptions;
use super::backend::Mysql;
use crate::connection::commit_error_processor::{
    default_process_commit_error, CommitErrorOutcome, CommitErrorProcessor,
};
use crate::connection::statement_cache::{MaybeCached, StatementCache};
use crate::connection::*;
use crate::expression::QueryMetadata;
use crate::query_builder::bind_collector::RawBytesBindCollector;
use crate::query_builder::*;
use crate::result::*;
use crate::RunQueryDsl;
#[cfg(feature = "mysql")]
#[allow(missing_debug_implementations, missing_copy_implementations)]
/// A connection to a MySQL database. Connection URLs should be in the form
/// `mysql://[user[:password]@]host/database_name`
pub struct MysqlConnection {
    raw_connection: RawConnection,
    transaction_state: AnsiTransactionManager,
    statement_cache: StatementCache<Mysql, Statement>,
}
unsafe impl Send for MysqlConnection {}
impl SimpleConnection for MysqlConnection {
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        loop {}
    }
}
impl<'conn, 'query> ConnectionGatWorkaround<'conn, 'query, Mysql> for MysqlConnection {
    type Cursor = self::stmt::iterator::StatementIterator<'conn>;
    type Row = self::stmt::iterator::MysqlRow;
}
impl CommitErrorProcessor for MysqlConnection {
    fn process_commit_error(&self, error: Error) -> CommitErrorOutcome {
        loop {}
    }
}
impl Connection for MysqlConnection {
    type Backend = Mysql;
    type TransactionManager = AnsiTransactionManager;
    /// Establishes a new connection to the MySQL database
    /// `database_url` may be enhanced by GET parameters
    /// `mysql://[user[:password]@]host/database_name[?unix_socket=socket-path&ssl_mode=SSL_MODE*&ssl_ca=/etc/ssl/certs/ca-certificates.crt]`
    ///
    /// * `unix_socket` expects the path to the unix socket
    /// * `ssl_ca` accepts a path to the system's certificate roots
    /// * `ssl_mode` expects a value defined for MySQL client command option `--ssl-mode`
    /// See <https://dev.mysql.com/doc/refman/5.7/en/connection-options.html#option_general_ssl-mode>
    fn establish(database_url: &str) -> ConnectionResult<Self> {
        loop {}
    }
    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> QueryResult<LoadRowIter<'conn, 'query, Self, Self::Backend>>
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
    fn transaction_state(&mut self) -> &mut AnsiTransactionManager {
        loop {}
    }
}
#[cfg(feature = "r2d2")]
impl crate::r2d2::R2D2Connection for MysqlConnection {
    fn ping(&mut self) -> QueryResult<()> {
        loop {}
    }
    fn is_broken(&mut self) -> bool {
        loop {}
    }
}
impl MysqlConnection {
    fn prepared_query<'a, T: QueryFragment<Mysql> + QueryId>(
        &'a mut self,
        source: &'_ T,
    ) -> QueryResult<MaybeCached<'a, Statement>> {
        loop {}
    }
    fn set_config_options(&mut self) -> QueryResult<()> {
        loop {}
    }
}
#[cfg(test)]
mod tests {
    extern crate dotenvy;
    use super::*;
    use std::env;
    fn connection() -> MysqlConnection {
        loop {}
    }
    #[test]
    fn batch_execute_handles_single_queries_with_results() {
        loop {}
    }
    #[test]
    fn batch_execute_handles_multi_queries_with_results() {
        loop {}
    }
    #[test]
    fn execute_handles_queries_which_return_results() {
        loop {}
    }
    #[test]
    fn check_client_found_rows_flag() {
        loop {}
    }
}
