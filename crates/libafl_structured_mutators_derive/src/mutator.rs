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
    println!("Generating mutator for struct: {}", &cont.ident);

    let mutator_name = format!("{}StructuredMutator", cont.ident.to_string());
    let mutator_ident = syn::Ident::new(&mutator_name, cont.ident.span());
    let mutator_struct_definition = generate_structured_mutator_definition(&mutator_ident);

    let mutator_struct_mutate_impl = generate_structured_mutator_mutate_impl(&mutator_ident, &cont);
    let impl_block = quote! {
        #mutator_struct_definition

        #mutator_struct_mutate_impl
    };

    Ok(impl_block.into())
}

/// Generates the base definition of the StructuredMutator struct.
fn generate_structured_mutator_definition(struct_ident: &syn::Ident) -> TokenStream {
    quote! {
        #[derive(Debug)]
        pub struct #struct_ident;

        impl #struct_ident {
            /// Creates a new structured mutator for this type
            pub fn new() -> Self {
                Self
            }
        }
    }
}

/// Generates the implementation of the Mutator trait for the StructuredMutator
fn generate_structured_mutator_mutate_impl(
    struct_ident: &syn::Ident,
    cont: &Container,
) -> TokenStream {
    let mutate_function_body = generate_mutate_function_body(&cont);
    let ident = &cont.ident;
    quote! {
        impl<S> libafl::mutators::Mutator<#ident, S> for #struct_ident
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

                #mutate_function_body

                Ok(MutationResult::Skipped)
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

        impl libafl_bolts::Named for #struct_ident {
            fn name(&self) -> &std::borrow::Cow<'static, str> {
                &std::borrow::Cow::Borrowed(stringify!(#struct_ident))
            }
        }
    }
}

fn generate_mutate_function_body(cont: &Container) -> TokenStream {
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

// fn generate_mutate_struct_body<'a>(style: &Style, fields: &Vec<Field<'a>>) -> TokenStream {
//     let mut field_mutations = Vec::new();
//     for field in fields {
//         let field_mutation = generate_field_mutation(field);
//         field_mutations.push(field_mutation);
//     }
//     quote! {
//         #(#field_mutations)*
//     }
// }
