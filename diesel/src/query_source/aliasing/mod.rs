mod alias;
mod aliased_field;
mod dsl_impls;
mod field_alias_mapper;
mod joins;
mod macros;
#[allow(unreachable_pub)]
pub use alias::Alias;
#[allow(unreachable_pub)]
#[doc(hidden)]
pub use alias::{
    AliasAliasAppearsInFromClause, AliasAliasAppearsInFromClauseSameTable,
    AliasAppearsInFromClause,
};
#[allow(unreachable_pub)]
pub use aliased_field::AliasedField;
#[allow(unreachable_pub)]
#[doc(hidden)]
pub use field_alias_mapper::{
    FieldAliasMapper, FieldAliasMapperAssociatedTypesDisjointnessTrick,
};
pub trait AliasSource {
    const NAME: &'static str;
    type Target;
    fn target(&self) -> &Self::Target;
}
