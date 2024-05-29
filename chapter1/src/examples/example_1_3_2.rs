// 1.3_2.dfy
// method Triple(x: int) returns (r: int)

pub fn triple(x: i64) -> i64 {
    // Note: this translation is not a direct mapping because Dafny uses
    // a non-deterministic choice here.
    let r = if x < 18 {
        let a = 2 * x;
        let b = 4 * x;
        (a + b) / 2
    } else if 0 <= x {
        let y = 2 * x;
        x + y
    } else {
        unreachable!();
    };
    assert!(r == 3 * x);
    r
}
