use prusti_contracts::*;

// 2.0.dfy
// method MyMethod(x: int) returns (y: int)

#[requires(10 <= x)]
#[ensures(25 <= result)]
pub fn my_method(x: i64) -> i64 {
    let a = x + 3;
    let b = 12;
    a + b
}

#[allow(clippy::let_and_return)]
#[requires(10 <= x)]
#[ensures(25 <= result)]
pub fn my_method_with_backwards_annotations(x: i64) -> i64 {
    prusti_assert!(10 <= x);
    prusti_assert!(25 <= x + 3 + 12);
    let a = x + 3;
    prusti_assert!(25 <= a + 12);
    let b = 12;
    prusti_assert!(25 <= a + b);
    let result = a + b;
    prusti_assert!(25 <= result); // postcondition
    result
}

#[allow(clippy::let_and_return)]
#[requires(10 <= x)]
#[ensures(25 <= result)]
pub fn my_method_with_forwards_annotations(x: i64) -> i64 {
    prusti_assert!(10 <= x); // precondition
    let a = x + 3;
    prusti_assert!(10 <= x && a == x + 3);
    let b = 12;
    prusti_assert!(10 <= x && a == x + 3 && b == 12);
    let result = a + b;
    prusti_assert!(10 <= x && a == x + 3 && b == 12 && result == a + b);
    prusti_assert!(25 <= result);
    result
}
