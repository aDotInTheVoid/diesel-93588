#![cfg_attr(rustfmt, rustfmt_skip)]
#[macro_export]
#[doc(hidden)]
macro_rules! static_cond {
    (@ go $lhs:tt $rhs:tt $arm1:tt $arm2:tt) => {
        macro_rules! __static_cond { ($lhs $lhs) => $arm1; ($lhs $rhs) => $arm2 }
        __static_cond!($lhs $rhs);
    };
    (if $lhs:tt == $rhs:tt $then:tt) => {
        $crate ::static_cond!(if $lhs == $rhs $then else {});
    };
    (if $lhs:tt != $rhs:tt $then:tt) => {
        $crate ::static_cond!(if $lhs != $rhs $then else {});
    };
    (if $lhs:tt == $rhs:tt $then:tt else $els:tt) => {
        $crate ::static_cond!(@ go $lhs $rhs $then $els);
    };
    (if $lhs:tt != $rhs:tt $then:tt else $els:tt) => {
        $crate ::static_cond!(@ go $lhs $rhs $els $then);
    };
}
