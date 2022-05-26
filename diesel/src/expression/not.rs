use crate::expression::grouped::Grouped;
use crate::expression::Expression;
use crate::helper_types::not;
use crate::sql_types::BoolOrNullableBool;
pub fn not<T>(expr: T) -> not<T>
where
    T: Expression,
    <T as Expression>::SqlType: BoolOrNullableBool,
{
    loop {}
}
