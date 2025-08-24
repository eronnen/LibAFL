//! Derive macros for LibAFL mutators
//!
//! This crate provides derive macros for implementing mutators in LibAFL.
//! The main macro is `#[derive(Mutator)]` which automatically implements
//! the `Mutator` trait for structs.

#![no_std]
#![warn(missing_docs)]

extern crate alloc;

use alloc::vec::Vec;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Field, Fields::Named, Type, parse_macro_input};

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
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    if let Struct(s) = data {
        if let Named(fields) = s.fields {
            let mut field_mutations = Vec::new();

            // Generate mutation code for each field
            for field in fields.named.iter() {
                let field_ident = &field.ident;
                let field_mutation = generate_field_mutation(field);
                field_mutations.push(quote! {
                    // Randomly choose whether to mutate this field
                    if state.rand_mut().below(100) < 20 { // 20% chance to mutate each field
                        #field_mutation
                    }
                });
            }

            return quote! {
                impl<S> Mutator<#ident, S> for #ident
                where
                    S: HasRand,
                {
                    fn mutate(
                        &mut self,
                        state: &mut S,
                        input: &mut #ident,
                    ) -> Result<MutationResult, Error> {
                        let mut mutated = false;

                        #(#field_mutations)*

                        if mutated {
                            Ok(MutationResult::Mutated)
                        } else {
                            Ok(MutationResult::Skipped)
                        }
                    }
                }
            }
            .into();
        }
    }
    panic!("Only structs with named fields are supported")
}

fn generate_field_mutation(field: &Field) -> TokenStream2 {
    let field_ident = &field.ident;

    if let Type::Path(type_path) = &field.ty {
        if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
            let segment = &type_path.path.segments[0];
            match segment.ident.to_string().as_str() {
                "String" => quote! {
                    if let Some(mutator) = StringMutator::new() {
                        if mutator.mutate(state, &mut input.#field_ident)?.is_mutated() {
                            mutated = true;
                        }
                    }
                },
                "Vec" => quote! {
                    if let Some(mutator) = VecMutator::new() {
                        if mutator.mutate(state, &mut input.#field_ident)?.is_mutated() {
                            mutated = true;
                        }
                    }
                },
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => quote! {
                    if let Some(mutator) = NumberMutator::new() {
                        if mutator.mutate(state, &mut input.#field_ident)?.is_mutated() {
                            mutated = true;
                        }
                    }
                },
                _ => quote! {
                    if let Some(mutator) = input.#field_ident.as_mutator() {
                        if mutator.mutate(state, &mut input.#field_ident)?.is_mutated() {
                            mutated = true;
                        }
                    }
                },
            }
        } else {
            quote! {
                if let Some(mutator) = input.#field_ident.as_mutator() {
                    if mutator.mutate(state, &mut input.#field_ident)?.is_mutated() {
                        mutated = true;
                    }
                }
            }
        }
    } else {
        quote! {}
    }
}
