mod belongs_to;
pub use self::belongs_to::{BelongsTo, GroupedBy};
use crate::query_source::Table;
#[doc(inline)]
pub use diesel_derives::Associations;
use std::hash::Hash;
pub trait HasTable {
    type Table: Table;
    fn table() -> Self::Table;
}
impl<'a, T: HasTable> HasTable for &'a T {
    type Table = T::Table;
    fn table() -> Self::Table {
        loop {}
    }
}
pub trait Identifiable: HasTable {
    type Id: Hash + Eq;
    fn id(self) -> Self::Id;
}
#[doc(inline)]
pub use diesel_derives::Identifiable;
