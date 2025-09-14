// Re-export derive(SerdeAny)
#[cfg(feature = "derive")]
#[expect(unused_imports)]
#[macro_use]
extern crate libafl_structured_mutators_derive;
use std::fmt::Debug;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use libafl_structured_mutators_derive::*;

pub mod inputs;
pub mod mutators;

pub trait StructuredMutator<D, S>: Debug
where
    S: libafl::state::HasRand,
{
    /// Mutate the given data of type `D`.
    fn mutate(&self, data: &mut D, state: &mut S);
}

pub trait StructuredInput: 'static {
    /// Returns the complexity of the input. used in order to check if it's worth to mutate it.
    fn complexity(&self) -> u64;

    /// Count the number of types D2 in this input
    fn count_data<D2>(&self) -> u32
    where
        D2: StructuredInput;

    /// Sample a random data type from the current input. Used for splicing mutators.
    fn sample_data<S, D2>(&self, state: &mut S) -> Option<D2>
    where
        S: libafl::state::HasRand,
        D2: StructuredInput;
}
