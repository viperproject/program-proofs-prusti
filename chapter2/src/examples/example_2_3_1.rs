use prusti_contracts::*;

// 2.3.1.dfy
// method Swap(x: int, y: int) returns (a: int, b: int)

#[ensures(result == (y, x))]
pub fn swap_a(x: i64, y: i64) -> (i64, i64) {
    let mut x = x;
    let mut y = y;
    x = y - x;
    y -= x;
    x += y;
    (x, y)
}

// // FUTURE: mutable_fn_args
// #[ensures(result == old((y, x)))]
// pub fn swap(mut x: i64, mut y: i64) -> (i64, i64) {
//     x = y - x;
//     y = y - x;
//     x = y + x;
//     (x, y)
// }

// // FUTURE: mutable_fn_args
// #[ensures(result == old((y, x)))]
// pub fn swap_annotated(mut x: i64, mut y: i64) -> (i64, i64) {
//     let x0 = x;
//     let y0 = y;
//     prusti_assert!(x == x0 && y == y0);
//     x = y - x;
//     prusti_assert!(x == y0 - x0 && y == y0);
//     y = y - x;
//     prusti_assert!(x == y0 - x0 && y == y0 - (y0 - x0));
//     prusti_assert!(x == y0 - x0 && y == x0);
//     x = y + x;
//     prusti_assert!(x == x0 + y0 - x0 && y == x0);
//     prusti_assert!(x == y0 && y == x0);
//     (x, y)
//     // (y, x)
// }
