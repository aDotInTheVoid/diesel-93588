extern crate libsqlite3_sys as ffi;
use std::cell::Ref;
use std::ptr::NonNull;
use std::{slice, str};
use crate::sqlite::SqliteType;
use super::row::PrivateSqliteRow;
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct SqliteValue<'row, 'stmt, 'query> {
    _row: Ref<'row, PrivateSqliteRow<'stmt, 'query>>,
    value: NonNull<ffi::sqlite3_value>,
}
#[repr(transparent)]
pub(super) struct OwnedSqliteValue {
    pub(super) value: NonNull<ffi::sqlite3_value>,
}
impl Drop for OwnedSqliteValue {
    fn drop(&mut self) {
        loop {}
    }
}
impl<'row, 'stmt, 'query> SqliteValue<'row, 'stmt, 'query> {
    pub(super) fn new(
        row: Ref<'row, PrivateSqliteRow<'stmt, 'query>>,
        col_idx: i32,
    ) -> Option<SqliteValue<'row, 'stmt, 'query>> {
        loop {}
    }
    pub(crate) fn parse_string<'value, R>(
        &'value self,
        f: impl FnOnce(&'value str) -> R,
    ) -> R {
        loop {}
    }
    pub(crate) fn read_text(&self) -> &str {
        loop {}
    }
    pub(crate) fn read_blob(&self) -> &[u8] {
        loop {}
    }
    pub(crate) fn read_integer(&self) -> i32 {
        loop {}
    }
    pub(crate) fn read_long(&self) -> i64 {
        loop {}
    }
    pub(crate) fn read_double(&self) -> f64 {
        loop {}
    }
        pub fn value_type(&self) -> Option<SqliteType> {
        loop {}
    }
}
impl OwnedSqliteValue {
    pub(super) fn copy_from_ptr(
        ptr: NonNull<ffi::sqlite3_value>,
    ) -> Option<OwnedSqliteValue> {
        loop {}
    }
    pub(super) fn duplicate(&self) -> OwnedSqliteValue {
        loop {}
    }
}
