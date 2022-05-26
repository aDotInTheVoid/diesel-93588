use std::cell::RefCell;
use std::rc::Rc;
use super::row::{PrivateSqliteRow, SqliteRow};
use super::stmt::StatementUse;
use crate::result::QueryResult;
#[allow(missing_debug_implementations)]
pub struct StatementIterator<'stmt, 'query> {
    inner: PrivateStatementIterator<'stmt, 'query>,
    column_names: Option<Rc<[Option<String>]>>,
    field_count: usize,
}
impl<'stmt, 'query> StatementIterator<'stmt, 'query> {
    #[cold]
    fn handle_duplicated_row_case(
        outer_last_row: &mut Rc<RefCell<PrivateSqliteRow<'stmt, 'query>>>,
        column_names: &mut Option<Rc<[Option<String>]>>,
        field_count: usize,
    ) -> Option<QueryResult<SqliteRow<'stmt, 'query>>> {
        loop {}
    }
}
enum PrivateStatementIterator<'stmt, 'query> {
    NotStarted(Option<StatementUse<'stmt, 'query>>),
    Started(Rc<RefCell<PrivateSqliteRow<'stmt, 'query>>>),
}
impl<'stmt, 'query> StatementIterator<'stmt, 'query> {
    pub fn new(stmt: StatementUse<'stmt, 'query>) -> StatementIterator<'stmt, 'query> {
        loop {}
    }
}
impl<'stmt, 'query> Iterator for StatementIterator<'stmt, 'query> {
    type Item = QueryResult<SqliteRow<'stmt, 'query>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {}
    }
}
