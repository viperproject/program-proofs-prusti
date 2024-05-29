use prusti_contracts::*;

// 3.3.2.dfy
// method RequiredStudyTime(c: nat) returns (hours: nat)
// method StudyPlan(n: nat)
// method Learn(n: nat, h: nat)

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
