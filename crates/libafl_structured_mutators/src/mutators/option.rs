use libafl::state::HasRand;
use libafl_bolts::rands::Rand;

use crate::{
    HasDefaultStructuredMutator, StructuredInput, StructuredMutator, debug::debug, registry,
};

#[derive(Debug, Default)]
pub struct OptionStructuredMutator {
    pub inner_mutator_key: u64,
}

impl<T, S> StructuredMutator<Option<T>, S> for OptionStructuredMutator
where
    T: StructuredInput + Default,
    S: HasRand + core::fmt::Debug + 'static,
{
    fn weight(&self, data: &Option<T>) -> usize {
        let inner_mutator: &dyn StructuredMutator<T, S> =
            registry::global_get_mutator(self.inner_mutator_key).unwrap();
        match data {
            Some(inner_data) => 1 + inner_mutator.weight(inner_data),
            None => 1,
        }
    }

    fn mutate(&mut self, data: &mut Option<T>, state: &mut S) -> bool {
        debug!(let _span = tracing::trace_span!("OptionStructuredMutator").entered(););
        let inner_mutator: &mut dyn StructuredMutator<T, S> =
            registry::global_get_mutator_mut(self.inner_mutator_key).unwrap();
        match data {
            Some(inner_data) => {
                let inner_weight = inner_mutator.weight(inner_data);
                debug!(tracing::trace!("inner_weight={inner_weight:?}"););
                if state.rand_mut().below_or_zero(inner_weight + 1) == 0 {
                    debug!(tracing::trace!("mutating to None"););
                    *data = None;
                    true
                } else {
                    debug!(tracing::trace!("mutating inner"););
                    inner_mutator.mutate(inner_data, state)
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
        registry::global_initialize::<T, S>();
        Box::new(OptionStructuredMutator::default())
    }
}
