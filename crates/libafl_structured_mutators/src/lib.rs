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

pub trait StructuredMutator<D: StructuredInput, S>: core::fmt::Debug {
    /// Return the weight of the given data. Used to decide how likely it is to mutate this data.
    /// Normally, the weight should be proportional to the complexity of the data.
    fn weight(&self, data: &D) -> u64 {
        data.complexity() as u64
    }

    /// Mutate the given data of type `D`.
    fn mutate(&mut self, data: &mut D, state: &mut S) -> bool;
}

pub trait HasDefaultStructuredMutator<S> {
    /// Returns the default structured mutator for this type.
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>>;
}

pub trait StructuredInput: core::fmt::Debug + Clone + 'static {
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
