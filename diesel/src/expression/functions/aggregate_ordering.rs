use self::private::SqlOrdAggregate;
use crate::expression::functions::sql_function;
sql_function! {
    #[doc =
    " Represents a SQL `MAX` function. This function can only take types which are"]
    #[doc = " ordered."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```rust"] #[doc = " # include!(\"../../doctest_setup.rs\");"] #[doc =
    " # use diesel::dsl::*;"] #[doc = " #"] #[doc = " # fn main() {"] #[doc =
    " #     use schema::animals::dsl::*;"] #[doc =
    " #     let connection = &mut establish_connection();"] #[doc =
    " assert_eq!(Ok(Some(8)), animals.select(max(legs)).first(connection));"] #[doc =
    " # }"] #[aggregate] fn max < ST : SqlOrdAggregate > (expr : ST) -> ST::Ret;
}
sql_function! {
    #[doc =
    " Represents a SQL `MIN` function. This function can only take types which are"]
    #[doc = " ordered."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```rust"] #[doc = " # include!(\"../../doctest_setup.rs\");"] #[doc =
    " # use diesel::dsl::*;"] #[doc = " #"] #[doc = " # fn main() {"] #[doc =
    " #     use schema::animals::dsl::*;"] #[doc =
    " #     let connection = &mut establish_connection();"] #[doc =
    " assert_eq!(Ok(Some(4)), animals.select(min(legs)).first(connection));"] #[doc =
    " # }"] #[aggregate] fn min < ST : SqlOrdAggregate > (expr : ST) -> ST::Ret;
}
mod private {
    use crate::sql_types::{IntoNullable, SingleValue, SqlOrd, SqlType};
    pub trait SqlOrdAggregate: SingleValue {
        type Ret: SqlType + SingleValue;
    }
    impl<T> SqlOrdAggregate for T
    where
        T: SqlOrd + IntoNullable + SingleValue,
        T::Nullable: SqlType + SingleValue,
    {
        type Ret = T::Nullable;
    }
}
