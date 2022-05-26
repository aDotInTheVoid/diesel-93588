use super::backend::Pg;
use crate::query_builder::QueryBuilder;
use crate::result::QueryResult;
mod distinct_on;
mod limit_offset;
pub(crate) mod on_constraint;
pub(crate) mod only;
mod query_fragment_impls;
pub use self::distinct_on::DistinctOnClause;
#[allow(missing_debug_implementations)]
#[derive(Default)]
#[cfg(feature = "postgres_backend")]
pub struct PgQueryBuilder {
    sql: String,
    bind_idx: u32,
}
impl PgQueryBuilder {
    pub fn new() -> Self {
        loop {}
    }
}
impl QueryBuilder<Pg> for PgQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        loop {}
    }
    fn push_identifier(&mut self, identifier: &str) -> QueryResult<()> {
        loop {}
    }
    fn push_bind_param(&mut self) {
        loop {}
    }
    fn push_bind_param_value_only(&mut self) {
        loop {}
    }
    fn finish(self) -> String {
        loop {}
    }
}
#[test]
fn check_sql_query_increments_bind_count() {
    loop {}
}
