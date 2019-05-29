
#[macro_export]
macro_rules! rpn {
    (@op [$b:expr, $a:expr $(,$stack:expr)*] $op:tt $($rest:tt)*) => {
        rpn!([$a $op $b $(,$stack)*] $($rest)*)
    };

    (@op $stack:tt $op:tt $($rest:tt)*) => {
        compile_error!(concat!("Could not apply operator `", stringify!($op), "` to the current stack: ", stringify!($stack)))
    };

    ($stack:tt + $($rest:tt)*) => {
        rpn!(@op $stack + $($rest)*)
    };

    ($stack:tt - $($rest:tt)*) => {
        rpn!(@op $stack - $($rest)*)
    };

    ($stack:tt * $($rest:tt)*) => {
        rpn!(@op $stack * $($rest)*)
    };

    ($stack:tt / $($rest:tt)*) => {
        rpn!(@op $stack / $($rest)*)
    };

    ([$($stack:expr),*] $num:tt $($rest:tt)*) => {
        rpn!([$num $(,$stack)*] $($rest)*)
    };

    ([$result:expr]) => {
        $result
    };

    ([$($stack:expr),*]) => {
        compile_error!(concat!("Could not find final value for the expression, perhaps you missed an operator? Final stack: ", stringify!([$($stack),*])))
    };

    ($($tokens:tt)*) => {
        rpn!([] $($tokens)*)
    };
}
