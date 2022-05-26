#![allow(non_camel_case_types)]
use crate::dsl::SqlTypeOf;
use crate::expression::grouped::Grouped;
use crate::expression::operators;
pub type not<Expr> = operators::Not<Grouped<Expr>>;
pub type max<Expr> = super::aggregate_ordering::max::HelperType<SqlTypeOf<Expr>, Expr>;
pub type min<Expr> = super::aggregate_ordering::min::HelperType<SqlTypeOf<Expr>, Expr>;
pub type sum<Expr> = super::aggregate_folding::sum::HelperType<SqlTypeOf<Expr>, Expr>;
pub type avg<Expr> = super::aggregate_folding::avg::HelperType<SqlTypeOf<Expr>, Expr>;
pub type exists<Expr> = crate::expression::exists::Exists<Expr>;
