#![allow(dead_code)]
use crate::backend::Backend;
use crate::connection::commit_error_processor::CommitErrorProcessor;
use crate::connection::{AnsiTransactionManager, TransactionManager};
use crate::pg::Pg;
use crate::prelude::*;
use crate::query_builder::{AstPass, QueryBuilder, QueryFragment};
use crate::result::Error;
/// Used to build a transaction, specifying additional details.
///
/// This struct is returned by [`.build_transaction`].
/// See the documentation for methods on this struct for usage examples.
/// See [the PostgreSQL documentation for `SET TRANSACTION`][pg-docs]
/// for details on the behavior of each option.
///
/// [`.build_transaction`]: PgConnection::build_transaction()
/// [pg-docs]: https://www.postgresql.org/docs/current/static/sql-set-transaction.html
#[allow(missing_debug_implementations)]
#[must_use = "Transaction builder does nothing unless you call `run` on it"]
#[cfg(feature = "postgres_backend")]
pub struct TransactionBuilder<'a, C> {
    connection: &'a mut C,
    isolation_level: Option<IsolationLevel>,
    read_mode: Option<ReadMode>,
    deferrable: Option<Deferrable>,
}
impl<'a, C> TransactionBuilder<'a, C>
where
    C: Connection<Backend = Pg, TransactionManager = AnsiTransactionManager>
        + CommitErrorProcessor,
{
    pub(crate) fn new(connection: &'a mut C) -> Self {
        loop {}
    }
    /// Makes the transaction `READ ONLY`
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// # use diesel::sql_query;
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # table! {
    /// #     users_for_read_only {
    /// #         id -> Integer,
    /// #         name -> Text,
    /// #     }
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use users_for_read_only::table as users;
    /// #     use users_for_read_only::columns::*;
    /// #     let conn = &mut connection_no_transaction();
    /// #     sql_query("CREATE TABLE IF NOT EXISTS users_for_read_only (
    /// #       id SERIAL PRIMARY KEY,
    /// #       name TEXT NOT NULL
    /// #     )").execute(conn)?;
    /// conn.build_transaction()
    ///     .read_only()
    ///     .run::<_, diesel::result::Error, _>(|conn| {
    ///         let read_attempt = users.select(name).load::<String>(conn);
    ///         assert!(read_attempt.is_ok());
    ///
    ///         let write_attempt = diesel::insert_into(users)
    ///             .values(name.eq("Ruby"))
    ///             .execute(conn);
    ///         assert!(write_attempt.is_err());
    ///
    ///         Ok(())
    ///     })?;
    /// #     sql_query("DROP TABLE users_for_read_only").execute(conn)?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn read_only(mut self) -> Self {
        loop {}
    }
    /// Makes the transaction `READ WRITE`
    ///
    /// This is the default, unless you've changed the
    /// `default_transaction_read_only` configuration parameter.
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// # use diesel::result::Error::RollbackTransaction;
    /// # use diesel::sql_query;
    /// #
    /// # fn main() {
    /// #     assert_eq!(run_test(), Err(RollbackTransaction));
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .read_write()
    ///     .run(|conn| {
    /// #         sql_query("CREATE TABLE IF NOT EXISTS users (
    /// #             id SERIAL PRIMARY KEY,
    /// #             name TEXT NOT NULL
    /// #         )").execute(conn)?;
    ///         let read_attempt = users.select(name).load::<String>(conn);
    ///         assert!(read_attempt.is_ok());
    ///
    ///         let write_attempt = diesel::insert_into(users)
    ///             .values(name.eq("Ruby"))
    ///             .execute(conn);
    ///         assert!(write_attempt.is_ok());
    ///
    /// #       Err(RollbackTransaction)
    /// #       /*
    ///         Ok(())
    /// #       */
    ///     })
    /// # }
    /// ```
    pub fn read_write(mut self) -> Self {
        loop {}
    }
    /// Makes the transaction `DEFERRABLE`
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .deferrable()
    ///     .run(|conn| Ok(()))
    /// # }
    /// ```
    pub fn deferrable(mut self) -> Self {
        loop {}
    }
    /// Makes the transaction `NOT DEFERRABLE`
    ///
    /// This is the default, unless you've changed the
    /// `default_transaction_deferrable` configuration parameter.
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .not_deferrable()
    ///     .run(|conn| Ok(()))
    /// # }
    /// ```
    pub fn not_deferrable(mut self) -> Self {
        loop {}
    }
    /// Makes the transaction `ISOLATION LEVEL READ COMMITTED`
    ///
    /// This is the default, unless you've changed the
    /// `default_transaction_isolation_level` configuration parameter.
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .read_committed()
    ///     .run(|conn| Ok(()))
    /// # }
    /// ```
    pub fn read_committed(mut self) -> Self {
        loop {}
    }
    /// Makes the transaction `ISOLATION LEVEL REPEATABLE READ`
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .repeatable_read()
    ///     .run(|conn| Ok(()))
    /// # }
    /// ```
    pub fn repeatable_read(mut self) -> Self {
        loop {}
    }
    /// Makes the transaction `ISOLATION LEVEL SERIALIZABLE`
    ///
    /// # Example
    ///
    /// ```rust
    /// # include!("../doctest_setup.rs");
    /// #
    /// # fn main() {
    /// #     run_test().unwrap();
    /// # }
    /// #
    /// # fn run_test() -> QueryResult<()> {
    /// #     use schema::users::dsl::*;
    /// #     let conn = &mut connection_no_transaction();
    /// conn.build_transaction()
    ///     .serializable()
    ///     .run(|conn| Ok(()))
    /// # }
    /// ```
    pub fn serializable(mut self) -> Self {
        loop {}
    }
    /// Runs the given function inside of the transaction
    /// with the parameters given to this builder.
    ///
    /// This function executes the provided closure `f` inside a database
    /// transaction. If there is already an open transaction for the current
    /// connection it will return an error. The connection is commited if
    /// the closure returns `Ok(_)`, it will be rolled back if it returns `Err(_)`.
    /// For both cases the original result value will be returned from this function.
    ///
    /// If the transaction fails to commit due to a `SerializationFailure` or a
    /// `ReadOnlyTransaction` a rollback will be attempted. In this case a
    /// [`Error::CommitTransactionFailed`](crate::result::Error::CommitTransactionFailed)
    /// error is returned, which contains details about the original error and
    /// the success of the rollback attempt.
    /// If the rollback failed the connection should be considered broken
    /// as it contains a uncommitted unabortable open transaction. Any further
    /// interaction with the transaction system will result in an returned error
    /// in this cases.
    ///
    /// If the closure returns an `Err(_)` and the rollback fails the function
    /// will return a [`Error::RollbackError`](crate::result::Error::RollbackError)
    /// wrapping the error generated by the rollback operation instead.
    /// In this case the connection should be considered broken as it contains
    /// an unabortable open transaction.
    pub fn run<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut C) -> Result<T, E>,
        E: From<Error>,
    {
        loop {}
    }
}
impl<'a, C> QueryFragment<Pg> for TransactionBuilder<'a, C> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
enum IsolationLevel {
    ReadCommitted,
    RepeatableRead,
    Serializable,
}
impl QueryFragment<Pg> for IsolationLevel {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
enum ReadMode {
    ReadOnly,
    ReadWrite,
}
impl QueryFragment<Pg> for ReadMode {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
#[derive(Debug, Clone, Copy)]
enum Deferrable {
    Deferrable,
    NotDeferrable,
}
impl QueryFragment<Pg> for Deferrable {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        loop {}
    }
}
#[test]
fn test_transaction_builder_generates_correct_sql() {
    loop {}
}
