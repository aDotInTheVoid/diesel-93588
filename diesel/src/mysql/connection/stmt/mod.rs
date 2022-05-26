extern crate mysqlclient_sys as ffi;
pub(super) mod iterator;
mod metadata;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::os::raw as libc;
use std::ptr::NonNull;
use super::bind::{OutputBinds, PreparedStatementBinds};
use crate::connection::statement_cache::MaybeCached;
use crate::mysql::MysqlType;
use crate::result::{DatabaseErrorKind, Error, QueryResult};
pub(super) use self::metadata::{MysqlFieldMetadata, StatementMetadata};
#[allow(dead_code, missing_debug_implementations)]
pub struct Statement {
    stmt: NonNull<ffi::MYSQL_STMT>,
    input_binds: Option<PreparedStatementBinds>,
}
impl Statement {
    pub(crate) fn new(stmt: NonNull<ffi::MYSQL_STMT>) -> Self {
        loop {}
    }
    pub fn prepare(&self, query: &str) -> QueryResult<()> {
        loop {}
    }
    pub fn bind<Iter>(&mut self, binds: Iter) -> QueryResult<()>
    where
        Iter: IntoIterator<Item = (MysqlType, Option<Vec<u8>>)>,
    {
        loop {}
    }
    pub(super) fn input_bind(
        &mut self,
        mut input_binds: PreparedStatementBinds,
    ) -> QueryResult<()> {
        loop {}
    }
    fn last_error_message(&self) -> String {
        loop {}
    }
    pub(super) fn metadata(&self) -> QueryResult<StatementMetadata> {
        loop {}
    }
    pub(super) fn did_an_error_occur(&self) -> QueryResult<()> {
        loop {}
    }
    fn last_error_type(&self) -> DatabaseErrorKind {
        loop {}
    }
            pub unsafe fn bind_result(&self, binds: *mut ffi::MYSQL_BIND) -> QueryResult<()> {
        loop {}
    }
}
impl<'a> MaybeCached<'a, Statement> {
    pub(super) fn execute_statement(
        self,
        binds: &mut OutputBinds,
    ) -> QueryResult<StatementUse<'a>> {
        loop {}
    }
                pub(super) unsafe fn execute(self) -> QueryResult<StatementUse<'a>> {
        loop {}
    }
}
impl Drop for Statement {
    fn drop(&mut self) {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub(super) struct StatementUse<'a> {
    inner: MaybeCached<'a, Statement>,
}
impl<'a> StatementUse<'a> {
    pub(in crate::mysql::connection) fn affected_rows(&self) -> usize {
        loop {}
    }
            pub(in crate::mysql::connection) unsafe fn result_size(
        &mut self,
    ) -> QueryResult<usize> {
        loop {}
    }
    pub(super) fn populate_row_buffers(
        &self,
        binds: &mut OutputBinds,
    ) -> QueryResult<Option<()>> {
        loop {}
    }
    pub(in crate::mysql::connection) unsafe fn fetch_column(
        &self,
        bind: &mut ffi::MYSQL_BIND,
        idx: usize,
        offset: usize,
    ) -> QueryResult<()> {
        loop {}
    }
            pub(in crate::mysql::connection) unsafe fn bind_result(
        &self,
        binds: *mut ffi::MYSQL_BIND,
    ) -> QueryResult<()> {
        loop {}
    }
}
impl<'a> Drop for StatementUse<'a> {
    fn drop(&mut self) {
        loop {}
    }
}
