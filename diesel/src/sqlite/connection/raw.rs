extern crate libsqlite3_sys as ffi;
use std::ffi::{CString, NulError};
use std::io::{stderr, Write};
use std::os::raw as libc;
use std::ptr::NonNull;
use std::{mem, ptr, slice, str};
use super::functions::{build_sql_function_args, process_sql_function_result};
use super::stmt::ensure_sqlite_ok;
use super::{Sqlite, SqliteAggregateFunction};
use crate::deserialize::FromSqlRow;
use crate::result::Error::DatabaseError;
use crate::result::*;
use crate::serialize::ToSql;
use crate::sql_types::HasSqlType;
macro_rules! assert_fail {
    ($fmt:expr $(,$args:tt)*) => {
        eprint!(concat!($fmt,
        "If you see this message, please open an issue at https://github.com/diesel-rs/diesel/issues/new.\n",
        "Source location: {}:{}\n",), $($args,)* file!(), line!()); std::process::abort()
    };
}
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub(super) struct RawConnection {
    pub(super) internal_connection: NonNull<ffi::sqlite3>,
}
impl RawConnection {
    pub(super) fn establish(database_url: &str) -> ConnectionResult<Self> {
        loop {}
    }
    pub(super) fn exec(&self, query: &str) -> QueryResult<()> {
        loop {}
    }
    pub(super) fn rows_affected_by_last_query(&self) -> usize {
        loop {}
    }
    pub(super) fn register_sql_function<F, Ret, RetSqlType>(
        &self,
        fn_name: &str,
        num_args: usize,
        deterministic: bool,
        f: F,
    ) -> QueryResult<()>
    where
        F: FnMut(&Self, &mut [*mut ffi::sqlite3_value]) -> QueryResult<Ret>
            + std::panic::UnwindSafe + Send + 'static,
        Ret: ToSql<RetSqlType, Sqlite>,
        Sqlite: HasSqlType<RetSqlType>,
    {
        loop {}
    }
    pub(super) fn register_aggregate_function<ArgsSqlType, RetSqlType, Args, Ret, A>(
        &self,
        fn_name: &str,
        num_args: usize,
    ) -> QueryResult<()>
    where
        A: SqliteAggregateFunction<Args, Output = Ret> + 'static + Send
            + std::panic::UnwindSafe,
        Args: FromSqlRow<ArgsSqlType, Sqlite>,
        Ret: ToSql<RetSqlType, Sqlite>,
        Sqlite: HasSqlType<RetSqlType>,
    {
        loop {}
    }
    pub(super) fn register_collation_function<F>(
        &self,
        collation_name: &str,
        collation: F,
    ) -> QueryResult<()>
    where
        F: Fn(&str, &str) -> std::cmp::Ordering + std::panic::UnwindSafe + Send
            + 'static,
    {
        loop {}
    }
    fn get_fn_name(fn_name: &str) -> Result<CString, NulError> {
        loop {}
    }
    fn get_flags(deterministic: bool) -> i32 {
        loop {}
    }
    fn process_sql_function_result(result: i32) -> Result<(), Error> {
        loop {}
    }
}
impl Drop for RawConnection {
    fn drop(&mut self) {
        loop {}
    }
}
enum SqliteCallbackError {
    Abort(&'static str),
    DieselError(crate::result::Error),
    Panic(Box<dyn std::any::Any + Send>, String),
}
impl SqliteCallbackError {
    fn emit(&self, ctx: *mut ffi::sqlite3_context) {
        loop {}
    }
}
impl From<crate::result::Error> for SqliteCallbackError {
    fn from(e: crate::result::Error) -> Self {
        loop {}
    }
}
struct CustomFunctionUserPtr<F> {
    callback: F,
    function_name: String,
}
#[allow(warnings)]
extern "C" fn run_custom_function<F, Ret, RetSqlType>(
    ctx: *mut ffi::sqlite3_context,
    num_args: libc::c_int,
    value_ptr: *mut *mut ffi::sqlite3_value,
)
where
    F: FnMut(&RawConnection, &mut [*mut ffi::sqlite3_value]) -> QueryResult<Ret>
        + std::panic::UnwindSafe + Send + 'static,
    Ret: ToSql<RetSqlType, Sqlite>,
    Sqlite: HasSqlType<RetSqlType>,
{
    loop {}
}
#[repr(u8)]
enum OptionalAggregator<A> {
    None,
    Some(A),
}
#[allow(warnings)]
extern "C" fn run_aggregator_step_function<ArgsSqlType, RetSqlType, Args, Ret, A>(
    ctx: *mut ffi::sqlite3_context,
    num_args: libc::c_int,
    value_ptr: *mut *mut ffi::sqlite3_value,
)
where
    A: SqliteAggregateFunction<Args, Output = Ret> + 'static + Send
        + std::panic::UnwindSafe,
    Args: FromSqlRow<ArgsSqlType, Sqlite>,
    Ret: ToSql<RetSqlType, Sqlite>,
    Sqlite: HasSqlType<RetSqlType>,
{
    loop {}
}
fn run_aggregator_step<A, Args, ArgsSqlType>(
    ctx: *mut ffi::sqlite3_context,
    args: &mut [*mut ffi::sqlite3_value],
) -> Result<(), SqliteCallbackError>
where
    A: SqliteAggregateFunction<Args>,
    Args: FromSqlRow<ArgsSqlType, Sqlite>,
{
    loop {}
}
extern "C" fn run_aggregator_final_function<ArgsSqlType, RetSqlType, Args, Ret, A>(
    ctx: *mut ffi::sqlite3_context,
)
where
    A: SqliteAggregateFunction<Args, Output = Ret> + 'static + Send,
    Args: FromSqlRow<ArgsSqlType, Sqlite>,
    Ret: ToSql<RetSqlType, Sqlite>,
    Sqlite: HasSqlType<RetSqlType>,
{
    loop {}
}
unsafe fn context_error_str(ctx: *mut ffi::sqlite3_context, error: &str) {
    loop {}
}
struct CollationUserPtr<F> {
    callback: F,
    collation_name: String,
}
#[allow(warnings)]
extern "C" fn run_collation_function<F>(
    user_ptr: *mut libc::c_void,
    lhs_len: libc::c_int,
    lhs_ptr: *const libc::c_void,
    rhs_len: libc::c_int,
    rhs_ptr: *const libc::c_void,
) -> libc::c_int
where
    F: Fn(&str, &str) -> std::cmp::Ordering + Send + std::panic::UnwindSafe + 'static,
{
    loop {}
}
extern "C" fn destroy_boxed<F>(data: *mut libc::c_void) {
    loop {}
}
