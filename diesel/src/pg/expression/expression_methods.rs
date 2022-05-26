pub(in crate::pg) use self::private::{
    ArrayOrNullableArray, InetOrCidr, JsonIndex,
    JsonOrNullableJsonOrJsonbOrNullableJsonb, JsonRemoveIndex, JsonbOrNullableJsonb,
    RangeHelper, RangeOrNullableRange, TextOrNullableText,
};
use super::date_and_time::{AtTimeZone, DateTimeLike};
use super::operators::*;
use crate::dsl;
use crate::expression::grouped::Grouped;
use crate::expression::operators::{Asc, Desc};
use crate::expression::{AsExpression, Expression, IntoSql, TypedExpressionType};
use crate::pg::expression::expression_methods::private::BinaryOrNullableBinary;
use crate::sql_types::{Array, Binary, Inet, Integer, Jsonb, SqlType, Text, VarChar};
use crate::EscapeExpressionMethods;
#[cfg(feature = "postgres_backend")]
pub trait PgExpressionMethods: Expression + Sized {
    #[allow(clippy::wrong_self_convention)]
    fn is_not_distinct_from<T>(self, other: T) -> dsl::IsNotDistinctFrom<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(IsNotDistinctFrom::new(self, other.as_expression()))
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_distinct_from<T>(self, other: T) -> dsl::IsDistinctFrom<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(IsDistinctFrom::new(self, other.as_expression()))
    }
}
impl<T: Expression> PgExpressionMethods for T {}
#[cfg(feature = "postgres_backend")]
pub trait PgTimestampExpressionMethods: Expression + Sized {
    fn at_time_zone<T>(self, timezone: T) -> dsl::AtTimeZone<Self, T>
    where
        T: AsExpression<VarChar>,
    {
        Grouped(AtTimeZone::new(self, timezone.as_expression()))
    }
}
impl<T: Expression> PgTimestampExpressionMethods for T
where
    T::SqlType: DateTimeLike,
{}
#[cfg(feature = "postgres_backend")]
pub trait PgArrayExpressionMethods: Expression + Sized {
    fn overlaps_with<T>(self, other: T) -> dsl::OverlapsWith<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(OverlapsWith::new(self, other.as_expression()))
    }
    fn contains<T>(self, other: T) -> dsl::ArrayContains<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(Contains::new(self, other.as_expression()))
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_contained_by<T>(self, other: T) -> dsl::IsContainedBy<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Self::SqlType>,
    {
        Grouped(IsContainedBy::new(self, other.as_expression()))
    }
    fn index<T>(self, other: T) -> dsl::ArrayIndex<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Integer>,
    {
        ArrayIndex::new(self, other.as_expression())
    }
}
impl<T> PgArrayExpressionMethods for T
where
    T: Expression,
    T::SqlType: ArrayOrNullableArray,
{}
#[cfg(feature = "postgres_backend")]
pub trait PgSortExpressionMethods: Sized {
    fn nulls_first(self) -> dsl::NullsFirst<Self> {
        NullsFirst::new(self)
    }
    fn nulls_last(self) -> dsl::NullsLast<Self> {
        NullsLast::new(self)
    }
}
impl<T> PgSortExpressionMethods for Asc<T> {}
impl<T> PgSortExpressionMethods for Desc<T> {}
#[cfg(feature = "postgres_backend")]
pub trait PgTextExpressionMethods: Expression + Sized {
    fn ilike<T>(self, other: T) -> dsl::ILike<Self, T>
    where
        T: AsExpression<Text>,
    {
        Grouped(ILike::new(self, other.as_expression()))
    }
    fn not_ilike<T>(self, other: T) -> dsl::NotILike<Self, T>
    where
        T: AsExpression<Text>,
    {
        Grouped(NotILike::new(self, other.as_expression()))
    }
    fn similar_to<T>(self, other: T) -> dsl::SimilarTo<Self, T>
    where
        T: AsExpression<Text>,
    {
        Grouped(SimilarTo::new(self, other.as_expression()))
    }
    fn not_similar_to<T>(self, other: T) -> dsl::NotSimilarTo<Self, T>
    where
        T: AsExpression<Text>,
    {
        Grouped(NotSimilarTo::new(self, other.as_expression()))
    }
}
impl<T> PgTextExpressionMethods for T
where
    T: Expression,
    T::SqlType: TextOrNullableText,
{}
impl<T, U> EscapeExpressionMethods for Grouped<ILike<T, U>> {
    type TextExpression = ILike<T, U>;
    fn escape(self, character: char) -> dsl::Escape<Self> {
        loop {}
    }
}
impl<T, U> EscapeExpressionMethods for Grouped<NotILike<T, U>> {
    type TextExpression = NotILike<T, U>;
    fn escape(self, character: char) -> dsl::Escape<Self> {
        loop {}
    }
}
impl<T, U> EscapeExpressionMethods for Grouped<SimilarTo<T, U>> {
    type TextExpression = SimilarTo<T, U>;
    fn escape(self, character: char) -> dsl::Escape<Self> {
        loop {}
    }
}
impl<T, U> EscapeExpressionMethods for Grouped<NotSimilarTo<T, U>> {
    type TextExpression = NotSimilarTo<T, U>;
    fn escape(self, character: char) -> dsl::Escape<Self> {
        loop {}
    }
}
#[cfg(feature = "postgres_backend")]
pub trait PgRangeExpressionMethods: Expression + Sized {
    fn contains<T>(self, other: T) -> dsl::RangeContains<Self, T>
    where
        Self::SqlType: RangeHelper,
        <Self::SqlType as RangeHelper>::Inner: SqlType + TypedExpressionType,
        T: AsExpression<<Self::SqlType as RangeHelper>::Inner>,
    {
        Grouped(Contains::new(self, other.as_expression()))
    }
}
impl<T> PgRangeExpressionMethods for T
where
    T: Expression,
    T::SqlType: RangeOrNullableRange,
{}
#[cfg(feature = "postgres_backend")]
pub trait PgNetExpressionMethods: Expression + Sized {
    fn contains<T>(self, other: T) -> dsl::ContainsNet<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(ContainsNet::new(self, other.as_expression()))
    }
    fn contains_or_eq<T>(self, other: T) -> dsl::ContainsNetLoose<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(ContainsNetLoose::new(self, other.as_expression()))
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_contained_by<T>(self, other: T) -> dsl::IsContainedByNet<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(IsContainedByNet::new(self, other.as_expression()))
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_contained_by_or_eq<T>(self, other: T) -> dsl::IsContainedByNetLoose<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(IsContainedByNetLoose::new(self, other.as_expression()))
    }
    fn overlaps_with<T>(self, other: T) -> dsl::OverlapsWithNet<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(OverlapsWith::new(self, other.as_expression()))
    }
    fn and<T>(self, other: T) -> dsl::AndNet<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(AndNet::new(self, other.as_expression()))
    }
    fn or<T>(self, other: T) -> dsl::OrNet<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(OrNet::new(self, other.as_expression()))
    }
    fn diff<T>(self, other: T) -> dsl::DifferenceNet<Self, T>
    where
        T: AsExpression<Inet>,
    {
        Grouped(DifferenceNet::new(self, other.as_expression()))
    }
}
impl<T> PgNetExpressionMethods for T
where
    T: Expression,
    T::SqlType: InetOrCidr,
{}
#[cfg(feature = "postgres_backend")]
pub trait PgJsonbExpressionMethods: Expression + Sized {
    fn concat<T>(self, other: T) -> dsl::ConcatJsonb<Self, T>
    where
        T: AsExpression<Jsonb>,
    {
        Grouped(ConcatJsonb::new(self, other.as_expression()))
    }
    fn has_key<T>(self, other: T) -> dsl::HasKeyJsonb<Self, T>
    where
        T: AsExpression<VarChar>,
    {
        Grouped(HasKeyJsonb::new(self, other.as_expression()))
    }
    fn has_any_key<T>(self, other: T) -> dsl::HasAnyKeyJsonb<Self, T>
    where
        T: AsExpression<Array<VarChar>>,
    {
        Grouped(HasAnyKeyJsonb::new(self, other.as_expression()))
    }
    fn has_all_keys<T>(self, other: T) -> dsl::HasAllKeysJsonb<Self, T>
    where
        T: AsExpression<Array<VarChar>>,
    {
        Grouped(HasAllKeysJsonb::new(self, other.as_expression()))
    }
    fn contains<T>(self, other: T) -> dsl::ContainsJsonb<Self, T>
    where
        T: AsExpression<Jsonb>,
    {
        Grouped(ContainsJsonb::new(self, other.as_expression()))
    }
    #[allow(clippy::wrong_self_convention)]
    fn is_contained_by<T>(self, other: T) -> dsl::IsContainedByJsonb<Self, T>
    where
        T: AsExpression<Jsonb>,
    {
        Grouped(IsContainedByJsonb::new(self, other.as_expression()))
    }
    fn remove<T>(
        self,
        other: T,
    ) -> dsl::RemoveFromJsonb<
        Self,
        T::Expression,
        <T::Expression as Expression>::SqlType,
    >
    where
        T: JsonRemoveIndex,
        <T::Expression as Expression>::SqlType: SqlType,
    {
        Grouped(RemoveFromJsonb::new(self, other.into_json_index_expression()))
    }
    fn remove_by_path<T>(
        self,
        other: T,
    ) -> dsl::RemoveByPathFromJsonb<Self, T::Expression>
    where
        T: AsExpression<Array<Text>>,
    {
        Grouped(RemoveByPathFromJsonb::new(self, other.as_expression()))
    }
}
impl<T> PgJsonbExpressionMethods for T
where
    T: Expression,
    T::SqlType: JsonbOrNullableJsonb,
{}
#[cfg(feature = "postgres_backend")]
pub trait PgAnyJsonExpressionMethods: Expression + Sized {
    fn retrieve_as_object<T>(
        self,
        other: T,
    ) -> dsl::RetrieveAsObjectJson<
        Self,
        T::Expression,
        <T::Expression as Expression>::SqlType,
    >
    where
        T: JsonIndex,
        <T::Expression as Expression>::SqlType: SqlType,
    {
        Grouped(RetrieveAsObjectJson::new(self, other.into_json_index_expression()))
    }
    fn retrieve_as_text<T>(
        self,
        other: T,
    ) -> dsl::RetrieveAsTextJson<
        Self,
        T::Expression,
        <T::Expression as Expression>::SqlType,
    >
    where
        T: JsonIndex,
        <T::Expression as Expression>::SqlType: SqlType,
    {
        Grouped(RetrieveAsTextJson::new(self, other.into_json_index_expression()))
    }
    fn retrieve_by_path_as_object<T>(
        self,
        other: T,
    ) -> dsl::RetrieveByPathAsObjectJson<Self, T::Expression>
    where
        T: AsExpression<Array<Text>>,
    {
        Grouped(RetrieveByPathAsObjectJson::new(self, other.as_expression()))
    }
    fn retrieve_by_path_as_text<T>(
        self,
        other: T,
    ) -> dsl::RetrieveByPathAsTextJson<Self, T::Expression>
    where
        T: AsExpression<Array<Text>>,
    {
        Grouped(RetrieveByPathAsTextJson::new(self, other.as_expression()))
    }
}
#[doc(hidden)]
impl<T> PgAnyJsonExpressionMethods for T
where
    T: Expression,
    T::SqlType: JsonOrNullableJsonOrJsonbOrNullableJsonb,
{}
#[cfg(feature = "postgres_backend")]
pub trait PgBinaryExpressionMethods: Expression + Sized {
    fn concat<T>(self, other: T) -> dsl::ConcatBinary<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Binary>,
    {
        Grouped(ConcatBinary::new(self, other.as_expression()))
    }
    fn like<T>(self, other: T) -> dsl::LikeBinary<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Binary>,
    {
        Grouped(LikeBinary::new(self, other.as_expression()))
    }
    fn not_like<T>(self, other: T) -> dsl::NotLikeBinary<Self, T>
    where
        Self::SqlType: SqlType,
        T: AsExpression<Binary>,
    {
        Grouped(NotLikeBinary::new(self, other.as_expression()))
    }
}
#[doc(hidden)]
impl<T> PgBinaryExpressionMethods for T
where
    T: Expression,
    T::SqlType: BinaryOrNullableBinary,
{}
mod private {
    use crate::sql_types::{
        Array, Binary, Cidr, Inet, Integer, Json, Jsonb, Nullable, Range, SqlType, Text,
    };
    use crate::{Expression, IntoSql};
    pub trait ArrayOrNullableArray {}
    impl<T> ArrayOrNullableArray for Array<T> {}
    impl<T> ArrayOrNullableArray for Nullable<Array<T>> {}
    pub trait InetOrCidr {}
    impl InetOrCidr for Inet {}
    impl InetOrCidr for Cidr {}
    impl InetOrCidr for Nullable<Inet> {}
    impl InetOrCidr for Nullable<Cidr> {}
    pub trait TextOrNullableText {}
    impl TextOrNullableText for Text {}
    impl TextOrNullableText for Nullable<Text> {}
    pub trait RangeHelper: SqlType {
        type Inner;
    }
    impl<ST> RangeHelper for Range<ST>
    where
        Self: 'static,
    {
        type Inner = ST;
    }
    pub trait RangeOrNullableRange {}
    impl<ST> RangeOrNullableRange for Range<ST> {}
    impl<ST> RangeOrNullableRange for Nullable<Range<ST>> {}
    pub trait JsonbOrNullableJsonb {}
    impl JsonbOrNullableJsonb for Jsonb {}
    impl JsonbOrNullableJsonb for Nullable<Jsonb> {}
    pub trait JsonRemoveIndex {
        type Expression: Expression;
        fn into_json_index_expression(self) -> Self::Expression;
    }
    impl<'a> JsonRemoveIndex for &'a str {
        type Expression = crate::dsl::AsExprOf<&'a str, crate::sql_types::Text>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl JsonRemoveIndex for String {
        type Expression = crate::dsl::AsExprOf<String, crate::sql_types::Text>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl JsonRemoveIndex for Vec<String> {
        type Expression = crate::dsl::AsExprOf<Self, Array<Text>>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl<'a> JsonRemoveIndex for Vec<&'a str> {
        type Expression = crate::dsl::AsExprOf<Self, Array<Text>>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl<'a> JsonRemoveIndex for &'a [&'a str] {
        type Expression = crate::dsl::AsExprOf<Self, Array<Text>>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl JsonRemoveIndex for i32 {
        type Expression = crate::dsl::AsExprOf<i32, crate::sql_types::Int4>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl<T> JsonRemoveIndex for T
    where
        T: Expression,
        T::SqlType: TextArrayOrTextOrInteger,
    {
        type Expression = Self;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    pub trait TextArrayOrTextOrInteger {}
    impl TextArrayOrTextOrInteger for Array<Text> {}
    impl TextArrayOrTextOrInteger for Text {}
    impl TextArrayOrTextOrInteger for Integer {}
    pub trait JsonOrNullableJsonOrJsonbOrNullableJsonb {}
    impl JsonOrNullableJsonOrJsonbOrNullableJsonb for Json {}
    impl JsonOrNullableJsonOrJsonbOrNullableJsonb for Nullable<Json> {}
    impl JsonOrNullableJsonOrJsonbOrNullableJsonb for Jsonb {}
    impl JsonOrNullableJsonOrJsonbOrNullableJsonb for Nullable<Jsonb> {}
    pub trait JsonIndex {
        type Expression: Expression;
        fn into_json_index_expression(self) -> Self::Expression;
    }
    impl<'a> JsonIndex for &'a str {
        type Expression = crate::dsl::AsExprOf<&'a str, crate::sql_types::Text>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl JsonIndex for String {
        type Expression = crate::dsl::AsExprOf<String, crate::sql_types::Text>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl JsonIndex for i32 {
        type Expression = crate::dsl::AsExprOf<i32, crate::sql_types::Int4>;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    impl<T> JsonIndex for T
    where
        T: Expression,
        T::SqlType: TextOrInteger,
    {
        type Expression = Self;
        fn into_json_index_expression(self) -> Self::Expression {
            loop {}
        }
    }
    pub trait TextOrInteger {}
    impl TextOrInteger for Text {}
    impl TextOrInteger for Integer {}
    pub trait BinaryOrNullableBinary {}
    impl BinaryOrNullableBinary for Binary {}
    impl BinaryOrNullableBinary for Nullable<Binary> {}
}
