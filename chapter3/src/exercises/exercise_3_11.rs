use prusti_contracts::*;

// 3.11.dfy
// method RequiredStudyTime(c: nat) returns (hours: nat)
// method Study(n: nat, h: nat)

#[trusted]
pub fn required_study_time(c: u64) -> u64 {
    unimplemented!()
}

// #[decreases(n * 200 + h + n)] // FUTURE: termination_check
pub fn study(n: u64, h: u64) {
    if h != 0 {
        study(n, h - 1);
    } else if n == 0 {
    } else {
        let hours = required_study_time(n - 1);

        study(n - 1, hours);
    }
}
