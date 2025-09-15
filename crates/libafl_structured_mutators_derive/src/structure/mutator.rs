use alloc::{format, string::ToString, vec::Vec};

use proc_macro2::TokenStream;
use quote::quote;

use crate::internals::ast::Container;

pub struct StructuredMutatorGenerator<'a> {
    cont: &'a Container<'a>,
    mutator_ident: syn::Ident,
}

impl<'a> StructuredMutatorGenerator<'a> {
    pub fn new(original_container: &'a Container<'a>) -> Self {
        let mutator_name = format!("{}StructuredMutator", original_container.ident.to_string());
        let mutator_ident = syn::Ident::new(&mutator_name, original_container.ident.span());
        Self {
            cont: original_container,
            mutator_ident,
        }
    }

    pub fn generate(&self) -> syn::Result<TokenStream> {
        let mutator_struct_definition = self.generate_structured_mutator_definition();
        let libafl_mutator_impl = self.generate_libafl_mutator_impl();
        let mutator_struct_mutate_impl = self.generate_structured_mutator_impl();

        let impl_block = quote! {
            #mutator_struct_definition

            #libafl_mutator_impl

            #mutator_struct_mutate_impl
        };

        Ok(impl_block.into())
    }

    /// Generates the base definition of the StructuredMutator struct.
    fn generate_structured_mutator_definition(&self) -> TokenStream {
        let struct_ident = &self.mutator_ident;
        quote! {
            #[derive(Debug)]
            pub struct #struct_ident;

            impl #struct_ident {
                /// Creates a new structured mutator for this type
                pub fn new() -> Self {
                    Self
                }
            }

            impl Default for #struct_ident {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    }

    /// Generates the implementation of the libafl::Mutator trait for the StructuredMutator
    fn generate_libafl_mutator_impl(&self) -> TokenStream {
        let ident = &self.cont.ident;
        let struct_ident = &self.mutator_ident;
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
                    let mutated = ::libafl_structured_mutators::StructuredMutator::mutate(self, input, state);
                    if mutated {
                        Ok(libafl::mutators::MutationResult::Mutated)
                    } else {
                        Ok(libafl::mutators::MutationResult::Skipped)
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

            impl libafl_bolts::Named for #struct_ident {
                fn name(&self) -> &std::borrow::Cow<'static, str> {
                    &std::borrow::Cow::Borrowed(stringify!(#struct_ident))
                }
            }
        }
    }

    /// Generates the implementation of the StructuredMutator trait for the StructuredMutator
    fn generate_structured_mutator_impl(&self) -> TokenStream {
        let mutate_function_body = self.generate_libafl_mutate_function_body();
        let ident = &self.cont.ident;
        let struct_ident = &self.mutator_ident;
        quote! {
            impl<S> ::libafl_structured_mutators::StructuredMutator<#ident, S> for #struct_ident
            where
                S: libafl::state::HasRand,
            {
                fn mutate(&mut self, _data: &mut #ident, state: &mut S) -> bool {
                    use core::num::NonZeroUsize;
                    use libafl::inputs::Input;
                    use libafl::state::HasRand;
                    use libafl_bolts::rands::Rand;

                    #mutate_function_body

                    false
                }
            }
        }
    }

    /// Generates the body of the `mutate` function.
    fn generate_libafl_mutate_function_body(&self) -> TokenStream {
        match &self.cont.data {
            crate::internals::ast::Data::Enum(_variants) => todo!(),
            crate::internals::ast::Data::Struct(_style, fields) => {
                let number_of_mutatable_fields = fields.len();
                if number_of_mutatable_fields == 0 {
                    return quote! {
                        return false;
                    };
                }

                let choose_field_index_body = quote! {
                    let field_index = state.rand_mut().below_or_zero(#number_of_mutatable_fields);
                };

                let mut field_mutation_checks = Vec::new();
                for (i, _field) in fields.iter().enumerate() {
                    let field_mutation = quote! {
                        if field_index == #i {
                            println!("Mutating field index: {}", field_index);
                            return true;
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
}
