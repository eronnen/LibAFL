use crate::{HasDefaultStructuredMutator, StructuredMutator};

// String mutator is not implemented. Users should use Vec<char> instead.
// The reason is that String in Rust is that String in Rust doesn't guarantee direct access to its characters,
// because each character can be multiple bytes (UTF-8 encoding). This makes mutation operations less efficient.
impl<S> HasDefaultStructuredMutator<S> for String
where
    S: 'static,
{
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
        unimplemented!("String mutator is not implemented. Use Vec<char> instead.");
    }
}
