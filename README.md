# `program-proofs-prusti`

Examples and exercises from the book [*Program Proofs*](https://program-proofs.com/) by K. Rustan M. Leino translated to Rust and verified with [Prusti](https://prusti.ethz.ch/), a deductive verifier for Rust programs developed at ETH Zurich.

## Repository structure

Each chapter of the book that we have translated can be found in its own crate at the root of this repository.

Each crate can be verified, as a whole, using [`cargo prusti`](https://viperproject.github.io/prusti-dev/user-guide/basic.html#command-line) from the command line or using the [Prusti Assistant](https://viperproject.github.io/prusti-dev/user-guide/basic.html#prusti-assistant) extension for VS Code.

Within each crate, there are two main subdirectories of interest:

- `(chapter)/src/examples` - contains translated *examples*, i.e. various snippets of code from the given chapter;
- `(chapter)/src/exercises` - contains translated *exercises*, i.e. (possible) solutions to the exercises from the given chapter.

Files in these subdirectories follow the naming scheme `example_(section)_(subsection).rs` or `exercise_(section)_(subsection).rs`. The remaining files in each crate serve to configure Prusti, configure Cargo, and to tie together the example and exercise files.

## Current status

|            | Chapter | Notes |
| ----------:| ------- | ----- |
| **PART 0** | **Learning the Ropes** | |
| Chapter 1  | [Basics](chapter1) | |
| Chapter 2  | [Making it Formal](chapter2) | |
| Chapter 3  | [Recursion and Termination](chapter3) | Termination checking is not yet supported in Prusti |
| Chapter 4  | [Inductive Datatypes](chapter4) | |
| Chapter 5  | [Lemmas and Proofs](chapter5) | Ghost code is not yet supported in Prusti |
| **PART 1** | **Functional Programs** | |
| Chapter 6  | ~~Lists~~ | (Skipped) |
| Chapter 7  | ~~Unary Numbers~~ | (Skipped) |
| Chapter 8  | [Sorting](chapter8) | |
| Chapter 9  | [Modules](chapter9) | |
| Chapter 10 | ~~Data-Structure Invariants~~ | (Skipped) |
| **PART 2** | **Imperative Programs** | |
| Chapter 11 | ~~Loops~~ | (Skipped) |
| Chapter 12 | ~~Recursive Specifications, Iterative Programs~~ | (Skipped) |
| Chapter 13 | ~~Arrays and Searching~~ | (Skipped) |
| Chapter 14 | [Modifying Arrays](chapter14) | |
| Chapter 15 | ~~In-situ Sorting~~ | (Skipped) | |
| Chapter 16 | [Objects](chapter16) | |
| Chapter 17 | [Mutable Data Structures](chapter17) | |

## References

The translations found in this repository were developed as part of Patrick Muntwiler's BSc thesis.

- Muntwiler, Patrick. ["Evaluating and Documenting a Rust Verifier."](https://ethz.ch/content/dam/ethz/special-interest/infk/chair-program-method/pm/documents/Education/Theses/Patrick_Muntwiler_BS_Thesis.pdf) (2023).
