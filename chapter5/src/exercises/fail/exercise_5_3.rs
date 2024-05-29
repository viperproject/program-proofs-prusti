use prusti_contracts::*;

// 5.3.dfy
// function method More(x: int): int
// lemma {:induction false} Increasing(x: int)

// NOTE: Verification should fail

// FUTURE: termination_check
#[pure]
pub fn more(x: i64) -> i64 {
    if x <= 0 {
        1
    } else {
        more(x - 2) + 3
    }
}

// FUTURE: lemma_syntax
// FUTURE: numberOfErrorsToReport
#[pure]
#[ensures(x < more(x))]
pub fn lemma_increasing(x: i64) {
    if x <= 0 {
        assert!(more(x) == 2); // Should and does fail
    } else {
        assert!(more(x) == more(x - 2) + 3);
        assert!(x - 2 < more(x - 2)); // Should fail, but Prusti only reports 1 error per function
        assert!(x + 1 < more(x - 2) + 3);
        assert!(x + 1 < more(x));
    }
}
