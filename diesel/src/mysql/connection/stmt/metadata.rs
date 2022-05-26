use std::ffi::CStr;
use std::ptr::NonNull;
use std::slice;
use super::ffi;
use crate::mysql::connection::bind::Flags;
pub(in crate::mysql::connection) struct StatementMetadata {
    result: NonNull<ffi::MYSQL_RES>,
}
impl StatementMetadata {
    pub(in crate::mysql::connection) fn new(result: NonNull<ffi::MYSQL_RES>) -> Self {
        loop {}
    }
    pub(in crate::mysql::connection) fn fields(
        &'_ self,
    ) -> &'_ [MysqlFieldMetadata<'_>] {
        loop {}
    }
}
impl Drop for StatementMetadata {
    fn drop(&mut self) {
        loop {}
    }
}
#[repr(transparent)]
pub(in crate::mysql::connection) struct MysqlFieldMetadata<'a>(
    ffi::MYSQL_FIELD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MysqlFieldMetadata<'a> {
    pub(in crate::mysql::connection) fn field_name(&self) -> Option<&str> {
        loop {}
    }
    pub(in crate::mysql::connection) fn field_type(&self) -> ffi::enum_field_types {
        loop {}
    }
    pub(in crate::mysql::connection) fn flags(&self) -> Flags {
        loop {}
    }
}
