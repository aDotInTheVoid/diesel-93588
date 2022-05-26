extern crate pq_sys;
use std::ffi::CString;
use std::os::raw as libc;
use std::ptr;
use super::result::PgResult;
use crate::pg::PgTypeMetadata;
use crate::result::QueryResult;
use super::raw::RawConnection;
pub(crate) struct Statement {
    name: CString,
    param_formats: Vec<libc::c_int>,
}
impl Statement {
    pub(super) fn execute(
        &self,
        raw_connection: &mut RawConnection,
        param_data: &[Option<Vec<u8>>],
    ) -> QueryResult<PgResult> {
        loop {}
    }
    pub(super) fn prepare(
        raw_connection: &mut RawConnection,
        sql: &str,
        name: Option<&str>,
        param_types: &[PgTypeMetadata],
    ) -> QueryResult<Self> {
        loop {}
    }
}
fn param_types_to_ptr(param_types: Option<&Vec<u32>>) -> *const pq_sys::Oid {
    loop {}
}
