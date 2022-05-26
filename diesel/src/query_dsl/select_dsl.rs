use crate::expression::Expression;
use crate::query_source::Table;
pub trait SelectDsl<Selection: Expression> {
        type Output;
        fn select(self, selection: Selection) -> Self::Output;
}
impl<T, Selection> SelectDsl<Selection> for T
where
    Selection: Expression,
    T: Table,
    T::Query: SelectDsl<Selection>,
{
    type Output = <T::Query as SelectDsl<Selection>>::Output;
    fn select(self, selection: Selection) -> Self::Output {
        loop {}
    }
}
