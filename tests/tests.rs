use iff::iff;

macro_rules! t {
    ( $( [ $($tt:tt)+ ] => $expr:expr, )+ ) => {
        $(
            let mut ok = false;
            iff!($($tt)+ => { ok = true });
            assert!(ok == $expr);
        )+
    }
}

#[test]
fn t() {
    t! {
        [1 == 1] => true,
        [1 == 1, 2 == 2] => true,
        [1 == 2] => false,
        [let Some(_) = Some(10)] => true,
        [let Some(x) = Some(10), x == 10] => true,
        [let Some(x) = Some(10), x > 10] => false,
        [let Some(x) = Some(10), x < 10] => false,
        [let Some(x) = Some(Some(10)), let Some(y) = x, y == 10] => true,
        [let Some(x) = Some(Some(10)), let Some(y) = x, y > 10] => false,
        [let Some(x) = Some(Some(None::<i32>)), let Some(y) = x, let None = y] => true,
        [let Some(x) = Some(Some(None::<i32>)), let Some(y) = x, let Some(_) = y] => false,
    }
}
