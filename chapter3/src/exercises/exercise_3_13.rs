use prusti_contracts::*;

// 3.13.dfy
// method RequredStudyTime(c: nat) returns (hours: nat)
// method StudyPlan(n: nat)
// method Outer(a: nat)
// method Inner(a: nat, b: nat)

#[trusted]
pub fn required_study_time(c: u64) -> u64 {
    unimplemented!()
}

#[requires(n <= 40)]
// #[decreases(40 - n)] // FUTURE: termination_check
pub fn study_plan(n: u64) {
    if n == 40 {
    } else {
        let hours = required_study_time(n);
        learn(n, hours);
    }
}

#[requires(n < 40)]
// #[decreases(40 - n, h)] // FUTURE: termination_check
pub fn learn(n: u64, h: u64) {
    if h == 0 {
        study_plan(n + 1);
    } else {
        learn(n, h - 1);
    }
}

// #[decreases(a, 0)] // FUTURE: termination_check
pub fn outer(a: u64) {
    if a != 0 {
        let b = required_study_time(a - 1);
        inner(a - 1, b);
    }
}

// #[decreases(a, 1, b)] // FUTURE: termination_check
pub fn inner(a: u64, b: u64) {
    if b == 0 {
        outer(a);
    } else {
        inner(a, b - 1);
    }
}
