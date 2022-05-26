extern crate pq_sys;
use self::pq_sys::*;
use std::ffi::CStr;
use std::num::NonZeroU32;
use std::os::raw as libc;
use std::rc::Rc;
use std::{slice, str};
use super::raw::{RawConnection, RawResult};
use super::row::PgRow;
use crate::result::{DatabaseErrorInformation, DatabaseErrorKind, Error, QueryResult};
use crate::util::OnceCell;
pub(crate) struct PgResult {
    internal_result: RawResult,
    column_count: usize,
    row_count: usize,
    column_name_map: OnceCell<Vec<Option<*const str>>>,
}
impl PgResult {
    #[allow(clippy::new_ret_no_self)]
    pub(super) fn new(
        internal_result: RawResult,
        conn: &RawConnection,
    ) -> QueryResult<Self> {
        loop {}
    }
    pub(super) fn rows_affected(&self) -> usize {
        loop {}
    }
    pub(super) fn num_rows(&self) -> usize {
        loop {}
    }
    pub(super) fn get_row(self: Rc<Self>, idx: usize) -> PgRow {
        loop {}
    }
    pub(super) fn get(&self, row_idx: usize, col_idx: usize) -> Option<&[u8]> {
        loop {}
    }
    pub(super) fn is_null(&self, row_idx: usize, col_idx: usize) -> bool {
        loop {}
    }
    pub(super) fn column_type(&self, col_idx: usize) -> NonZeroU32 {
        loop {}
    }
    pub(super) fn column_name(&self, col_idx: usize) -> Option<&str> {
        loop {}
    }
    pub(super) fn column_count(&self) -> usize {
        loop {}
    }
}
struct PgErrorInformation(RawResult);
impl DatabaseErrorInformation for PgErrorInformation {
    fn message(&self) -> &str {
        loop {}
    }
    fn details(&self) -> Option<&str> {
        loop {}
    }
    fn hint(&self) -> Option<&str> {
        loop {}
    }
    fn table_name(&self) -> Option<&str> {
        loop {}
    }
    fn column_name(&self) -> Option<&str> {
        loop {}
    }
    fn constraint_name(&self) -> Option<&str> {
        loop {}
    }
    fn statement_position(&self) -> Option<i32> {
        loop {}
    }
}
#[repr(i32)]
enum ResultField {
    SqlState = 'C' as i32,
    MessagePrimary = 'M' as i32,
    MessageDetail = 'D' as i32,
    MessageHint = 'H' as i32,
    TableName = 't' as i32,
    ColumnName = 'c' as i32,
    ConstraintName = 'n' as i32,
    StatementPosition = 'P' as i32,
}
fn get_result_field<'a>(res: *mut PGresult, field: ResultField) -> Option<&'a str> {
    loop {}
}
mod error_codes {
    pub(in crate::pg::connection) const CONNECTION_EXCEPTION: &str = "08000";
    pub(in crate::pg::connection) const CONNECTION_FAILURE: &str = "08006";
    pub(in crate::pg::connection) const SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION: &str = "08001";
    pub(in crate::pg::connection) const SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION: &str = "08004";
    pub(in crate::pg::connection) const NOT_NULL_VIOLATION: &str = "23502";
    pub(in crate::pg::connection) const FOREIGN_KEY_VIOLATION: &str = "23503";
    pub(in crate::pg::connection) const UNIQUE_VIOLATION: &str = "23505";
    pub(in crate::pg::connection) const CHECK_VIOLATION: &str = "23514";
    pub(in crate::pg::connection) const READ_ONLY_TRANSACTION: &str = "25006";
    pub(in crate::pg::connection) const SERIALIZATION_FAILURE: &str = "40001";
}
