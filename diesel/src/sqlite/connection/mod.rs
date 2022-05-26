extern crate libsqlite3_sys as ffi;
mod bind_collector;
mod functions;
mod raw;
mod row;
mod sqlite_value;
mod statement_iterator;
mod stmt;
pub(in crate::sqlite) use self::bind_collector::SqliteBindCollector;
pub use self::bind_collector::SqliteBindValue;
pub use self::sqlite_value::SqliteValue;
use std::os::raw as libc;
use self::raw::RawConnection;
use self::statement_iterator::*;
use self::stmt::{Statement, StatementUse};
use super::SqliteAggregateFunction;
use crate::connection::commit_error_processor::{
    default_process_commit_error, CommitErrorOutcome, CommitErrorProcessor,
};
use crate::connection::statement_cache::StatementCache;
use crate::connection::*;
use crate::deserialize::{FromSqlRow, StaticallySizedRow};
use crate::expression::QueryMetadata;
use crate::query_builder::*;
use crate::result::*;
use crate::serialize::ToSql;
use crate::sql_types::HasSqlType;
use crate::sqlite::Sqlite;
#[allow(missing_debug_implementations)]
#[cfg(feature = "sqlite")]
pub struct SqliteConnection {
    statement_cache: StatementCache<Sqlite, Statement>,
    raw_connection: RawConnection,
    transaction_state: AnsiTransactionManager,
}
unsafe impl Send for SqliteConnection {}
impl SimpleConnection for SqliteConnection {
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        loop {}
    }
}
impl<'conn, 'query> ConnectionGatWorkaround<'conn, 'query, Sqlite> for SqliteConnection {
    type Cursor = StatementIterator<'conn, 'query>;
    type Row = self::row::SqliteRow<'conn, 'query>;
}
impl CommitErrorProcessor for SqliteConnection {
    fn process_commit_error(&self, error: Error) -> CommitErrorOutcome {
        loop {}
    }
}
impl Connection for SqliteConnection {
    type Backend = Sqlite;
    type TransactionManager = AnsiTransactionManager;
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
    fn transaction_state(&mut self) -> &mut AnsiTransactionManager
    where
        Self: Sized,
    {
        loop {}
    }
}
#[cfg(feature = "r2d2")]
impl crate::r2d2::R2D2Connection for crate::sqlite::SqliteConnection {
    fn ping(&mut self) -> QueryResult<()> {
        loop {}
    }
    fn is_broken(&mut self) -> bool {
        loop {}
    }
}
impl SqliteConnection {
    pub fn immediate_transaction<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<Error>,
    {
        loop {}
    }
    pub fn exclusive_transaction<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<Error>,
    {
        loop {}
    }
    fn transaction_sql<T, E, F>(&mut self, f: F, sql: &str) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<Error>,
    {
        loop {}
    }
    fn prepared_query<'a, 'b, T>(
        &'a mut self,
        source: T,
    ) -> QueryResult<StatementUse<'a, 'b>>
    where
        T: QueryFragment<Sqlite> + QueryId + 'b,
    {
        loop {}
    }
    #[doc(hidden)]
    pub fn register_sql_function<ArgsSqlType, RetSqlType, Args, Ret, F>(
        &mut self,
        fn_name: &str,
        deterministic: bool,
        mut f: F,
    ) -> QueryResult<()>
    where
        F: FnMut(Args) -> Ret + std::panic::UnwindSafe + Send + 'static,
        Args: FromSqlRow<ArgsSqlType, Sqlite> + StaticallySizedRow<ArgsSqlType, Sqlite>,
        Ret: ToSql<RetSqlType, Sqlite>,
        Sqlite: HasSqlType<RetSqlType>,
    {
        loop {}
    }
    #[doc(hidden)]
    pub fn register_noarg_sql_function<RetSqlType, Ret, F>(
        &self,
        fn_name: &str,
        deterministic: bool,
        f: F,
    ) -> QueryResult<()>
    where
        F: FnMut() -> Ret + std::panic::UnwindSafe + Send + 'static,
        Ret: ToSql<RetSqlType, Sqlite>,
        Sqlite: HasSqlType<RetSqlType>,
    {
        loop {}
    }
    #[doc(hidden)]
    pub fn register_aggregate_function<ArgsSqlType, RetSqlType, Args, Ret, A>(
        &mut self,
        fn_name: &str,
    ) -> QueryResult<()>
    where
        A: SqliteAggregateFunction<Args, Output = Ret> + 'static + Send
            + std::panic::UnwindSafe,
        Args: FromSqlRow<ArgsSqlType, Sqlite> + StaticallySizedRow<ArgsSqlType, Sqlite>,
        Ret: ToSql<RetSqlType, Sqlite>,
        Sqlite: HasSqlType<RetSqlType>,
    {
        loop {}
    }
    pub fn register_collation<F>(
        &mut self,
        collation_name: &str,
        collation: F,
    ) -> QueryResult<()>
    where
        F: Fn(&str, &str) -> std::cmp::Ordering + Send + 'static
            + std::panic::UnwindSafe,
    {
        loop {}
    }
    fn register_diesel_sql_functions(&self) -> QueryResult<()> {
        loop {}
    }
}
fn error_message(err_code: libc::c_int) -> &'static str {
    loop {}
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::sql;
    use crate::prelude::*;
    use crate::sql_types::Integer;
    #[test]
    fn prepared_statements_are_cached_when_run() {
        loop {}
    }
    #[test]
    fn sql_literal_nodes_are_not_cached() {
        loop {}
    }
    #[test]
    fn queries_containing_sql_literal_nodes_are_not_cached() {
        loop {}
    }
    #[test]
    fn queries_containing_in_with_vec_are_not_cached() {
        loop {}
    }
    #[test]
    fn queries_containing_in_with_subselect_are_cached() {
        loop {}
    }
    use crate::sql_types::Text;
    sql_function!(fn fun_case(x : Text) -> Text);
    #[test]
    fn register_custom_function() {
        loop {}
    }
    sql_function!(fn my_add(x : Integer, y : Integer) -> Integer);
    #[test]
    fn register_multiarg_function() {
        loop {}
    }
    sql_function!(fn answer() -> Integer);
    #[test]
    fn register_noarg_function() {
        loop {}
    }
    #[test]
    fn register_nondeterministic_noarg_function() {
        loop {}
    }
    sql_function!(fn add_counter(x : Integer) -> Integer);
    #[test]
    fn register_nondeterministic_function() {
        loop {}
    }
    use crate::sqlite::SqliteAggregateFunction;
    sql_function! {
        #[aggregate] fn my_sum(expr : Integer) -> Integer;
    }
    #[derive(Default)]
    struct MySum {
        sum: i32,
    }
    impl SqliteAggregateFunction<i32> for MySum {
        type Output = i32;
        fn step(&mut self, expr: i32) {
            loop {}
        }
        fn finalize(aggregator: Option<Self>) -> Self::Output {
            loop {}
        }
    }
    table! {
        my_sum_example { id -> Integer, value -> Integer, }
    }
    #[test]
    fn register_aggregate_function() {
        loop {}
    }
    #[test]
    fn register_aggregate_function_returns_finalize_default_on_empty_set() {
        loop {}
    }
    sql_function! {
        #[aggregate] fn range_max(expr1 : Integer, expr2 : Integer, expr3 : Integer) ->
        Nullable < Integer >;
    }
    #[derive(Default)]
    struct RangeMax<T> {
        max_value: Option<T>,
    }
    impl<T: Default + Ord + Copy + Clone> SqliteAggregateFunction<(T, T, T)>
    for RangeMax<T> {
        type Output = Option<T>;
        fn step(&mut self, (x0, x1, x2): (T, T, T)) {
            loop {}
        }
        fn finalize(aggregator: Option<Self>) -> Self::Output {
            loop {}
        }
    }
    table! {
        range_max_example { id -> Integer, value1 -> Integer, value2 -> Integer, value3
        -> Integer, }
    }
    #[test]
    fn register_aggregate_multiarg_function() {
        loop {}
    }
    table! {
        my_collation_example { id -> Integer, value -> Text, }
    }
    #[test]
    fn register_collation_function() {
        loop {}
    }
}
