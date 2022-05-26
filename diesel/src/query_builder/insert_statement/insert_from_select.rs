use crate::backend::Backend;
use crate::expression::{Expression, NonAggregate, SelectableExpression};
use crate::insertable::*;
use crate::query_builder::*;
use crate::query_source::Table;
#[derive(Debug, Clone, Copy, QueryId)]
pub struct InsertFromSelect<Select, Columns> {
    pub(in crate::query_builder) query: Select,
    pub(in crate::query_builder) columns: Columns,
}
impl<Select, Columns> InsertFromSelect<Select, Columns> {
            pub fn new<T>(query: Select) -> Self
    where
        T: Table<AllColumns = Columns>,
        Columns: SelectableExpression<T> + NonAggregate,
    {
        loop {}
    }
        pub fn with_columns<C>(self, columns: C) -> InsertFromSelect<Select, C> {
        loop {}
    }
}
impl<DB, Select, Columns> CanInsertInSingleQuery<DB>
for InsertFromSelect<Select, Columns>
where
    DB: Backend,
{
    fn rows_to_insert(&self) -> Option<usize> {
        loop {}
    }
}
impl<DB, Select, Columns> QueryFragment<DB> for InsertFromSelect<Select, Columns>
where
    DB: Backend,
    Columns: ColumnList + Expression,
    Select: Query<SqlType = Columns::SqlType> + QueryFragment<DB>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        loop {}
    }
}
impl<Select, Columns> UndecoratedInsertRecord<Columns::Table>
for InsertFromSelect<Select, Columns>
where
    Columns: ColumnList + Expression,
    Select: Query<SqlType = Columns::SqlType>,
{}
