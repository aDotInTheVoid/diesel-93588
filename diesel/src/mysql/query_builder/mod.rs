use super::backend::Mysql;
use crate::query_builder::QueryBuilder;
use crate::result::QueryResult;
mod limit_offset;
mod query_fragment_impls;
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct MysqlQueryBuilder {
    sql: String,
}
impl MysqlQueryBuilder {
        pub fn new() -> Self {
        loop {}
    }
}
impl QueryBuilder<Mysql> for MysqlQueryBuilder {
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
