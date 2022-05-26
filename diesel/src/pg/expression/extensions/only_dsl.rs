use crate::query_builder::Only;
use crate::Table;
pub trait OnlyDsl: Table {
        fn only(self) -> Only<Self> {
        Only { source: self }
    }
}
impl<T: Table> OnlyDsl for T {}
