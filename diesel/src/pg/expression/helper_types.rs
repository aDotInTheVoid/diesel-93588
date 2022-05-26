use crate::dsl::{AsExpr, AsExprOf, SqlTypeOf};
use crate::expression::grouped::Grouped;
use crate::pg::types::sql_types::Array;
use crate::sql_types::{Binary, Inet, Integer, Jsonb, VarChar};
#[cfg(feature = "postgres_backend")]
pub type ILike<Lhs, Rhs> = Grouped<super::operators::ILike<Lhs, AsExprOf<Rhs, VarChar>>>;
#[cfg(feature = "postgres_backend")]
pub type NotILike<Lhs, Rhs> = Grouped<
    super::operators::NotILike<Lhs, AsExprOf<Rhs, VarChar>>,
>;
#[cfg(feature = "postgres_backend")]
pub type SimilarTo<Lhs, Rhs> = Grouped<
    super::operators::SimilarTo<Lhs, AsExprOf<Rhs, VarChar>>,
>;
#[cfg(feature = "postgres_backend")]
pub type NotSimilarTo<Lhs, Rhs> = Grouped<
    super::operators::NotSimilarTo<Lhs, AsExprOf<Rhs, VarChar>>,
>;
#[cfg(feature = "postgres_backend")]
pub type IsNotDistinctFrom<Lhs, Rhs> = Grouped<
    super::operators::IsNotDistinctFrom<Lhs, AsExpr<Rhs, Lhs>>,
>;
#[cfg(feature = "postgres_backend")]
pub type IsDistinctFrom<Lhs, Rhs> = Grouped<
    super::operators::IsDistinctFrom<Lhs, AsExpr<Rhs, Lhs>>,
>;
#[cfg(feature = "postgres_backend")]
pub type OverlapsWith<Lhs, Rhs> = Grouped<
    super::operators::OverlapsWith<Lhs, AsExpr<Rhs, Lhs>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ArrayContains<Lhs, Rhs> = Grouped<
    super::operators::Contains<Lhs, AsExpr<Rhs, Lhs>>,
>;
#[cfg(feature = "postgres_backend")]
pub type RangeContains<Lhs, Rhs> = Grouped<
    super::operators::Contains<
        Lhs,
        AsExprOf<Rhs, <SqlTypeOf<Lhs> as super::expression_methods::RangeHelper>::Inner>,
    >,
>;
#[cfg(feature = "postgres_backend")]
pub type IsContainedBy<Lhs, Rhs> = Grouped<
    super::operators::IsContainedBy<Lhs, AsExpr<Rhs, Lhs>>,
>;
#[cfg(feature = "postgres_backend")]
pub type NullsFirst<T> = super::operators::NullsFirst<T>;
#[cfg(feature = "postgres_backend")]
pub type NullsLast<T> = super::operators::NullsLast<T>;
#[cfg(feature = "postgres_backend")]
pub type AtTimeZone<Lhs, Rhs> = Grouped<
    super::date_and_time::AtTimeZone<Lhs, AsExprOf<Rhs, VarChar>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ContainsNet<Lhs, Rhs> = Grouped<
    super::operators::ContainsNet<Lhs, AsExprOf<Rhs, Inet>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ContainsNetLoose<Lhs, Rhs> = Grouped<
    super::operators::ContainsNetLoose<Lhs, AsExprOf<Rhs, Inet>>,
>;
#[cfg(feature = "postgres_backend")]
pub type IsContainedByNet<Lhs, Rhs> = Grouped<
    super::operators::IsContainedByNet<Lhs, AsExprOf<Rhs, Inet>>,
>;
#[cfg(feature = "postgres_backend")]
pub type IsContainedByNetLoose<Lhs, Rhs> = Grouped<
    super::operators::IsContainedByNetLoose<Lhs, AsExprOf<Rhs, Inet>>,
>;
#[cfg(feature = "postgres_backend")]
pub type OverlapsWithNet<Lhs, Rhs> = Grouped<
    super::operators::OverlapsWith<Lhs, AsExprOf<Rhs, Inet>>,
