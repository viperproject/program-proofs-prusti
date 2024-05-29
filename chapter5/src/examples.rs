#[cfg(feature = "pass")]
mod pass {
    mod example_5_2_1;
    mod example_5_2_2;
    mod example_5_3_1;
    mod example_5_4_0_1;
    mod example_5_4_0_2;
    mod example_5_5;
    mod example_5_5_1;
    mod example_5_6;
}

#[cfg(feature = "fail")]
pub use fail::example_5_8_0::*;

#[cfg(feature = "fail")]
mod fail {
    mod example_5_0; // FUTURE: lemma_induction_proof
    mod example_5_1; // FUTURE: lemma_induction_proof
    mod example_5_7_0; // FUTURE: allocation_in_pure_fn + non_copy_types_in_pure_fn + std_lib_extern_spec_requirement
    mod example_5_7_1; // FUTURE: allocation_in_pure_fn + non_copy_types_in_pure_fn + std_lib_extern_spec_requirement
    pub mod example_5_8_0; // FUTURE: allocation_in_pure_fn + non_copy_types_in_pure_fn + map_support + reference_typed_structures
    mod example_5_8_1_1; // FUTURE: non_copy_types_in_pure_fn + allocation_in_pure_fn
    mod example_5_8_1_2; // FUTURE: non_copy_types_in_pure_fn + allocation_in_pure_fn
}
