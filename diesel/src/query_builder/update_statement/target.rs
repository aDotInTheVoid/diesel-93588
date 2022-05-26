use crate::associations::{HasTable, Identifiable};
use crate::dsl::Find;
use crate::query_dsl::methods::FindDsl;
use crate::query_source::Table;
#[doc(hidden)]
#[derive(Debug)]
pub struct UpdateTarget<Table, WhereClause> {
    pub table: Table,
    pub where_clause: WhereClause,
}
pub trait IntoUpdateTarget: HasTable {
        type WhereClause;
        fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause>;
}
impl<T, Tab, V> IntoUpdateTarget for T
where
    T: Identifiable<Table = Tab>,
    Tab: Table + FindDsl<T::Id>,
    Find<Tab, T::Id>: IntoUpdateTarget<Table = Tab, WhereClause = V>,
{
    type WhereClause = V;
    fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
        loop {}
    }
}
