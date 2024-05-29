// 1.1.dfy
// method Triple(x: int) returns (r: int)

pub fn triple(x: i64) -> i64 {
    let y = 2 * x;
    let r = x + y;
    assert!(r == 3 * x);
    r
}
