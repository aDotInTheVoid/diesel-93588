use crate::expression::TypedExpressionType;
use crate::expression::ValidGrouping;
use crate::query_builder::AsQuery;
use crate::query_builder::FromClause;
use crate::query_builder::SelectStatement;
use crate::query_source::Table;
use crate::Expression;
pub trait LockingDsl<Lock> {
    type Output;
    fn with_lock(self, lock: Lock) -> Self::Output;
}
impl<T, Lock> LockingDsl<Lock> for T
where
    T: Table + AsQuery<Query = SelectStatement<FromClause<T>>>,
    T::DefaultSelection: Expression<SqlType = T::SqlType> + ValidGrouping<()>,
    T::SqlType: TypedExpressionType,
{
    type Output = <SelectStatement<FromClause<T>> as LockingDsl<Lock>>::Output;
    fn with_lock(self, lock: Lock) -> Self::Output {
        loop {}
    }
}
pub trait ModifyLockDsl<Modifier> {
    type Output;
    fn modify_lock(self, modifier: Modifier) -> Self::Output;
}
