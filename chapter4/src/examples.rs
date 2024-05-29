#[cfg(feature = "pass")]
mod pass {
    mod example_4_1_1_to_4_2_2;
    mod example_4_2_3_and_4_2_4;
    mod example_4_4;
    mod example_4_5_1_and_4_5_2;
}

#[cfg(feature = "fail")]
mod fail {
    mod example_4_6; // FUTURE: reference_typed_structures
}