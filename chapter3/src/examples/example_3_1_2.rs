use prusti_contracts::*;

// 3.1_2.dfy
// function SeqSum(s: seq<int>, lo: int, hi: int): int

// FUTURE: seq_support
// Sequences are replaced with slices in this example

pub fn slice_sum(s: &[i64]) -> i64 {
    // if s.is_empty() { // FUTURE: std_lib_extern_spec_enhancement
    #[allow(clippy::len_zero)]
    if s.len() == 0 {
        0
    } else {
        s[0] + slice_sum(&s[1..s.len()])
    }
}

#[pure]
#[requires(lo <= hi && hi <= s.len())]
// #[decreases(hi - lo)] // FUTURE: termination_check
pub fn slice_sum_index(s: &[i64], lo: usize, hi: usize) -> i64 {
    if lo == hi {
        0
    } else {
        s[lo] + slice_sum_index(s, lo + 1, hi)
    }
}

// FUTURE: std_lib_extern_spec_enhancement
// #[pure]
// pub fn slice_sum_iter(s: &[i64]) -> i64 {
//     s.iter().sum()
// }

#[pure]
pub fn slice_sum_recursive(s: &[i64]) -> i64 {
    // if s.is_empty() { // FUTURE: std_lib_extern_spec_enhancement
    #[allow(clippy::len_zero)]
    if s.len() == 0 {
        0
    } else {
        s[0] + slice_sum_recursive(&s[1..s.len()])
    }
}
