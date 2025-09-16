use alloc::{format, string::ToString, vec::Vec};

use proc_macro2::TokenStream;
use quote::quote;

use crate::internals::ast::{Container, Field};

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
        let libafl_mutator_impl = self.generate_libafl_mutator_impl();

        let (mutator_struct_definition, mutator_struct_mutate_impl) = match &self.cont.data {
            crate::internals::ast::Data::Enum(_variants) => todo!(),
            crate::internals::ast::Data::Struct(_style, fields) => {
                let generator = StructuredMutatorGeneratorForStruct::new(self, fields);
                (
                    generator.generate_structured_mutator_definition(),
                    generator.generate_structured_mutator_impl(),
                )
            }
        };

        let impl_block = quote! {
            #mutator_struct_definition

            #libafl_mutator_impl

            #mutator_struct_mutate_impl
        };

        Ok(impl_block.into())
    }

    /// Generates the implementation of the libafl::Mutator trait for the StructuredMutator
    fn generate_libafl_mutator_impl(&self) -> TokenStream {
        let ident = &self.cont.ident;
        let struct_ident = &self.mutator_ident;
        quote! {
            impl<S> ::libafl::mutators::Mutator<#ident, S> for #struct_ident<S>
            where
                S: 'static + ::libafl::state::HasRand + ::core::fmt::Debug,
            {
                fn mutate(
                    &mut self,
                    state: &mut S,
                    input: &mut #ident,
                ) -> Result<::libafl::mutators::MutationResult, ::libafl::Error> {
                    let mutated = ::libafl_structured_mutators::StructuredMutator::mutate(self, input, state);
                    if mutated {
                        Ok(::libafl::mutators::MutationResult::Mutated)
                    } else {
                        Ok(::libafl::mutators::MutationResult::Skipped)
                    }
                }

                #[inline]
                fn post_exec(
                    &mut self,
                    _state: &mut S,
                    _corpus_idx: Option<::libafl::corpus::CorpusId>,
                ) -> Result<(), ::libafl::Error> {
                    Ok(())
                }
            }

            impl<S> ::libafl_bolts::Named for #struct_ident<S>
            {
                fn name(&self) -> &::std::borrow::Cow<'static, str> {
                    &::std::borrow::Cow::Borrowed(stringify!(#struct_ident))
                }
            }
        }
    }
}

pub struct StructuredMutatorGeneratorForStruct<'a> {
    parent: &'a StructuredMutatorGenerator<'a>,

    /// The identifiers of the mutator fields and their respective member in the original struct.
    mutator_fields: Vec<(syn::Ident, &'a Field<'a>)>,
}

impl<'a> StructuredMutatorGeneratorForStruct<'a> {
    pub fn new(parent: &'a StructuredMutatorGenerator<'a>, fields: &'a Vec<Field<'a>>) -> Self {
        let mutator_fields = fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                // TODO: use name based on field name so the user can access it if needed
                (
                    syn::Ident::new(&format!("field_mutator_{}", i), parent.cont.ident.span()),
                    f,
                )
            })
            .collect();

        Self {
            parent,
            mutator_fields,
        }
    }

    /// Generates the base definition of the StructuredMutator struct.
    fn generate_structured_mutator_definition(&self) -> TokenStream {
        let mut mutator_fields_declarations = Vec::new();
        let mut mutator_fields_definitions = Vec::new();
        for (field_mutator, field) in self.mutator_fields.iter() {
            let field_type = field.ty;
            mutator_fields_declarations.push(quote! {
                pub #field_mutator: Box<dyn ::libafl_structured_mutators::StructuredMutator<#field_type, S>>,
            });

            mutator_fields_definitions.push(quote! {
                #field_mutator: <#field_type as ::libafl_structured_mutators::HasDefaultStructuredMutator<S>>::default_structured_mutator(),
            });
        }

        let struct_ident = &self.parent.mutator_ident;
        quote! {
            #[derive(Debug)]
            pub struct #struct_ident<S>
            {
                #(#mutator_fields_declarations)*
            }

            impl<S> #struct_ident<S>
            where
                S: 'static + ::libafl::state::HasRand + ::core::fmt::Debug,
            {
                pub fn new() -> Self {
                    Self {
                        #(#mutator_fields_definitions)*
                    }
                }
            }
        }
    }

    /// Generates the implementation of the StructuredMutator trait for the StructuredMutator
    fn generate_structured_mutator_impl(&self) -> TokenStream {
        let mutate_function_body = self.generate_libafl_mutate_function_body();
        let ident = &self.parent.cont.ident;
        let struct_ident = &self.parent.mutator_ident;
        quote! {
            impl<S> ::libafl_structured_mutators::StructuredMutator<#ident, S> for #struct_ident<S>
            where
                #ident: ::libafl_structured_mutators::StructuredInput,
                S: 'static + ::libafl::state::HasRand + ::core::fmt::Debug,
            {
                fn mutate(&mut self, data: &mut #ident, state: &mut S) -> bool {
                    #mutate_function_body

                    false
                }
            }

            impl<S> libafl_structured_mutators::HasDefaultStructuredMutator<S> for #ident
            where
                S: 'static + ::libafl::state::HasRand + ::core::fmt::Debug,
            {
                fn default_structured_mutator() -> Box<dyn ::libafl_structured_mutators::StructuredMutator<Self, S>> {
                    Box::new(#struct_ident::new())
                }
            }
        }
    }

    /// Generates the body of the `mutate` function.
    fn generate_libafl_mutate_function_body(&self) -> TokenStream {
        let number_of_mutatable_fields = self.mutator_fields.len();
        if number_of_mutatable_fields == 0 {
            return quote! {
                return false;
            };
        }

        let choose_field_index_body = quote! {
            let field_index = ::libafl_bolts::rands::Rand::below_or_zero(<S as ::libafl::state::HasRand>::rand_mut(state), (#number_of_mutatable_fields));
        };

        let mut field_mutation_checks = Vec::new();
        for (i, (field_mutator, member)) in self.mutator_fields.iter().enumerate() {
            let member_ident = &member.member;
            let field_mutation = quote! {
                if field_index == #i {
                    println!("Mutating field index: {}", field_index);
                    ::libafl_structured_mutators::StructuredMutator::mutate(&mut *self.#field_mutator, &mut data.#member_ident, state);
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
