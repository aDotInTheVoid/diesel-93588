use crate::dsl::AsExpr;
use crate::expression::grouped::Grouped;
pub type Is<Lhs, Rhs> = Grouped<super::operators::Is<Lhs, AsExpr<Rhs, Lhs>>>;
pub type IsNot<Lhs, Rhs> = Grouped<super::operators::IsNot<Lhs, AsExpr<Rhs, Lhs>>>;
