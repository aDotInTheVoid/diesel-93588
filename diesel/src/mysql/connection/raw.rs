extern crate mysqlclient_sys as ffi;
use std::ffi::CStr;
use std::os::raw as libc;
use std::ptr::{self, NonNull};
use std::sync::Once;
use super::stmt::Statement;
use super::url::ConnectionOptions;
use crate::result::{ConnectionError, ConnectionResult, QueryResult};
pub(super) struct RawConnection(NonNull<ffi::MYSQL>);
impl RawConnection {
    pub(super) fn new() -> Self {
        loop {}
    }
    pub(super) fn connect(
        &self,
        connection_options: &ConnectionOptions,
    ) -> ConnectionResult<()> {
        loop {}
    }
    pub(super) fn last_error_message(&self) -> String {
        loop {}
    }
    pub(super) fn execute(&self, query: &str) -> QueryResult<()> {
        loop {}
    }
    pub(super) fn enable_multi_statements<T, F>(&self, f: F) -> QueryResult<T>
    where
        F: FnOnce() -> QueryResult<T>,
    {
        loop {}
    }
    pub(super) fn prepare(&self, query: &str) -> QueryResult<Statement> {
        loop {}
    }
    fn did_an_error_occur(&self) -> QueryResult<()> {
        loop {}
    }
    fn flush_pending_results(&self) -> QueryResult<()> {
        loop {}
    }
    fn consume_current_result(&self) -> QueryResult<()> {
        loop {}
    }
    fn more_results(&self) -> bool {
        loop {}
    }
    fn next_result(&self) -> QueryResult<()> {
        loop {}
    }
    fn set_ssl_mode(&self, ssl_mode: mysqlclient_sys::mysql_ssl_mode) {
        loop {}
    }
    fn set_ssl_ca(&self, ssl_ca: &CStr) {
        loop {}
    }
}
impl Drop for RawConnection {
    fn drop(&mut self) {
        loop {}
    }
}
/// > In a non-multi-threaded environment, `mysql_init()` invokes
/// > `mysql_library_init()` automatically as necessary. However,
/// > `mysql_library_init()` is not thread-safe in a multi-threaded environment,
/// > and thus neither is `mysql_init()`. Before calling `mysql_init()`, either
/// > call `mysql_library_init()` prior to spawning any threads, or use a mutex
/// > to protect the `mysql_library_init()` call. This should be done prior to
/// > any other client library call.
///
/// <https://dev.mysql.com/doc/refman/5.7/en/mysql-init.html>
static MYSQL_THREAD_UNSAFE_INIT: Once = Once::new();
fn perform_thread_unsafe_library_initialization() {
    loop {}
}
