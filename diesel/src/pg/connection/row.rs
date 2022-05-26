use super::result::PgResult;
use crate::pg::value::TypeOidLookup;
use crate::pg::{Pg, PgValue};
use crate::row::*;
use std::rc::Rc;
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct PgRow {
    db_result: Rc<PgResult>,
    row_idx: usize,
}
impl PgRow {
    pub(crate) fn new(db_result: Rc<PgResult>, row_idx: usize) -> Self {
        loop {}
    }
}
impl<'a> RowGatWorkaround<'a, Pg> for PgRow {
    type Field = PgField<'a>;
}
impl<'a> Row<'a, Pg> for PgRow {
    type InnerPartialRow = Self;
    fn field_count(&self) -> usize {
        loop {}
    }
    fn get<'b, I>(&'b self, idx: I) -> Option<<Self as RowGatWorkaround<'b, Pg>>::Field>
    where
        'a: 'b,
        Self: RowIndex<I>,
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
impl RowIndex<usize> for PgRow {
    fn idx(&self, idx: usize) -> Option<usize> {
        loop {}
    }
}
impl<'a> RowIndex<&'a str> for PgRow {
    fn idx(&self, field_name: &'a str) -> Option<usize> {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub struct PgField<'a> {
    db_result: &'a PgResult,
    row_idx: usize,
    col_idx: usize,
}
impl<'a> Field<'a, Pg> for PgField<'a> {
    fn field_name(&self) -> Option<&str> {
        loop {}
    }
    fn value(&self) -> Option<crate::backend::RawValue<'_, Pg>> {
        loop {}
    }
}
impl<'a> TypeOidLookup for PgField<'a> {
    fn lookup(&self) -> std::num::NonZeroU32 {
        loop {}
    }
}