>;
#[cfg(feature = "postgres_backend")]
pub type AndNet<Lhs, Rhs> = Grouped<super::operators::AndNet<Lhs, AsExprOf<Rhs, Inet>>>;
#[cfg(feature = "postgres_backend")]
pub type OrNet<Lhs, Rhs> = Grouped<super::operators::OrNet<Lhs, AsExprOf<Rhs, Inet>>>;
#[cfg(feature = "postgres_backend")]
pub type DifferenceNet<Lhs, Rhs> = Grouped<
    super::operators::DifferenceNet<Lhs, AsExprOf<Rhs, Inet>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ConcatJsonb<Lhs, Rhs> = Grouped<
    super::operators::ConcatJsonb<Lhs, AsExprOf<Rhs, Jsonb>>,
>;
#[cfg(feature = "postgres_backend")]
pub type HasKeyJsonb<Lhs, Rhs> = Grouped<
    super::operators::HasKeyJsonb<Lhs, AsExprOf<Rhs, VarChar>>,
>;
#[cfg(feature = "postgres_backend")]
pub type HasAnyKeyJsonb<Lhs, Rhs> = Grouped<
    super::operators::HasAnyKeyJsonb<Lhs, AsExprOf<Rhs, Array<VarChar>>>,
>;
#[cfg(feature = "postgres_backend")]
pub type HasAllKeysJsonb<Lhs, Rhs> = Grouped<
    super::operators::HasAllKeysJsonb<Lhs, AsExprOf<Rhs, Array<VarChar>>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ContainsJsonb<Lhs, Rhs> = Grouped<
    super::operators::ContainsJsonb<Lhs, AsExprOf<Rhs, Jsonb>>,
>;
#[cfg(feature = "postgres_backend")]
pub type IsContainedByJsonb<Lhs, Rhs> = Grouped<
    super::operators::IsContainedByJsonb<Lhs, AsExprOf<Rhs, Jsonb>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ArrayIndex<Lhs, Rhs> = super::operators::ArrayIndex<
    Lhs,
    AsExprOf<Rhs, Integer>,
>;
#[cfg(feature = "postgres_backend")]
pub type RemoveFromJsonb<Lhs, Rhs, ST> = Grouped<
    super::operators::RemoveFromJsonb<Lhs, AsExprOf<Rhs, ST>>,
>;
#[cfg(feature = "postgres_backend")]
pub type RetrieveAsObjectJson<Lhs, Rhs, ST> = Grouped<
    super::operators::RetrieveAsObjectJson<Lhs, AsExprOf<Rhs, ST>>,
>;
#[cfg(feature = "postgres_backend")]
pub type RetrieveAsTextJson<Lhs, Rhs, ST> = Grouped<
    super::operators::RetrieveAsTextJson<Lhs, AsExprOf<Rhs, ST>>,
>;
#[cfg(feature = "postgres_backend")]
pub type RetrieveByPathAsObjectJson<Lhs, Rhs> = Grouped<
    super::operators::RetrieveByPathAsObjectJson<Lhs, AsExprOf<Rhs, Array<VarChar>>>,
>;
#[cfg(feature = "postgres_backend")]
pub type RetrieveByPathAsTextJson<Lhs, Rhs> = Grouped<
    super::operators::RetrieveByPathAsTextJson<Lhs, AsExprOf<Rhs, Array<VarChar>>>,
>;
#[cfg(feature = "postgres_backend")]
pub type RemoveByPathFromJsonb<Lhs, Rhs> = Grouped<
    super::operators::RemoveByPathFromJsonb<Lhs, AsExprOf<Rhs, Array<VarChar>>>,
>;
#[cfg(feature = "postgres_backend")]
pub type ConcatBinary<Lhs, Rhs> = Grouped<
    super::operators::ConcatBinary<Lhs, AsExprOf<Rhs, Binary>>,
>;
#[cfg(feature = "postgres_backend")]
pub type LikeBinary<Lhs, Rhs> = Grouped<
    super::operators::LikeBinary<Lhs, AsExprOf<Rhs, Binary>>,
>;
#[cfg(feature = "postgres_backend")]
pub type NotLikeBinary<Lhs, Rhs> = Grouped<
    super::operators::NotLikeBinary<Lhs, AsExprOf<Rhs, Binary>>,
>;
