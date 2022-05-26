#![allow(dead_code)]
use crate::backend::Backend;
use crate::connection::commit_error_processor::CommitErrorProcessor;
use crate::connection::{AnsiTransactionManager, TransactionManager};
use crate::pg::Pg;
use crate::prelude::*;
use crate::query_builder::{AstPass, QueryBuilder, QueryFragment};
use crate::result::Error;
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
    pub fn read_only(mut self) -> Self {
        loop {}
    }
    pub fn read_write(mut self) -> Self {
        loop {}
    }
    pub fn deferrable(mut self) -> Self {
        loop {}
    }
    pub fn not_deferrable(mut self) -> Self {
        loop {}
    }
    pub fn read_committed(mut self) -> Self {
        loop {}
    }
    pub fn repeatable_read(mut self) -> Self {
        loop {}
    }
    pub fn serializable(mut self) -> Self {
        loop {}
    }
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
