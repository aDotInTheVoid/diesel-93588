use super::{Alias, AliasSource, AliasedField};
use crate::expression;
use crate::query_source::{Column, Table, TableNotEqual};
pub trait FieldAliasMapper<S> {
                    type Out;
        fn map(self, alias: &Alias<S>) -> Self::Out;
}
#[doc(hidden)]
pub trait FieldAliasMapperAssociatedTypesDisjointnessTrick<CT, S, C> {
    type Out;
    fn map(column: C, alias: &Alias<S>) -> Self::Out;
}
impl<S, C> FieldAliasMapper<S> for C
where
    S: AliasSource,
    C: Column,
    S::Target: FieldAliasMapperAssociatedTypesDisjointnessTrick<C::Table, S, C>,
{
    type Out = <S::Target as FieldAliasMapperAssociatedTypesDisjointnessTrick<
        C::Table,
        S,
        C,
    >>::Out;
    fn map(self, alias: &Alias<S>) -> Self::Out {
        loop {}
    }
}
impl<TS, TC, S, C> FieldAliasMapperAssociatedTypesDisjointnessTrick<TC, S, C> for TS
where
    S: AliasSource<Target = TS>,
    C: Column<Table = TC>,
    TC: Table,
    TS: TableNotEqual<TC>,
{
    type Out = C;
    fn map(column: C, _alias: &Alias<S>) -> Self::Out {
        loop {}
    }
}
macro_rules! field_alias_mapper {
    ($($Tuple:tt { $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+ })+) => {
        $(impl < _S, $($T,)*> FieldAliasMapper < _S > for ($($T,)*) where _S :
        AliasSource, $($T : FieldAliasMapper < _S >,)* { type Out = ($(<$T as
        FieldAliasMapper < _S >>::Out,)*); fn map(self, alias : & Alias < _S >) ->
        Self::Out { ($(self.$idx .map(alias),)*) } })*
    };
}
diesel_derives::__diesel_for_each_tuple!(field_alias_mapper);
impl<SPrev, SNew, F> FieldAliasMapper<SNew> for AliasedField<SPrev, F>
where
    SNew: AliasSource,
{
    type Out = Self;
    fn map(self, _alias: &Alias<SNew>) -> Self::Out {
        loop {}
    }
}
impl<S, F> FieldAliasMapper<S> for expression::nullable::Nullable<F>
where
    F: FieldAliasMapper<S>,
{
    type Out = expression::nullable::Nullable<<F as FieldAliasMapper<S>>::Out>;
    fn map(self, alias: &Alias<S>) -> Self::Out {
        loop {}
    }
}
impl<S, F> FieldAliasMapper<S> for expression::grouped::Grouped<F>
where
    F: FieldAliasMapper<S>,
{
    type Out = expression::grouped::Grouped<<F as FieldAliasMapper<S>>::Out>;
    fn map(self, alias: &Alias<S>) -> Self::Out {
        loop {}
    }
}
