// 1.4_1.dfy
// method Triple(x: int) returns (r: int)
// method Caller()

// Note: This file correctly fails verification

pub fn triple(x: i64) -> i64 {
    // Note: this translation is not a direct mapping because Dafny uses
    // a non-deterministic choice here.
    let r = match x {
        x if x < 18 => {
            let a = 2 * x;
            let b = 4 * x;
            (a + b) / 2
        }
        x if 0 <= x => {
            let y = 2 * x;
            x + y
        }
        _ => {
            unreachable!();
        }
    };
    assert!(r == 3 * x);
    r
}

pub fn caller() {
    let t = triple(18);
    assert!(t < 100); // cannot be proven due to missing postcondition of triple
}
