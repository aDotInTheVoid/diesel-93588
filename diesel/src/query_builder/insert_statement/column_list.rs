use crate::backend::Backend;
use crate::query_builder::*;
use crate::query_source::Column;
use crate::result::QueryResult;
pub trait ColumnList {
        type Table;
                fn walk_ast<DB: Backend>(&self, out: AstPass<'_, '_, DB>) -> QueryResult<()>;
}
impl<C> ColumnList for C
where
    C: Column,
{
    type Table = <C as Column>::Table;
    fn walk_ast<DB: Backend>(&self, mut out: AstPass<'_, '_, DB>) -> QueryResult<()> {
        loop {}
    }
}
