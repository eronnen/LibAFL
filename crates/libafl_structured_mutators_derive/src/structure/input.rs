use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;

use crate::internals::ast::{Container, Field, Style};

pub struct StructuredInputGenerator<'a> {
    cont: &'a Container<'a>,
}

impl<'a> StructuredInputGenerator<'a> {
    pub fn new(original_container: &'a Container<'a>) -> Self {
        Self {
            cont: original_container,
        }
    }

    pub fn generate(&self) -> syn::Result<TokenStream> {
        Ok(self.generate_structured_input_impl())
    }

    /// Generates the base definition of the StructuredMutator struct.
    fn generate_structured_input_impl(&self) -> TokenStream {
        match &self.cont.data {
            crate::internals::ast::Data::Enum(_variants) => todo!(),
            crate::internals::ast::Data::Struct(style, fields) => {
                self.generate_structured_input_impl_for_struct(style, fields)
            }
        }
    }

    fn generate_structured_input_impl_for_struct(
        &self,
        _style: &'a Style,
        fields: &'a Vec<Field<'a>>,
    ) -> TokenStream {
        let struct_ident = &self.cont.ident;
        let field_complexity = fields
            .iter()
            .map(|field| {
                let member = &field.member;
                quote!(::libafl_structured_mutators::StructuredInput::complexity(&self.#member))
            })
            .collect::<Vec<_>>();

        let field_counts = fields
            .iter()
            .map(|field| {
                let member = &field.member;
                let ty = &field.ty;
                quote! {
                    // If this field's type matches D2, count it as 1
                    (if core::any::TypeId::of::<#ty>() == core::any::TypeId::of::<D2>() { 1 } else { 0 })
                    // Also add any D2 instances from this field's contents
                    + ::libafl_structured_mutators::StructuredInput::count_data::<D2>(&self.#member)
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl ::libafl_structured_mutators::StructuredInput for #struct_ident {
                fn complexity(&self) -> u64 {
                    1 + #(#field_complexity)+*
                }

                fn count_data<D2>(&self) -> u32
                where
                    D2: ::libafl_structured_mutators::StructuredInput,
                {
                    #(#field_counts)+*
                }

                fn sample_data<S, D2>(&self, _state: &mut S) -> Option<D2>
                where
                    S: libafl::state::HasRand,
                    D2: ::libafl_structured_mutators::StructuredInput,
                {
                    None
                }
            }
        }
    }
}
