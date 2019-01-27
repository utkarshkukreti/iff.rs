#[macro_export]
macro_rules! iff {
    (let $pat:pat = $expr:expr, $($tt:tt)+) => {
        if let $pat = $expr {
            iff!($($tt)+)
        }
    };
    (let $pat:pat = $expr:expr => $block:block) => {
        if let $pat = $expr {
            $block
        }
    };
    ($expr:expr, $($tt:tt)+) => {
        if $expr {
            iff!($($tt)+)
        }
    };
    ($expr:expr => $block:block) => {
        if $expr {
            $block
        }
    };
}
