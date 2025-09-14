use proc_macro2::TokenStream;
use quote::quote;

use crate::internals::ast::Container;

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
        let structured_input_impl = self.generate_structured_input_impl();
        Ok(structured_input_impl.into())
    }

    /// Generates the base definition of the StructuredMutator struct.
    fn generate_structured_input_impl(&self) -> TokenStream {
        let struct_ident = &self.cont.ident;
        quote! {
            impl libafl_structured_mutators::StructuredInput for #struct_ident {
                fn complexity(&self) -> u64 {
                    1
                }

                fn count_data<D2>(&self) -> u32
                where
                    D2: libafl_structured_mutators::StructuredInput,
                {
                    0
                }

                fn sample_data<S, D2>(&self, _state: &mut S) -> Option<D2>
                where
                    S: libafl::state::HasRand,
                    D2: libafl_structured_mutators::StructuredInput,
                {
                    None
                }
            }
        }
    }
}
