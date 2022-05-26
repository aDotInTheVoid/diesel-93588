use crate::expression::functions::sql_function;
use crate::sql_types::Foldable;
sql_function! {
    #[doc =
    " Represents a SQL `SUM` function. This function can only take types which are"]
    #[doc = " Foldable."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```rust"] #[doc = " # include!(\"../../doctest_setup.rs\");"] #[doc =
    " # use diesel::dsl::*;"] #[doc = " #"] #[doc = " # fn main() {"] #[doc =
    " #     use schema::animals::dsl::*;"] #[doc =
    " #     let connection = &mut establish_connection();"] #[doc =
    " assert_eq!(Ok(Some(12i64)), animals.select(sum(legs)).first(connection));"] #[doc =
    " # }"] #[doc = " ```"] #[aggregate] fn sum < ST : Foldable > (expr : ST) -> ST::Sum;
}
sql_function! {
    #[doc =
    " Represents a SQL `AVG` function. This function can only take types which are"]
    #[doc = " Foldable."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```rust"] #[doc = " # include!(\"../../doctest_setup.rs\");"] #[doc =
    " # use diesel::dsl::*;"] #[doc = " # #[cfg(feature = \"bigdecimal\")]"] #[doc =
    " # extern crate bigdecimal;"] #[doc = " #"] #[doc = " # fn main() {"] #[doc =
    " #     run_test().unwrap();"] #[doc = " # }"] #[doc = " #"] #[doc = " # table! {"]
    #[doc = " #     numbers (number) {"] #[doc = " #         number -> Integer,"] #[doc =
    " #     }"] #[doc = " # }"] #[doc = " #"] #[doc =
    " # #[cfg(all(feature = \"numeric\", any(feature = \"postgres\", not(feature = \"sqlite\"))))]"]
    #[doc = " # fn run_test() -> QueryResult<()> {"] #[doc =
    " #     use bigdecimal::BigDecimal;"] #[doc = " #     use self::numbers::dsl::*;"]
    #[doc = " #     let conn = &mut establish_connection();"] #[doc =
    " #     diesel::sql_query(\"DROP TABLE IF EXISTS numbers\").execute(conn)?;"] #[doc =
    " #     diesel::sql_query(\"CREATE TABLE numbers (number INTEGER)\").execute(conn)?;"]
    #[doc = " diesel::insert_into(numbers)"] #[doc =
    "     .values(&vec![number.eq(1), number.eq(2)])"] #[doc = "     .execute(conn)?;"]
    #[doc = " let average = numbers.select(avg(number)).get_result(conn)?;"] #[doc =
    " let expected = \"1.5\".parse::<BigDecimal>().unwrap();"] #[doc =
    " assert_eq!(Some(expected), average);"] #[doc = " #     Ok(())"] #[doc = " # }"]
    #[doc = " #"] #[doc =
    " # #[cfg(not(all(feature = \"numeric\", any(feature = \"postgres\", not(feature = \"sqlite\")))))]"]
    #[doc = " # fn run_test() -> QueryResult<()> {"] #[doc = " #     Ok(())"] #[doc =
    " # }"] #[aggregate] fn avg < ST : Foldable > (expr : ST) -> ST::Avg;
}
