#[macro_export]
macro_rules! alias {
    ($($($table:ident)::+ as $alias:ident),* $(,)?) => {
        { $crate ::alias!(NoConst $($($table)::+ as $alias : $alias,)*); ($($crate
        ::query_source::Alias::<$alias >::default()),*) }
    };
    ($($($table:ident)::+ as $alias_name:ident : $alias_ty:ident),* $(,)?) => {
        $crate ::alias! { $(pub const $alias_name : Alias <$alias_ty > = $($table)::+ as
        $alias_name;)* }
    };
    (
        $($vis:vis const $const_name:ident : Alias <$alias_ty:ident > =
        $($table:ident)::+ as $alias_sql_name:ident);* $(;)?
    ) => {
        $crate ::alias!(NoConst $($($table)::+ as $alias_sql_name : $vis $alias_ty,)*);
        $(#[allow(non_upper_case_globals)] $vis const $const_name : $crate
        ::query_source::Alias::<$alias_ty > = $crate ::query_source::Alias::new($alias_ty
        { table : $($table)::+:: table });)*
    };
    (
        NoConst $($($table:ident)::+ as $alias_sql_name:ident : $vis:vis
        $alias_ty:ident),* $(,)?
    ) => {
        $(#[allow(non_camel_case_types)] #[derive(Debug, Clone, Copy)] $vis struct
        $alias_ty { table : $($table)::+:: table, } impl $crate
        ::query_source::AliasSource for $alias_ty { const NAME : &'static str =
        stringify!($alias_sql_name); type Target = $($table)::+:: table; fn target(&
        self) -> & Self::Target { & self.table } } impl ::std::default::Default for
        $alias_ty { fn default() -> Self { Self { table : $($table)::+:: table } } } impl
        $crate ::internal::alias_macro::AliasAliasAppearsInFromClauseSameTable
        <$alias_ty, $($table)::+:: table > for $alias_ty { type Count = $crate
        ::query_source::Once; })* $crate ::__internal_alias_helper!($(table_ty =
        $($table)::+:: table, table_tt = ($($table)::+), alias_ty = $alias_ty,
        alias_sql_name = $alias_sql_name;)*);
    };
}
#[macro_export]
#[doc(hidden)]
macro_rules! __internal_alias_helper {
    (
        table_ty = $left_table_ty:ty, table_tt = $left_table_tt:tt, alias_ty =
        $left_alias:ident, alias_sql_name = $left_sql_name:ident; $(table_ty =
        $right_table_ty:ty, table_tt = $right_table_tt:tt, alias_ty = $right_alias:ident,
        alias_sql_name = $right_sql_name:ident;)+
    ) => {
        $($crate ::static_cond! { if ($left_table_tt) == ($right_table_tt) { $crate
        ::static_cond! { if ($left_sql_name) != ($right_sql_name) { impl $crate
        ::internal::alias_macro::AliasAliasAppearsInFromClauseSameTable <$left_alias,
        $left_table_ty > for $right_alias { type Count = $crate ::query_source::Never; }
        impl $crate ::internal::alias_macro::AliasAliasAppearsInFromClauseSameTable
        <$right_alias, $left_table_ty > for $left_alias { type Count = $crate
        ::query_source::Never; } } } } })* $crate ::__internal_alias_helper!($(table_ty =
        $right_table_ty, table_tt = $right_table_tt, alias_ty = $right_alias,
        alias_sql_name = $right_sql_name;)+);
    };
    (
        table_ty = $left_table_ty:ty, table_tt = $left_table_tt:tt, alias_ty =
        $left_alias:ident, alias_sql_name = $left_sql_name:ident;
    ) => {};
    () => {};
}
