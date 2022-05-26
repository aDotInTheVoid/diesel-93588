extern crate libsqlite3_sys as ffi;
use super::bind_collector::{InternalSqliteBindValue, SqliteBindCollector};
use super::raw::RawConnection;
use super::sqlite_value::OwnedSqliteValue;
use crate::connection::statement_cache::{MaybeCached, PrepareForCache};
use crate::query_builder::{QueryFragment, QueryId};
use crate::result::Error::DatabaseError;
use crate::result::*;
use crate::sqlite::{Sqlite, SqliteType};
use crate::util::OnceCell;
use std::ffi::{CStr, CString};
use std::io::{stderr, Write};
use std::os::raw as libc;
use std::ptr::{self, NonNull};
pub(super) struct Statement {
    inner_statement: NonNull<ffi::sqlite3_stmt>,
}
impl Statement {
    pub(super) fn prepare(
        raw_connection: &RawConnection,
        sql: &str,
        is_cached: PrepareForCache,
    ) -> QueryResult<Self> {
        loop {}
    }
    unsafe fn bind(
        &mut self,
        tpe: SqliteType,
        value: InternalSqliteBindValue<'_>,
        bind_index: i32,
    ) -> QueryResult<Option<NonNull<[u8]>>> {
        loop {}
    }
    fn reset(&mut self) {
        loop {}
    }
    fn raw_connection(&self) -> *mut ffi::sqlite3 {
        loop {}
    }
}
pub(super) fn ensure_sqlite_ok(
    code: libc::c_int,
    raw_connection: *mut ffi::sqlite3,
) -> QueryResult<()> {
    loop {}
}
fn last_error(raw_connection: *mut ffi::sqlite3) -> Error {
    loop {}
}
fn last_error_message(conn: *mut ffi::sqlite3) -> String {
    loop {}
}
fn last_error_code(conn: *mut ffi::sqlite3) -> libc::c_int {
    loop {}
}
impl Drop for Statement {
    fn drop(&mut self) {
        loop {}
    }
}
struct BoundStatement<'stmt, 'query> {
    statement: MaybeCached<'stmt, Statement>,
    query: Option<NonNull<dyn QueryFragment<Sqlite> + 'query>>,
    binds_to_free: Vec<(i32, Option<NonNull<[u8]>>)>,
}
impl<'stmt, 'query> BoundStatement<'stmt, 'query> {
    fn bind<T>(
        statement: MaybeCached<'stmt, Statement>,
        query: T,
    ) -> QueryResult<BoundStatement<'stmt, 'query>>
    where
        T: QueryFragment<Sqlite> + QueryId + 'query,
    {
        loop {}
    }
    fn bind_buffers(
        &mut self,
        binds: Vec<(InternalSqliteBindValue<'_>, SqliteType)>,
    ) -> QueryResult<()> {
        loop {}
    }
}
impl<'stmt, 'query> Drop for BoundStatement<'stmt, 'query> {
    fn drop(&mut self) {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub struct StatementUse<'stmt, 'query> {
    statement: BoundStatement<'stmt, 'query>,
    column_names: OnceCell<Vec<*const str>>,
}
impl<'stmt, 'query> StatementUse<'stmt, 'query> {
    pub(super) fn bind<T>(
        statement: MaybeCached<'stmt, Statement>,
        query: T,
    ) -> QueryResult<StatementUse<'stmt, 'query>>
    where
        T: QueryFragment<Sqlite> + QueryId + 'query,
    {
        loop {}
    }
    pub(super) fn run(mut self) -> QueryResult<()> {
        loop {}
    }
    pub(super) unsafe fn step(&mut self, first_step: bool) -> QueryResult<bool> {
        loop {}
    }
    unsafe fn column_name(&self, idx: i32) -> *const str {
        loop {}
    }
    pub(super) fn column_count(&self) -> i32 {
        loop {}
    }
    pub(super) fn index_for_column_name(&mut self, field_name: &str) -> Option<usize> {
        loop {}
    }
    pub(super) fn field_name(&self, idx: i32) -> Option<&str> {
        loop {}
    }
    pub(super) fn copy_value(&self, idx: i32) -> Option<OwnedSqliteValue> {
        loop {}
    }
    pub(super) fn column_value(&self, idx: i32) -> Option<NonNull<ffi::sqlite3_value>> {
        loop {}
    }
}
