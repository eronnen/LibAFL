use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{
    internals::{Ctxt, ast::Container},
    structure::{input::StructuredInputGenerator, mutator::StructuredMutatorGenerator},
};

mod input;
mod mutator;

pub fn expand_derive_structured_mutator(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();
    let cont = match Container::from_ast(&ctxt, input) {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };

    ctxt.check()?;

    let structured_input_generator = StructuredInputGenerator::new(&cont);
    let structured_mutator_generator = StructuredMutatorGenerator::new(&cont);

    let input_block = structured_input_generator.generate()?;
    let mutator_block = structured_mutator_generator.generate()?;

    Ok(quote::quote! {
        #input_block

        #mutator_block
    })
}
