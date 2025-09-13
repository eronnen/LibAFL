use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{
    internals::{Ctxt, ast::Container},
    mutator::mutator::StructuredMutatorGenerator,
};

mod mutator;

pub fn expand_derive_structured_mutator(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();
    let cont = match Container::from_ast(&ctxt, input) {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };

    ctxt.check()?;
    println!("Generating mutator for struct: {}", &cont.ident);

    let structured_mutator_generator = StructuredMutatorGenerator::new(cont);
    structured_mutator_generator.generate()
}
