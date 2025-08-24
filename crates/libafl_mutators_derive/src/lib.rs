//! Derive macros for LibAFL mutators
//!
//! This crate provides derive macros for implementing mutators in LibAFL.
//! The main macro is `#[derive(Mutator)]` which automatically implements
//! the `Mutator` trait for structs.

#![no_std]

extern crate alloc;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod mutator;

/// Derive macro to implement the `Mutator` trait for a struct.
///
/// This macro will generate an implementation of `Mutator` that:
/// - Has a configurable chance to mutate each field
/// - Uses specialized mutators for common types
/// - Supports nested struct mutations
///
/// # Example
///
/// ```rust
/// use libafl_mutators_derive::Mutator;
///
/// #[derive(Mutator)]
/// struct MyStruct {
///     name: String,
///     count: u32,
///     data: Vec<u8>,
/// }
/// ```
#[proc_macro_derive(Mutator)]
pub fn mutator_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    mutator::expand_derive_structured_mutator(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
