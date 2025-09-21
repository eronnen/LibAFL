use crate::{HasDefaultStructuredMutator, StructuredMutator};

#[derive(Default, Debug)]
pub struct BoolStructuredMutator;

impl<S> StructuredMutator<bool, S> for BoolStructuredMutator {
    fn weight(&self, _data: &bool) -> u64 {
        1
    }

    fn mutate(&mut self, data: &mut bool, _state: &mut S) -> bool {
        *data = !*data;
        true
    }
}

impl<S> HasDefaultStructuredMutator<S> for bool {
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
        Box::new(BoolStructuredMutator)
    }
}
