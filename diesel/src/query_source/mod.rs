pub(crate) mod aliasing;
pub(crate) mod joins;
mod peano_numbers;
pub use self::aliasing::{Alias, AliasSource, AliasedField};
pub use self::joins::JoinTo;
pub use self::peano_numbers::*;
pub(crate) use self::private::Pick;
use crate::expression::{Expression, SelectableExpression, ValidGrouping};
use crate::query_builder::*;
pub trait QuerySource {
    type FromClause;
    type DefaultSelection: SelectableExpression<Self>;
    #[allow(clippy::wrong_self_convention)]
    fn from_clause(&self) -> Self::FromClause;
    fn default_selection(&self) -> Self::DefaultSelection;
}
pub trait Column: Expression {
    type Table: Table;
    const NAME: &'static str;
}
pub trait Table: QuerySource + AsQuery + Sized {
    type PrimaryKey: SelectableExpression<Self> + ValidGrouping<()>;
    type AllColumns: SelectableExpression<Self> + ValidGrouping<()>;
    fn primary_key(&self) -> Self::PrimaryKey;
    fn all_columns() -> Self::AllColumns;
}
pub trait AppearsInFromClause<QS> {
    type Count;
}
pub trait TableNotEqual<T: Table>: Table {}
impl<T1, T2> AppearsInFromClause<T2> for T1
where
    T1: TableNotEqual<T2> + Table,
    T2: Table,
{
    type Count = Never;
}
pub(crate) mod private {
    use super::{Never, Once};
    #[doc(hidden)]
    pub trait Pick<Left, Right> {
        type Selection;
    }
    impl<Left, Right> Pick<Left, Right> for (Once, Never) {
        type Selection = Left;
    }
    impl<Left, Right> Pick<Left, Right> for (Never, Once) {
        type Selection = Right;
    }
}
#[doc(hidden)]
#[allow(
    non_camel_case_types,
    missing_debug_implementations,
    missing_copy_implementations
)]
mod impls_which_are_only_here_to_improve_error_messages {
    use super::*;
    pub struct this_table_doesnt_appear_in_the_from_clause_of_your_query;
    impl<Left, Right> Pick<Left, Right> for (Never, Never) {
        type Selection = this_table_doesnt_appear_in_the_from_clause_of_your_query;
    }
    pub struct this_table_appears_in_your_query_more_than_once_and_must_be_aliased;
    impl<Left, Right, OtherCount> Pick<Left, Right> for (MoreThanOnce, OtherCount) {
        type Selection = this_table_appears_in_your_query_more_than_once_and_must_be_aliased;
    }
    impl<Left, Right> Pick<Left, Right> for (Never, MoreThanOnce) {
        type Selection = this_table_appears_in_your_query_more_than_once_and_must_be_aliased;
    }
    impl<Left, Right> Pick<Left, Right> for (Once, MoreThanOnce) {
        type Selection = this_table_appears_in_your_query_more_than_once_and_must_be_aliased;
    }
}
