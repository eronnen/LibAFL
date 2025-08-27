use alloc::{format, string::ToString, vec::Vec};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type};

use crate::internals::{
    Ctxt,
    ast::{Container, Field, Style},
};

pub fn expand_derive_structured_mutator(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();
    let cont = match Container::from_ast(&ctxt, input) {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };

    ctxt.check()?;

    let mutator_name = format!("{}StructuredMutator", cont.ident.to_string());
    let mutator_ident = syn::Ident::new(&mutator_name, cont.ident.span());
    let ident = &cont.ident;
    println!("Generating mutator for struct: {}", ident);

    let mutation_body = generate_mutate_body(&cont);
    let impl_block = quote! {
        #[derive(Debug, Default)]
        pub struct #mutator_ident;

        impl #mutator_ident {
            /// Creates a new structured mutator for this type
            pub fn new() -> Self {
                Self
            }
        }

        impl<S> libafl::mutators::Mutator<#ident, S> for #mutator_ident
        where
            S: libafl::state::HasRand,
        {
            fn mutate(
                &mut self,
                state: &mut S,
                input: &mut #ident,
            ) -> Result<libafl::mutators::MutationResult, libafl::Error> {
                use core::num::NonZeroUsize;
                use libafl::mutators::MutationResult;
                use libafl::inputs::Input;
                use libafl::state::HasRand;
                use libafl_bolts::rands::Rand;

                let mut mutated = false;

                #mutation_body

                if mutated {
                    Ok(MutationResult::Mutated)
                } else {
                    Ok(MutationResult::Skipped)
                }
            }

            #[inline]
            fn post_exec(
                &mut self,
                _state: &mut S,
                _corpus_idx: Option<libafl::corpus::CorpusId>,
            ) -> Result<(), libafl::Error> {
                Ok(())
            }
        }

        impl libafl_bolts::Named for #mutator_ident {
            fn name(&self) -> &std::borrow::Cow<'static, str> {
                &std::borrow::Cow::Borrowed(stringify!(#mutator_ident))
            }
        }
    };

    Ok(impl_block.into())
}

fn generate_mutate_body(cont: &Container) -> TokenStream {
    match &cont.data {
        crate::internals::ast::Data::Enum(_variants) => todo!(),
        crate::internals::ast::Data::Struct(_style, fields) => {
            let number_of_mutatable_fields = fields.len();
            if number_of_mutatable_fields == 0 {
                return quote! {
                    return Ok(libafl::mutators::MutationResult::Skipped)
                };
            }

            let choose_field_index_body = quote! {
                let field_index = state.rand_mut().below_or_zero(#number_of_mutatable_fields);
            };

            let mut field_mutation_checks = Vec::new();
            for (i, field) in fields.iter().enumerate() {
                let field_mutation = quote! {
                    if field_index == #i {
                        println!("Mutating field index: {}", field_index);
                        return Ok(libafl::mutators::MutationResult::Mutated);
                    }
                };
                field_mutation_checks.push(field_mutation);
            }
            quote! {
                #choose_field_index_body
                #(#field_mutation_checks)*
            }
        }
    }
}

fn generate_mutate_struct_body<'a>(style: &Style, fields: &Vec<Field<'a>>) -> TokenStream {
    let mut field_mutations = Vec::new();
    for field in fields {
        let field_mutation = generate_field_mutation(field);
        field_mutations.push(field_mutation);
    }
    quote! {
        #(#field_mutations)*
    }
}

fn generate_field_mutation<'a>(field: &'a Field<'a>) -> TokenStream {
    let field_ident = &field.original.ident;

    if let Type::Path(type_path) = &field.original.ty {
        if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
            let segment: &syn::PathSegment = &type_path.path.segments[0];
            match segment.ident.to_string().as_str() {
                "Vec" => {
                    // Check if it's Vec<u8> specifically
                    if let Some(Type::Path(inner_type)) = get_vec_element_type(field) {
                        if inner_type.path.is_ident("u8") {
                            quote! {
                                // For Vec<u8>, do random mutations
                                if let Some(max) = NonZeroUsize::new(100) {
                                    if state.rand_mut().below(max) < 10 {
                                        let len = input.#field_ident.len();
                                        if len > 0 {
                                            if let Some(max_len) = NonZeroUsize::new(len) {
                                                let idx = state.rand_mut().below(max_len);
                                                input.#field_ident[idx] = state.rand_mut().next() as u8;
                                                mutated = true;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            quote! {}
                        }
                    } else {
                        quote! {}
                    }
                }
                "Option" => {
                    quote! {
                        // For Option types, randomly toggle None/Some
                        if let Some(max) = NonZeroUsize::new(100) {
                            if state.rand_mut().below(max) < 10 {
                                input.#field_ident = match &input.#field_ident {
                                    Some(_) => None,
                                    None => Some(Default::default()),
                                };
                                mutated = true;
                            }
                        }
                    }
                }
                "bool" => quote! {
                    // For booleans, randomly flip
                    if let Some(max) = NonZeroUsize::new(100) {
                        if state.rand_mut().below(max) < 10 {
                            input.#field_ident = !input.#field_ident;
                            mutated = true;
                        }
                    }
                },
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => quote! {
                    // For numeric types, do random mutations
                    if let Some(max) = NonZeroUsize::new(100) {
                        if state.rand_mut().below(max) < 10 {
                            let val = state.rand_mut().next();
                            input.#field_ident = val as _;
                            mutated = true;
                        }
                    }
                },
                _ => quote! {},
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

fn get_vec_element_type<'a>(field: &'a Field<'a>) -> Option<&'a Type> {
    if let Type::Path(type_path) = &field.original.ty {
        if type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == "Vec" {
            if let syn::PathArguments::AngleBracketed(args) = &type_path.path.segments[0].arguments
            {
                if args.args.len() == 1 {
                    if let syn::GenericArgument::Type(ty) = &args.args[0] {
                        return Some(ty);
                    }
                }
            }
        }
    }
    None
}
