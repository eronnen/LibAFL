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
        let mut fields_complexity = Vec::new();
        let mut fields_count = Vec::new();
        let mut fields_sample_count_check = Vec::new();
        for field in fields.iter() {
            let member = &field.member;
            fields_complexity.push(
                quote!(::libafl_structured_mutators::StructuredInput::complexity(&self.#member)),
            );
            fields_count.push(quote!(::libafl_structured_mutators::StructuredInput::count_data::<D2>(&self.#member)));
            fields_sample_count_check.push(quote! {
                current_sample_size = ::libafl_structured_mutators::StructuredInput::count_data::<D2>(&self.#member);
                if current_idx <= idx && idx < current_idx + current_sample_size {
                    return ::libafl_structured_mutators::StructuredInput::sample_data::<D2>(&self.#member, idx - current_idx);
                }
                current_idx += current_sample_size;
            });
        }

        let struct_ident = &self.cont.ident;
        quote! {
            impl ::libafl_structured_mutators::StructuredInput for #struct_ident {
                fn complexity(&self) -> u64 {
                    1 + #(#fields_complexity)+*
                }

                fn count_data<D2>(&self) -> u32
                where
                    D2: ::libafl_structured_mutators::StructuredInput,
                {
                    let result = if ::core::any::TypeId::of::<Self>() == ::core::any::TypeId::of::<D2>() { 1 } else { 0 };
                    result + (#(#fields_count)+*)
                }

                fn sample_data<D2>(&self, mut idx: u32) -> Option<D2>
                where
                    D2: ::libafl_structured_mutators::StructuredInput,
                {
                    if ::core::any::TypeId::of::<Self>() == ::core::any::TypeId::of::<D2>() {
                        if idx == 0 {
                            return Some(unsafe {
                                let ptr = self as *const _ as *const D2;
                                (*ptr).clone()
                            });
                        }

                        idx -= 1;
                    }

                    let mut current_idx = 0;
                    let mut current_sample_size = 0;

                    #(#fields_sample_count_check)*
                    None
                }
            }
        }
    }
}
