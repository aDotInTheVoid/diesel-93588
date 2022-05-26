use crate::query_builder::BindCollector;
use crate::serialize::{IsNull, Output};
use crate::sql_types::HasSqlType;
use crate::sqlite::{Sqlite, SqliteType};
use crate::QueryResult;
#[derive(Debug)]
pub struct SqliteBindCollector<'a> {
    pub(in crate::sqlite) binds: Vec<(InternalSqliteBindValue<'a>, SqliteType)>,
}
impl SqliteBindCollector<'_> {
    pub(in crate::sqlite) fn new() -> Self {
        loop {}
    }
}
#[derive(Debug)]
pub struct SqliteBindValue<'a> {
    pub(in crate::sqlite) inner: InternalSqliteBindValue<'a>,
}
impl<'a> From<i32> for SqliteBindValue<'a> {
    fn from(i: i32) -> Self {
        loop {}
    }
}
impl<'a> From<i64> for SqliteBindValue<'a> {
    fn from(i: i64) -> Self {
        loop {}
    }
}
impl<'a> From<f64> for SqliteBindValue<'a> {
    fn from(f: f64) -> Self {
        loop {}
    }
}
impl<'a, T> From<Option<T>> for SqliteBindValue<'a>
where
    T: Into<SqliteBindValue<'a>>,
{
    fn from(o: Option<T>) -> Self {
        loop {}
    }
}
impl<'a> From<&'a str> for SqliteBindValue<'a> {
    fn from(s: &'a str) -> Self {
        loop {}
    }
}
impl<'a> From<String> for SqliteBindValue<'a> {
    fn from(s: String) -> Self {
        loop {}
    }
}
impl<'a> From<Vec<u8>> for SqliteBindValue<'a> {
    fn from(b: Vec<u8>) -> Self {
        loop {}
    }
}
impl<'a> From<&'a [u8]> for SqliteBindValue<'a> {
    fn from(b: &'a [u8]) -> Self {
        loop {}
    }
}
#[derive(Debug)]
pub(crate) enum InternalSqliteBindValue<'a> {
    BorrowedString(&'a str),
    String(Box<str>),
    BorrowedBinary(&'a [u8]),
    Binary(Box<[u8]>),
    I32(i32),
    I64(i64),
    F64(f64),
    Null,
}
impl std::fmt::Display for InternalSqliteBindValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        loop {}
    }
}
impl InternalSqliteBindValue<'_> {
    pub(in crate::sqlite) fn result_of(self, ctx: &mut libsqlite3_sys::sqlite3_context) {
        loop {}
    }
}
impl<'a> BindCollector<'a, Sqlite> for SqliteBindCollector<'a> {
    type Buffer = SqliteBindValue<'a>;
    fn push_bound_value<T, U>(
        &mut self,
        bind: &'a U,
        metadata_lookup: &mut (),
    ) -> QueryResult<()>
    where
        Sqlite: crate::sql_types::HasSqlType<T>,
        U: crate::serialize::ToSql<T, Sqlite>,
    {
        loop {}
    }
}
