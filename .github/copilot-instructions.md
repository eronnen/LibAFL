# Instructions for Implementing a Derive Proc-Macro for a Generic Mutator in LibAFL

This document outlines the steps to implement a new feature in the LibAFL project. The feature will use a Rust derive proc-macro to automatically generate a mutator for any type of struct. The generated mutator will mutate all fields of the struct and implement the `Mutator` trait from LibAFL.

## Overview

The goal is to simplify the creation of mutators for complex data structures by leveraging Rust's procedural macros. This feature will:
1. Automatically generate a mutator for any struct annotated with a custom derive macro.
2. Ensure that the generated mutator implements the `Mutator` trait.
3. Mutate all fields of the struct, handling nested structs and collections where applicable.

## Steps to Implement

### 1. Create a New Procedural Macro Crate
- Add a new crate to the LibAFL workspace for the procedural macro (e.g., `libafl_derive`).
- Update the `Cargo.toml` of the workspace to include the new crate.
### 2. Implement the Derive Macro
- In the `libafl_derive` crate, create a procedural macro named `#[derive(Mutator)]`.
- Use the `syn` crate to parse the input struct and extract its fields.
- Use the `quote` crate to generate the implementation of the `Mutator` trait for the struct.
    - For each field, generate code to mutate it based on its type.
    - Handle nested structs by recursively invoking their mutators.
    - Handle collections (e.g., `Vec`, `HashMap`) by iterating over their elements and mutating them.
- Ensure the generated code compiles and adheres to the `Mutator` trait's requirements.
- Write unit tests to validate the macro's behavior with various struct definitions.
