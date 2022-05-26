pub(crate) mod cursor;
mod raw;
mod result;
mod row;
mod stmt;
use std::ffi::CString;
use std::os::raw as libc;
use self::cursor::*;
use self::raw::{PgTransactionStatus, RawConnection};
use self::result::PgResult;
use self::stmt::Statement;
use crate::connection::commit_error_processor::{
    CommitErrorOutcome, CommitErrorProcessor,
};
use crate::connection::statement_cache::{MaybeCached, StatementCache};
use crate::connection::*;
use crate::expression::QueryMetadata;
use crate::pg::metadata_lookup::{GetPgMetadataCache, PgMetadataCache};
use crate::pg::{Pg, TransactionBuilder};
use crate::query_builder::bind_collector::RawBytesBindCollector;
use crate::query_builder::*;
use crate::result::ConnectionError::CouldntSetupConfiguration;
use crate::result::*;
use crate::RunQueryDsl;
/// The connection string expected by `PgConnection::establish`
/// should be a PostgreSQL connection string, as documented at
/// <https://www.postgresql.org/docs/9.4/static/libpq-connect.html#LIBPQ-CONNSTRING>
#[allow(missing_debug_implementations)]
#[cfg(feature = "postgres")]
pub struct PgConnection {
    raw_connection: RawConnection,
    transaction_state: AnsiTransactionManager,
    statement_cache: StatementCache<Pg, Statement>,
    metadata_cache: PgMetadataCache,
}
unsafe impl Send for PgConnection {}
impl SimpleConnection for PgConnection {
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        loop {}
    }
}
impl<'conn, 'query> ConnectionGatWorkaround<'conn, 'query, Pg> for PgConnection {
    type Cursor = Cursor<'conn>;
    type Row = self::row::PgRow;
}
impl CommitErrorProcessor for PgConnection {
    fn process_commit_error(&self, error: Error) -> CommitErrorOutcome {
        loop {}
    }
}
impl Connection for PgConnection {
    type Backend = Pg;
    type TransactionManager = AnsiTransactionManager;
    fn establish(database_url: &str) -> ConnectionResult<PgConnection> {
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
        T: QueryFragment<Pg> + QueryId,
    {
        loop {}
    }
    fn transaction_state(&mut self) -> &mut AnsiTransactionManager
    where
        Self: Sized,
    {
        loop {}
    }
}
impl GetPgMetadataCache for PgConnection {
    fn get_metadata_cache(&mut self) -> &mut PgMetadataCache {
        loop {}
    }
}
#[cfg(feature = "r2d2")]
impl crate::r2d2::R2D2Connection for PgConnection {
    fn ping(&mut self) -> QueryResult<()> {
        loop {}
    }
    fn is_broken(&mut self) -> bool {
        loop {}
    }
}
impl PgConnection {
    /// Build a transaction, specifying additional details such as isolation level
    ///
    /// See [`TransactionBuilder`] for more examples.
    ///
    /// [`TransactionBuilder`]: crate::pg::TransactionBuilder
    ///
    /// ```rust
    /// # include!("../../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .read_only()
    ///     .serializable()
    ///     .deferrable()
    ///     .run(|conn| Ok(()))
    /// # }
    /// ```
    pub fn build_transaction(&mut self) -> TransactionBuilder<'_, Self> {
        loop {}
    }
    fn with_prepared_query<'conn, T: QueryFragment<Pg> + QueryId, R>(
        &'conn mut self,
        source: &'_ T,
        f: impl FnOnce(
            MaybeCached<'_, Statement>,
            Vec<Option<Vec<u8>>>,
            &'conn mut RawConnection,
        ) -> QueryResult<R>,
    ) -> QueryResult<R> {
        loop {}
    }
    fn set_config_options(&mut self) -> QueryResult<()> {
        loop {}
    }
}
extern "C" fn noop_notice_processor(
    _: *mut libc::c_void,
    _message: *const libc::c_char,
) {
    loop {}
}
#[cfg(test)]
mod tests {
    extern crate dotenvy;
    use super::*;
    use crate::dsl::sql;
    use crate::prelude::*;
    use crate::result::Error::DatabaseError;
    use crate::sql_types::{Integer, VarChar};
    use std::num::NonZeroU32;
    #[test]
    fn malformed_sql_query() {
        loop {}
    }
    #[test]
    fn prepared_statements_are_cached() {
        loop {}
    }
    #[test]
    fn queries_with_identical_sql_but_different_types_are_cached_separately() {
        loop {}
    }
    #[test]
    fn queries_with_identical_types_and_sql_but_different_bind_types_are_cached_separately() {
        loop {}
    }
    sql_function!(fn lower(x : VarChar) -> VarChar);
    #[test]
    fn queries_with_identical_types_and_binds_but_different_sql_are_cached_separately() {
        loop {}
    }
    #[test]
    fn queries_with_sql_literal_nodes_are_not_cached() {
        loop {}
    }
    table! {
        users { id -> Integer, name -> Text, }
    }
    #[test]
    fn inserts_from_select_are_cached() {
        loop {}
    }
    #[test]
    fn single_inserts_are_cached() {
        loop {}
    }
    #[test]
    fn dynamic_batch_inserts_are_not_cached() {
        loop {}
    }
    #[test]
    fn static_batch_inserts_are_cached() {
        loop {}
    }
    #[test]
    fn queries_containing_in_with_vec_are_cached() {
        loop {}
    }
    fn connection() -> PgConnection {
        loop {}
    }
    #[test]
    fn transaction_manager_returns_an_error_when_attempting_to_commit_outside_of_a_transaction() {
        loop {}
    }
    #[test]
    fn transaction_manager_returns_an_error_when_attempting_to_rollback_outside_of_a_transaction() {
        loop {}
    }
    #[test]
    fn postgres_transaction_is_rolled_back_upon_syntax_error() {
        loop {}
    }
    #[test]
    fn nested_postgres_transaction_is_rolled_back_upon_syntax_error() {
        loop {}
    }
    #[test]
    #[allow(clippy::needless_collect)]
    fn postgres_transaction_depth_is_tracked_properly_on_serialization_failure() {
        loop {}
    }
    #[test]
    #[allow(clippy::needless_collect)]
    fn postgres_transaction_depth_is_tracked_properly_on_nested_serialization_failure() {
        loop {}
    }
    #[test]
    fn postgres_transaction_is_rolled_back_upon_deferred_constraint_failure() {
        loop {}
    }
    #[test]
    fn postgres_transaction_is_rolled_back_upon_deferred_trigger_failure() {
        loop {}
    }
    #[test]
    fn nested_postgres_transaction_is_rolled_back_upon_deferred_trigger_failure() {
        loop {}
    }
    #[test]
    fn nested_postgres_transaction_is_rolled_back_upon_deferred_constraint_failure() {
        loop {}
    }
}
