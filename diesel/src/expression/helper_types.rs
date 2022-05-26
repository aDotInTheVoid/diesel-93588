use super::array_comparison::{AsInExpression, In, NotIn};
use super::grouped::Grouped;
use super::select_by::SelectBy;
use super::{AsExpression, Expression};
use crate::sql_types;
pub type SqlTypeOf<Expr> = <Expr as Expression>::SqlType;
pub type AsExpr<Item, TargetExpr> = AsExprOf<Item, SqlTypeOf<TargetExpr>>;
pub type AsExprOf<Item, Type> = <Item as AsExpression<Type>>::Expression;
pub type Eq<Lhs, Rhs> = Grouped<super::operators::Eq<Lhs, AsExpr<Rhs, Lhs>>>;
pub type NotEq<Lhs, Rhs> = Grouped<super::operators::NotEq<Lhs, AsExpr<Rhs, Lhs>>>;
pub type EqAny<Lhs, Rhs> = Grouped<
    In<Lhs, <Rhs as AsInExpression<SqlTypeOf<Lhs>>>::InExpression>,
>;
pub type NeAny<Lhs, Rhs> = Grouped<
    NotIn<Lhs, <Rhs as AsInExpression<SqlTypeOf<Lhs>>>::InExpression>,
>;
pub type IsNull<Expr> = Grouped<super::operators::IsNull<Expr>>;
pub type IsNotNull<Expr> = Grouped<super::operators::IsNotNull<Expr>>;
pub type Gt<Lhs, Rhs> = Grouped<super::operators::Gt<Lhs, AsExpr<Rhs, Lhs>>>;
pub type GtEq<Lhs, Rhs> = Grouped<super::operators::GtEq<Lhs, AsExpr<Rhs, Lhs>>>;
pub type Lt<Lhs, Rhs> = Grouped<super::operators::Lt<Lhs, AsExpr<Rhs, Lhs>>>;
pub type LtEq<Lhs, Rhs> = Grouped<super::operators::LtEq<Lhs, AsExpr<Rhs, Lhs>>>;
pub type Between<Lhs, Lower, Upper> = Grouped<
    super::operators::Between<
        Lhs,
        super::operators::And<AsExpr<Lower, Lhs>, AsExpr<Upper, Lhs>>,
    >,
>;
pub type NotBetween<Lhs, Lower, Upper> = Grouped<
    super::operators::NotBetween<
        Lhs,
        super::operators::And<AsExpr<Lower, Lhs>, AsExpr<Upper, Lhs>>,
    >,
>;
pub type Concat<Lhs, Rhs> = Grouped<super::operators::Concat<Lhs, AsExpr<Rhs, Lhs>>>;
pub type Desc<Expr> = super::operators::Desc<Expr>;
pub type Asc<Expr> = super::operators::Asc<Expr>;
pub type Nullable<Expr> = super::nullable::Nullable<Expr>;
pub type AssumeNotNull<Expr> = super::assume_not_null::AssumeNotNull<Expr>;
pub type And<Lhs, Rhs, ST = sql_types::Bool> = Grouped<
    super::operators::And<Lhs, AsExprOf<Rhs, ST>>,
>;
pub type Or<Lhs, Rhs, ST = sql_types::Bool> = Grouped<
    super::operators::Or<Lhs, AsExprOf<Rhs, ST>>,
>;
pub type Escape<Lhs> = Grouped<
    super::operators::Escape<
        <Lhs as crate::expression_methods::EscapeExpressionMethods>::TextExpression,
        AsExprOf<String, sql_types::VarChar>,
    >,
>;
pub type Like<Lhs, Rhs> = Grouped<
    super::operators::Like<Lhs, AsExprOf<Rhs, SqlTypeOf<Lhs>>>,
>;
pub type NotLike<Lhs, Rhs> = Grouped<
    super::operators::NotLike<Lhs, AsExprOf<Rhs, SqlTypeOf<Lhs>>>,
>;
pub type AsSelect<Source, DB> = SelectBy<Source, DB>;
#[doc(inline)]
#[allow(unreachable_pub)]
pub use super::functions::helper_types::*;
#[doc(inline)]
#[cfg(feature = "postgres_backend")]
#[allow(unreachable_pub)]
pub use crate::pg::expression::helper_types::*;
#[doc(inline)]
#[cfg(feature = "sqlite")]
#[allow(unreachable_pub)]
pub use crate::sqlite::expression::helper_types::*;
