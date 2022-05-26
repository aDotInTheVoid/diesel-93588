use super::methods::LimitDsl;
use crate::dsl::Limit;
use crate::expression::grouped::Grouped;
use crate::expression::subselect::Subselect;
use crate::query_builder::SelectQuery;
use crate::sql_types::IntoNullable;
pub trait SingleValueDsl {
        type Output;
        fn single_value(self) -> Self::Output;
}
impl<T> SingleValueDsl for T
where
    Self: SelectQuery + LimitDsl,
    <Self as SelectQuery>::SqlType: IntoNullable,
{
    type Output = Grouped<
        Subselect<
            Limit<Self>,
            <<Self as SelectQuery>::SqlType as IntoNullable>::Nullable,
        >,
    >;
    fn single_value(self) -> Self::Output {
        loop {}
    }
}
