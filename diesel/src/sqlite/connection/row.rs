use std::cell::{Ref, RefCell};
use std::convert::TryFrom;
use std::rc::Rc;
use super::sqlite_value::{OwnedSqliteValue, SqliteValue};
use super::stmt::StatementUse;
use crate::row::{Field, PartialRow, Row, RowGatWorkaround, RowIndex};
use crate::sqlite::Sqlite;
#[allow(missing_debug_implementations)]
pub struct SqliteRow<'stmt, 'query> {
    pub(super) inner: Rc<RefCell<PrivateSqliteRow<'stmt, 'query>>>,
    pub(super) field_count: usize,
}
pub(super) enum PrivateSqliteRow<'stmt, 'query> {
    Direct(StatementUse<'stmt, 'query>),
    Duplicated {
        values: Vec<Option<OwnedSqliteValue>>,
        column_names: Rc<[Option<String>]>,
    },
}
impl<'stmt, 'query> PrivateSqliteRow<'stmt, 'query> {
    pub(super) fn duplicate(
        &mut self,
        column_names: &mut Option<Rc<[Option<String>]>>,
    ) -> PrivateSqliteRow<'stmt, 'query> {
        loop {}
    }
}
impl<'field, 'stmt, 'query> RowGatWorkaround<'field, Sqlite>
for SqliteRow<'stmt, 'query> {
    type Field = SqliteField<'field, 'field>;
}
impl<'stmt, 'query> Row<'stmt, Sqlite> for SqliteRow<'stmt, 'query> {
    type InnerPartialRow = Self;
    fn field_count(&self) -> usize {
        loop {}
    }
    fn get<'field, I>(
        &'field self,
        idx: I,
    ) -> Option<<Self as RowGatWorkaround<'field, Sqlite>>::Field>
    where
        'stmt: 'field,
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
impl<'stmt, 'query> RowIndex<usize> for SqliteRow<'stmt, 'query> {
    fn idx(&self, idx: usize) -> Option<usize> {
        loop {}
    }
}
impl<'stmt, 'idx, 'query> RowIndex<&'idx str> for SqliteRow<'stmt, 'query> {
    fn idx(&self, field_name: &'idx str) -> Option<usize> {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub struct SqliteField<'stmt, 'query> {
    pub(super) row: Ref<'stmt, PrivateSqliteRow<'stmt, 'query>>,
    pub(super) col_idx: i32,
}
impl<'stmt, 'query> Field<'stmt, Sqlite> for SqliteField<'stmt, 'query> {
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
#[test]
fn fun_with_row_iters() {
    loop {}
}
