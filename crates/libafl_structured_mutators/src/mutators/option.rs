use libafl::state::HasRand;
use libafl_bolts::rands::Rand;

use crate::{HasDefaultStructuredMutator, StructuredInput, StructuredMutator};

#[derive(Debug)]
pub struct OptionStructuredMutator<T, S> {
    pub inner: Box<dyn StructuredMutator<T, S>>,
}

impl<T, S> StructuredMutator<Option<T>, S> for OptionStructuredMutator<T, S>
where
    T: StructuredInput + Default,
    S: HasRand + core::fmt::Debug,
{
    fn weight(&self, data: &Option<T>) -> usize {
        match data {
            Some(inner_data) => 1 + self.inner.weight(inner_data),
            None => 1,
        }
    }

    fn mutate(&mut self, data: &mut Option<T>, state: &mut S) -> bool {
        // println!("Mutating Option: {:?}", data);
        match data {
            Some(inner_data) => {
                // println!("Option is Some, inner data: {:?}", inner_data);
                let inner_weight = self.inner.weight(inner_data);
                // println!("Inner weight: {}", inner_weight);

                if state.rand_mut().below_or_zero(inner_weight + 1) == 0 {
                    // println!("Mutating to None");
                    *data = None;
                    true
                } else {
                    // println!("Mutating inner data: {:?}", inner_data);
                    self.inner.mutate(inner_data, state)
                }
            }
            None => {
                *data = Some(T::default());
                true
            }
        }
    }
}

impl<T, S> HasDefaultStructuredMutator<S> for Option<T>
where
    T: StructuredInput + Default + HasDefaultStructuredMutator<S>,
    S: HasRand + core::fmt::Debug + 'static,
{
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
        Box::new(OptionStructuredMutator {
            inner: T::default_structured_mutator(),
        })
    }
}
