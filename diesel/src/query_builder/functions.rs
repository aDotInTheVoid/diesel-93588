use super::delete_statement::DeleteStatement;
use super::distinct_clause::NoDistinctClause;
use super::insert_statement::{Insert, InsertOrIgnore, Replace};
use super::select_clause::SelectClause;
use super::{
    AsQuery, IncompleteInsertOrIgnoreStatement, IncompleteInsertStatement,
    IncompleteReplaceStatement, IntoUpdateTarget, SelectStatement, SqlQuery, UpdateStatement,
};
use crate::expression::Expression;
use crate::Table;
pub fn update<T: IntoUpdateTarget>(source: T) -> UpdateStatement<T::Table, T::WhereClause> {
    loop {}
}
pub fn delete<T: IntoUpdateTarget>(source: T) -> DeleteStatement<T::Table, T::WhereClause> {
    loop {}
}
pub fn insert_into<T: Table>(target: T) -> IncompleteInsertStatement<T> {
    loop {}
}
pub fn insert_or_ignore_into<T: Table>(target: T) -> IncompleteInsertOrIgnoreStatement<T> {
    loop {}
}
pub fn select<T>(expression: T) -> crate::dsl::BareSelect<T>
where
    T: Expression,
    crate::dsl::BareSelect<T>: AsQuery,
{
    loop {}
}
pub fn replace_into<T: Table>(target: T) -> IncompleteReplaceStatement<T> {
    loop {}
}
pub fn sql_query<T: Into<String>>(query: T) -> SqlQuery {
    loop {}
}
