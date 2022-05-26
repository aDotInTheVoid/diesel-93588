use super::QueryFragment;
use std::any::{Any, TypeId};
pub trait QueryId {
                                                                        type QueryId: Any;
                                const HAS_STATIC_QUERY_ID: bool = true;
                    fn query_id() -> Option<TypeId> {
        if Self::HAS_STATIC_QUERY_ID {
            Some(TypeId::of::<Self::QueryId>())
        } else {
            None
        }
    }
}
#[doc(inline)]
pub use diesel_derives::QueryId;
impl QueryId for () {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = true;
}
impl<T: QueryId + ?Sized> QueryId for Box<T> {
    type QueryId = T::QueryId;
    const HAS_STATIC_QUERY_ID: bool = T::HAS_STATIC_QUERY_ID;
}
impl<'a, T: QueryId + ?Sized> QueryId for &'a T {
    type QueryId = T::QueryId;
    const HAS_STATIC_QUERY_ID: bool = T::HAS_STATIC_QUERY_ID;
}
impl<DB> QueryId for dyn QueryFragment<DB> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}
#[cfg(test)]
#[allow(unused_parens)]
mod tests {
    use std::any::TypeId;
    use super::QueryId;
    use crate::prelude::*;
    table! {
        users { id -> Integer, name -> VarChar, }
    }
    fn query_id<T: QueryId>(_: T) -> Option<TypeId> {
        loop {}
    }
    #[test]
    fn queries_with_no_dynamic_elements_have_a_static_id() {
        loop {}
    }
    #[test]
    fn queries_with_different_types_have_different_ids() {
        loop {}
    }
    #[test]
    fn bind_params_use_only_sql_type_for_query_id() {
        loop {}
    }
    #[test]
    #[cfg(features = "postgres")]
    fn boxed_queries_do_not_have_static_query_id() {
        loop {}
    }
}
