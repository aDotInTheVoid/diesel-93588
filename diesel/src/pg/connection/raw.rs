#![allow(clippy::too_many_arguments)]
extern crate pq_sys;
use self::pq_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw as libc;
use std::ptr::NonNull;
use std::{ptr, str};
use crate::result::*;
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub(super) struct RawConnection {
    internal_connection: NonNull<PGconn>,
}
impl RawConnection {
    pub(super) fn establish(database_url: &str) -> ConnectionResult<Self> {
        loop {}
    }
    pub(super) fn last_error_message(&self) -> String {
        loop {}
    }
    pub(super) fn set_notice_processor(&self, notice_processor: NoticeProcessor) {
        loop {}
    }
    pub(super) unsafe fn exec(
        &self,
        query: *const libc::c_char,
    ) -> QueryResult<RawResult> {
        loop {}
    }
    pub(super) unsafe fn exec_prepared(
        &self,
        stmt_name: *const libc::c_char,
        param_count: libc::c_int,
        param_values: *const *const libc::c_char,
        param_lengths: *const libc::c_int,
        param_formats: *const libc::c_int,
        result_format: libc::c_int,
    ) -> QueryResult<RawResult> {
        loop {}
    }
    pub(super) unsafe fn prepare(
        &self,
        stmt_name: *const libc::c_char,
        query: *const libc::c_char,
        param_count: libc::c_int,
        param_types: *const Oid,
    ) -> QueryResult<RawResult> {
        loop {}
    }
    pub(super) fn transaction_status(&self) -> PgTransactionStatus {
        loop {}
    }
    pub(super) fn get_status(&self) -> ConnStatusType {
        loop {}
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum PgTransactionStatus {
        Idle,
        Active,
        InTransaction,
        InError,
        Unknown,
}
impl From<PGTransactionStatusType> for PgTransactionStatus {
    fn from(trans_status_type: PGTransactionStatusType) -> Self {
        loop {}
    }
}
pub(super) type NoticeProcessor = extern "C" fn(
    arg: *mut libc::c_void,
    message: *const libc::c_char,
);
impl Drop for RawConnection {
    fn drop(&mut self) {
        loop {}
    }
}
fn last_error_message(conn: *const PGconn) -> String {
    loop {}
}
#[allow(missing_debug_implementations)]
pub(super) struct RawResult(NonNull<PGresult>);
unsafe impl Send for RawResult {}
unsafe impl Sync for RawResult {}
impl RawResult {
    #[allow(clippy::new_ret_no_self)]
    fn new(ptr: *mut PGresult, conn: &RawConnection) -> QueryResult<Self> {
        loop {}
    }
    pub(super) fn as_ptr(&self) -> *mut PGresult {
        loop {}
    }
    pub(super) fn error_message(&self) -> &str {
        loop {}
    }
}
impl Drop for RawResult {
    fn drop(&mut self) {
        loop {}
    }
}
