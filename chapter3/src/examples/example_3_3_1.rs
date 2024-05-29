use prusti_contracts::*;

// 3.3.1.dfy
// function Ack(m: nat, n: nat): nat

// #[decreases(m, n)] // FUTURE: termination_check
pub fn ack(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}
