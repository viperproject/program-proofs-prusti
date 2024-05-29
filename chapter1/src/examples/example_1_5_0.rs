use prusti_contracts::*;

// 1.5.0.dfy
// method Triple(x: int) returns (r: int)
// ghost method DoubleQuadruple(x: int) returns (a: int, b: int)

#[ensures(result == x * 3)]
pub fn triple(x: i64) -> i64 {
    let y = 2 * x;
    let r = x + y;

    //FUTURE: ghost_variables
    let (a, b) = ghost_double_quadruple(x);
    prusti_assert!((a <= r && r <= b) || (b <= r && r <= a));
    r
}

// FUTURE: ghost_functions
#[pure]
fn ghost_double_quadruple(x: i64) -> (i64, i64) {
    (2 * x, 4 * x)
}
