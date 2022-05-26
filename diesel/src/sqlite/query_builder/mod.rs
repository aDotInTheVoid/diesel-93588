use super::backend::Sqlite;
use crate::query_builder::QueryBuilder;
use crate::result::QueryResult;
mod limit_offset;
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct SqliteQueryBuilder {
    sql: String,
}
impl SqliteQueryBuilder {
    pub fn new() -> Self {
        loop {}
    }
}
impl QueryBuilder<Sqlite> for SqliteQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        loop {}
    }
    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        loop {}
    }
    fn push_bind_param(&mut self) {
        loop {}
    }
    fn finish(self) -> String {
        loop {}
    }
}
