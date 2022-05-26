use crate::mysql::Mysql;
use crate::query_builder::insert_statement::DefaultValues;
use crate::query_builder::locking_clause::{
    ForShare, ForUpdate, NoModifier, NoWait, SkipLocked,
};
use crate::query_builder::{AstPass, QueryFragment};
use crate::result::QueryResult;
impl QueryFragment<Mysql> for ForUpdate {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl QueryFragment<Mysql> for ForShare {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl QueryFragment<Mysql> for NoModifier {
    fn walk_ast<'b>(&'b self, _out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl QueryFragment<Mysql> for SkipLocked {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl QueryFragment<Mysql> for NoWait {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
impl QueryFragment<Mysql, crate::mysql::backend::MysqlStyleDefaultValueClause>
for DefaultValues {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
        loop {}
    }
}
