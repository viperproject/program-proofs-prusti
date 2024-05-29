#[cfg(feature = "pass")]
mod pass {
    mod exercise_4_5;
}

#[cfg(feature = "fail")]
mod fail {
    mod exercise_4_1_to_4_4; // FUTURE: non_copy_types_in_pure_fn + allocation_in_pure_fn
    mod exercise_4_6; // FUTURE: non_copy_types_in_pure_fn + allocation_in_pure_fn
    mod exercise_4_7; // FUTURE: reference_typed_structures + allocation_in_pure_fn
    mod exercise_4_8; // FUTURE: reference_typed_structures + allocation_in_pure_fn + mutually_rec_pure_fn
    mod exercise_4_9; // FUTURE: reference_typed_structures + allocation_in_pure_fn + mutually_rec_pure_fn
}