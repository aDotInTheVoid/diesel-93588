#[macro_export]
#[doc(hidden)]
macro_rules! impl_selectable_expression {
    ($struct_name:ident) => {
        $crate ::impl_selectable_expression!(ty_params = (), struct_ty = $struct_name,);
    };
    ($struct_name:ident <$($ty_params:ident),+>) => {
        $crate ::impl_selectable_expression!(ty_params = ($($ty_params),+), struct_ty =
        $struct_name <$($ty_params),+>,);
    };
    (ty_params = ($($ty_params:ident),*), struct_ty = $struct_ty:ty,) => {
        impl <$($ty_params,)* QS > $crate ::expression::SelectableExpression < QS > for
        $struct_ty where $struct_ty : $crate ::expression::AppearsOnTable < QS >,
        $($ty_params : $crate ::expression::SelectableExpression < QS >,)* {} impl
        <$($ty_params,)* QS > $crate ::expression::AppearsOnTable < QS > for $struct_ty
        where $struct_ty : $crate ::expression::Expression, $($ty_params : $crate
        ::expression::AppearsOnTable < QS >,)* {}
    };
}
