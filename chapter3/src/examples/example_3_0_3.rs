use prusti_contracts::*;

// 3.0_3.dfy
// method Squarish(x: int, guess: int) returns (y: int)

// Note (2022-10-19): Neither rustc, clippy or Prusti generate any warnings/errors on this code

// FUTURE: termination_check
// Note: this is not a perfect translation, since Dafny has non-deterministic match statements
// See attempts at non-determinism below
#[ensures(result == x * x)]
pub fn squarish(x: i64, guess: i64) -> i64 {
    if guess == x * x {
        guess
    } else if true {
        squarish(x, guess - 1)
    } else {
        squarish(x, guess + 1)
    }
}

#[ensures(result == x * x)]
pub fn squarish_non_deterministic(x: i64, guess: i64) -> i64 {
    loop {
        let r: u64 = rand::random::<u64>() % 3;
        if r == 0 && guess == x * x {
            return guess;
        } else if r == 1 {
            return squarish(x, guess - 1);
        } else if r == 2 {
            return squarish(x, guess + 1);
        }
    }
}

// This function is non-deterministic from Prusti's point of view:
#[trusted]
pub fn rand_bool() -> bool {
    true
}

#[ensures(result == x * x)]
pub fn squarish_non_deterministic_2(x: i64, guess: i64) -> i64 {
    if rand_bool() && guess == x * x {
        guess
    } else if rand_bool() {
        squarish(x, guess - 1)
    } else {
        squarish(x, guess + 1)
    }
}
