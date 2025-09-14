// Re-export derive(SerdeAny)
#[cfg(feature = "derive")]
#[expect(unused_imports)]
#[macro_use]
extern crate libafl_structured_mutators_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use libafl_structured_mutators_derive::*;

pub mod inputs;
pub mod mutators;

pub trait StructuredMutator<D, S>: std::fmt::Debug
where
    S: libafl::state::HasRand,
{
    /// Mutate the given data of type `D`.
    fn mutate(&self, data: &mut D, state: &mut S) -> bool;
}

pub trait StructuredInput: Clone + 'static {
    /// Returns the complexity of the input. used in order to check if it's worth to mutate it.
    fn complexity(&self) -> u64;

    /// Count the number of types D2 in this input
    fn count_data<D2>(&self) -> u32
    where
        D2: StructuredInput;

    /// Returns the D2 instance at the given index (0-based) if it exists, None otherwise.
    /// The index should be less than count_data::<D2>().
    fn sample_data<D2>(&self, idx: u32) -> Option<D2>
    where
        D2: StructuredInput;
}
