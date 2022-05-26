use std::cell::{Ref, RefCell};
use std::rc::Rc;
use super::{OutputBinds, Statement, StatementMetadata, StatementUse};
use crate::connection::statement_cache::MaybeCached;
use crate::mysql::{Mysql, MysqlType};
use crate::result::QueryResult;
use crate::row::*;
#[allow(missing_debug_implementations)]
pub struct StatementIterator<'a> {
    stmt: StatementUse<'a>,
    last_row: Rc<RefCell<PrivateMysqlRow>>,
    metadata: Rc<StatementMetadata>,
    len: usize,
}
impl<'a> StatementIterator<'a> {
    pub fn from_stmt(
        stmt: MaybeCached<'a, Statement>,
        types: &[Option<MysqlType>],
    ) -> QueryResult<Self> {
        loop {}
    }
}
impl<'a> Iterator for StatementIterator<'a> {
    type Item = QueryResult<MysqlRow>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        loop {}
    }
    fn count(self) -> usize
    where
        Self: Sized,
    {
        loop {}
    }
}
impl<'a> ExactSizeIterator for StatementIterator<'a> {
    fn len(&self) -> usize {
        loop {}
    }
}
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct MysqlRow {
    row: Rc<RefCell<PrivateMysqlRow>>,
    metadata: Rc<StatementMetadata>,
}
enum PrivateMysqlRow {
    Direct(OutputBinds),
    Copied(OutputBinds),
}
impl PrivateMysqlRow {
    fn duplicate(&self) -> Self {
        loop {}
    }
}
impl<'a> RowGatWorkaround<'a, Mysql> for MysqlRow {
    type Field = MysqlField<'a>;
}
impl<'a> Row<'a, Mysql> for MysqlRow {
    type InnerPartialRow = Self;
    fn field_count(&self) -> usize {
        loop {}
    }
    fn get<'b, I>(
        &'b self,
        idx: I,
    ) -> Option<<Self as RowGatWorkaround<'b, Mysql>>::Field>
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
impl RowIndex<usize> for MysqlRow {
    fn idx(&self, idx: usize) -> Option<usize> {
        loop {}
    }
}
impl<'a> RowIndex<&'a str> for MysqlRow {
    fn idx(&self, idx: &'a str) -> Option<usize> {
        loop {}
    }
}
#[allow(missing_debug_implementations)]
pub struct MysqlField<'a> {
    binds: Ref<'a, PrivateMysqlRow>,
    metadata: Rc<StatementMetadata>,
    idx: usize,
}
impl<'a> Field<'a, Mysql> for MysqlField<'a> {
    fn field_name(&self) -> Option<&str> {
        loop {}
    }
    fn is_null(&self) -> bool {
        loop {}
    }
    fn value(&self) -> Option<crate::backend::RawValue<'_, Mysql>> {
        loop {}
    }
}
#[test]
fn fun_with_row_iters() {
    loop {}
}
