use mysqlclient_sys as ffi;
use std::mem;
use std::mem::MaybeUninit;
use std::ops::Index;
use std::os::raw as libc;
use std::ptr::NonNull;
use super::stmt::MysqlFieldMetadata;
use super::stmt::StatementUse;
use crate::mysql::connection::stmt::StatementMetadata;
use crate::mysql::types::date_and_time::MysqlTime;
use crate::mysql::{MysqlType, MysqlValue};
use crate::result::QueryResult;
pub(super) struct PreparedStatementBinds(Binds);
pub(super) struct OutputBinds(Binds);
impl Clone for OutputBinds {
    fn clone(&self) -> Self {
        loop {}
    }
}
struct Binds {
    data: Vec<BindData>,
}
impl PreparedStatementBinds {
    pub(super) fn from_input_data<Iter>(input: Iter) -> Self
    where
        Iter: IntoIterator<Item = (MysqlType, Option<Vec<u8>>)>,
    {
        loop {}
    }
    pub(super) fn with_mysql_binds<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(*mut ffi::MYSQL_BIND) -> T,
    {
        loop {}
    }
}
impl OutputBinds {
    pub(super) fn from_output_types(
        types: &[Option<MysqlType>],
        metadata: &StatementMetadata,
    ) -> Self {
        loop {}
    }
    pub(super) fn populate_dynamic_buffers(
        &mut self,
        stmt: &StatementUse<'_>,
    ) -> QueryResult<()> {
        loop {}
    }
    pub(super) fn update_buffer_lengths(&mut self) {
        loop {}
    }
    pub(super) fn with_mysql_binds<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(*mut ffi::MYSQL_BIND) -> T,
    {
        loop {}
    }
}
impl Binds {
    fn with_mysql_binds<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(*mut ffi::MYSQL_BIND) -> T,
    {
        loop {}
    }
}
impl Index<usize> for OutputBinds {
    type Output = BindData;
    fn index(&self, index: usize) -> &Self::Output {
        loop {}
    }
}
bitflags::bitflags! {
    pub (crate) struct Flags : u32 { const NOT_NULL_FLAG = 1; const PRI_KEY_FLAG = 2;
    const UNIQUE_KEY_FLAG = 4; const MULTIPLE_KEY_FLAG = 8; const BLOB_FLAG = 16; const
    UNSIGNED_FLAG = 32; const ZEROFILL_FLAG = 64; const BINARY_FLAG = 128; const
    ENUM_FLAG = 256; const AUTO_INCREMENT_FLAG = 512; const TIMESTAMP_FLAG = 1024; const
    SET_FLAG = 2048; const NO_DEFAULT_VALUE_FLAG = 4096; const ON_UPDATE_NOW_FLAG = 8192;
    const NUM_FLAG = 32768; const PART_KEY_FLAG = 16384; const GROUP_FLAG = 32768; const
    UNIQUE_FLAG = 65536; const BINCMP_FLAG = 130_172; const GET_FIXED_FIELDS_FLAG = (1 <<
    18); const FIELD_IN_PART_FUNC_FLAG = (1 << 19); }
}
impl From<u32> for Flags {
    fn from(flags: u32) -> Self {
        loop {}
    }
}
#[derive(Debug)]
pub(super) struct BindData {
    tpe: ffi::enum_field_types,
    bytes: Option<NonNull<u8>>,
    length: libc::c_ulong,
    capacity: usize,
    flags: Flags,
    is_null: ffi::my_bool,
    is_truncated: Option<ffi::my_bool>,
}
impl Clone for BindData {
    fn clone(&self) -> Self {
        loop {}
    }
}
impl Drop for BindData {
    fn drop(&mut self) {
        loop {}
    }
}
impl BindData {
    fn for_input((tpe, data): (MysqlType, Option<Vec<u8>>)) -> Self {
        loop {}
    }
    fn for_output(tpe: Option<MysqlType>, metadata: &MysqlFieldMetadata<'_>) -> Self {
        loop {}
    }
    fn from_tpe_and_flags((tpe, flags): (ffi::enum_field_types, Flags)) -> Self {
        loop {}
    }
    fn is_truncated(&self) -> bool {
        loop {}
    }
    fn is_fixed_size_buffer(&self) -> bool {
        loop {}
    }
    pub(super) fn value(&'_ self) -> Option<MysqlValue<'_>> {
        loop {}
    }
    pub(super) fn is_null(&self) -> bool {
        loop {}
    }
    fn update_buffer_length(&mut self) {
        loop {}
    }
    unsafe fn mysql_bind(&mut self) -> ffi::MYSQL_BIND {
        loop {}
    }
    /// Resizes the byte buffer to fit the value of `self.length`, and returns
    /// a tuple of a bind pointing at the truncated data, and the offset to use
    /// in order to read the truncated data into it.
    ///
    /// This invalidates the bind previously returned by `mysql_bind`. Calling
    /// this function is unsafe unless the binds are immediately rebound.
    unsafe fn bind_for_truncated_data(&mut self) -> Option<(ffi::MYSQL_BIND, usize)> {
        loop {}
    }
    fn did_numeric_overflow_occur(&self) -> QueryResult<()> {
        loop {}
    }
}
impl From<MysqlType> for (ffi::enum_field_types, Flags) {
    fn from(tpe: MysqlType) -> Self {
        loop {}
    }
}
impl From<(ffi::enum_field_types, Flags)> for MysqlType {
    fn from((tpe, flags): (ffi::enum_field_types, Flags)) -> Self {
        loop {}
    }
}
fn known_buffer_size_for_ffi_type(tpe: ffi::enum_field_types) -> Option<usize> {
    loop {}
}
#[cfg(test)]
mod tests {
    #[cfg(feature = "bigdecimal")]
    use bigdecimal::FromPrimitive;
    use super::MysqlValue;
    use super::*;
    use crate::connection::statement_cache::MaybeCached;
    use crate::deserialize::FromSql;
    use crate::mysql::connection::stmt::Statement;
    use crate::prelude::*;
    use crate::sql_types::*;
    fn to_value<ST, T>(
        bind: &BindData,
    ) -> Result<T, Box<(dyn std::error::Error + Send + Sync + 'static)>>
    where
        T: FromSql<ST, crate::mysql::Mysql> + std::fmt::Debug,
    {
        loop {}
    }
    #[cfg(feature = "extras")]
    #[test]
    fn check_all_the_types() {
        loop {}
    }
    fn query_single_table(
        query: &'static str,
        conn: &MysqlConnection,
        bind_tpe: impl Into<(ffi::enum_field_types, Flags)>,
    ) -> BindData {
        loop {}
    }
    fn input_bind(
        query: &'static str,
        conn: &MysqlConnection,
        id: i32,
        (mut field, tpe): (Vec<u8>, impl Into<(ffi::enum_field_types, Flags)>),
    ) {
        loop {}
    }
    #[test]
    fn check_json_bind() {
        loop {}
    }
    #[test]
    fn check_enum_bind() {
        loop {}
    }
    #[test]
    fn check_set_bind() {
        loop {}
    }
}
