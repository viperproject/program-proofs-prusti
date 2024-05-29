// 1.3_1.dfy
// method Triple(x: int) returns (r: int)

pub fn triple(x: i64) -> i64 {
    let r = if x == 0 {
        0
    } else {
        let y = 2 * x;
        x + y
    };
    assert!(r == 3 * x);
    r
}
