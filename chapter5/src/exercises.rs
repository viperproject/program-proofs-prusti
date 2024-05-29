#[cfg(feature = "pass")]
mod pass {
    mod exercise_5_5;
    mod exercise_5_7;
}

#[cfg(feature = "fail")]
mod fail {
    mod exercise_5_0; // FUTURE: lemma_induction_proof
    mod exercise_5_3; // Verification should and does fail here
    mod exercise_5_8; // FUTURE: std_lib_extern_spec_requirement + allocation_in_pure_fn + non_copy_types_in_pure_fn
    mod exercise_5_10; // FUTURE: map_support
    mod exercise_5_11; // FUTURE: std_lib_extern_spec_requirement, map_support
    mod exercise_5_12; // FUTURE: allocation_in_pure_fn + non_copy_types_in_pure_fn
}

// exercise_5_9: skipped, builds on (failing) example_5_8_0.rs
// exercise_5_13: not a coding task
// exercise_5_14: skipped, builds on (failing) example_5_8_1_2.rs, requires termination checks
// exercise_5_15; skipped, builds on (failing) example_5_8_1_2.rs
