extern crate libsqlite3_sys as ffi;
use super::raw::RawConnection;
use super::row::PrivateSqliteRow;
use super::{Sqlite, SqliteAggregateFunction, SqliteBindValue};
use crate::deserialize::{FromSqlRow, StaticallySizedRow};
use crate::result::{DatabaseErrorKind, Error, QueryResult};
use crate::row::{Field, PartialRow, Row, RowGatWorkaround, RowIndex};
use crate::serialize::{IsNull, Output, ToSql};
use crate::sql_types::HasSqlType;
use crate::sqlite::connection::bind_collector::InternalSqliteBindValue;
use crate::sqlite::connection::sqlite_value::OwnedSqliteValue;
use crate::sqlite::SqliteValue;
use std::cell::{Ref, RefCell};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::DerefMut;
use std::rc::Rc;
pub(super) fn register<ArgsSqlType, RetSqlType, Args, Ret, F>(
    conn: &RawConnection,
    fn_name: &str,
    deterministic: bool,
    mut f: F,
) -> QueryResult<()>
where
    F: FnMut(&RawConnection, Args) -> Ret + std::panic::UnwindSafe + Send + 'static,
    Args: FromSqlRow<ArgsSqlType, Sqlite> + StaticallySizedRow<ArgsSqlType, Sqlite>,
    Ret: ToSql<RetSqlType, Sqlite>,
    Sqlite: HasSqlType<RetSqlType>,
{
    loop {}
}
pub(super) fn register_noargs<RetSqlType, Ret, F>(
    conn: &RawConnection,
    fn_name: &str,
    deterministic: bool,
    mut f: F,
) -> QueryResult<()>
where
    F: FnMut() -> Ret + std::panic::UnwindSafe + Send + 'static,
    Ret: ToSql<RetSqlType, Sqlite>,
    Sqlite: HasSqlType<RetSqlType>,
{
    loop {}
}
pub(super) fn register_aggregate<ArgsSqlType, RetSqlType, Args, Ret, A>(
    conn: &RawConnection,
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
pub(super) fn build_sql_function_args<ArgsSqlType, Args>(
    args: &mut [*mut ffi::sqlite3_value],
) -> Result<Args, Error>
where
    Args: FromSqlRow<ArgsSqlType, Sqlite>,
{
    loop {}
}
#[allow(clippy::let_unit_value)]
pub(super) fn process_sql_function_result<RetSqlType, Ret>(
    result: &'_ Ret,
) -> QueryResult<InternalSqliteBindValue<'_>>
where
    Ret: ToSql<RetSqlType, Sqlite>,
    Sqlite: HasSqlType<RetSqlType>,
{
    loop {}
}
struct FunctionRow<'a> {
    args: Rc<RefCell<ManuallyDrop<PrivateSqliteRow<'a, 'static>>>>,
    field_count: usize,
    marker: PhantomData<&'a ffi::sqlite3_value>,
}
impl<'a> Drop for FunctionRow<'a> {
    fn drop(&mut self) {
        loop {}
    }
}
impl<'a> FunctionRow<'a> {
    fn new(args: &mut [*mut ffi::sqlite3_value]) -> Self {
        loop {}
    }
}
impl<'a, 'b> RowGatWorkaround<'a, Sqlite> for FunctionRow<'b> {
    type Field = FunctionArgument<'a>;
}
impl<'a> Row<'a, Sqlite> for FunctionRow<'a> {
    type InnerPartialRow = Self;
    fn field_count(&self) -> usize {
        loop {}
    }
    fn get<'b, I>(
        &'b self,
        idx: I,
    ) -> Option<<Self as RowGatWorkaround<'b, Sqlite>>::Field>
    where
        'a: 'b,
        Self: crate::row::RowIndex<I>,
    {
        loop {}
    }
    fn partial_row(
        &self,
        range: std::ops::Range<usize>,
    ) -> PartialRow<'_, Self::InnerPartialRow> {
        loop {}
    }
}
impl<'a> RowIndex<usize> for FunctionRow<'a> {
    fn idx(&self, idx: usize) -> Option<usize> {
        loop {}
    }
}
impl<'a, 'b> RowIndex<&'a str> for FunctionRow<'b> {
    fn idx(&self, _idx: &'a str) -> Option<usize> {
        loop {}
    }
}
struct FunctionArgument<'a> {
    args: Ref<'a, ManuallyDrop<PrivateSqliteRow<'a, 'static>>>,
    col_idx: i32,
}
impl<'a> Field<'a, Sqlite> for FunctionArgument<'a> {
    fn field_name(&self) -> Option<&str> {
        loop {}
    }
    fn is_null(&self) -> bool {
        loop {}
    }
    fn value(&self) -> Option<crate::backend::RawValue<'_, Sqlite>> {
        loop {}
    }
}
