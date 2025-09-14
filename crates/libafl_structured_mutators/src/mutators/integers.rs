use libafl::mutators::mutations::{INTERESTING_8, INTERESTING_16, INTERESTING_32};
use libafl_bolts::rands::Rand;

use crate::StructuredMutator;

macro_rules! impl_int_inc_mutator {
    ($muator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $muator_name;

        impl<S> StructuredMutator<$name, S> for $muator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&self, data: &mut $name, _state: &mut S) {
                *data = data.wrapping_add(1);
            }
        }
    };
}

impl_int_inc_mutator!(U8IncMutator, u8);
impl_int_inc_mutator!(U16IncMutator, u16);
impl_int_inc_mutator!(U32IncMutator, u32);

macro_rules! impl_int_interesting_mutator {
    ($muator_name:ident, $name:ty, $interesting:ident) => {
        #[derive(Default, Debug)]
        pub struct $muator_name;

        impl<S> StructuredMutator<$name, S> for $muator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&self, data: &mut $name, state: &mut S) {
                let val = *state.rand_mut().choose(&$interesting).unwrap() as $name;
                *data = val;
            }
        }
    };
}

impl_int_interesting_mutator!(U8InterestingMutator, u8, INTERESTING_8);
impl_int_interesting_mutator!(I8InterestingMutator, i8, INTERESTING_8);
impl_int_interesting_mutator!(U16InterestingMutator, u16, INTERESTING_16);
impl_int_interesting_mutator!(U32InterestingMutator, u32, INTERESTING_32);

/// Implements a default mutator for integers that chooses a random basic integer mutation each time.
macro_rules! impl_int_default_mutator {
    ($mutator_name:ident, $name:ty, $($inner:ty),* $(,)?) => {
        #[derive(Debug)]
        pub struct $mutator_name<S>
        where
            S: libafl::state::HasRand + std::fmt::Debug,
        {
            mutations: Vec<Box<dyn StructuredMutator<u8, S>>>,
        }

        impl<S> $mutator_name<S>
        where
            S: libafl::state::HasRand + std::fmt::Debug,
        {
            pub fn new() -> Self {
                Self {
                    mutations: vec![
                        $(Box::new(<$inner>::default()),)*
                    ],
                }
            }
        }

        impl<S> StructuredMutator<$name, S> for $mutator_name<S>
        where
            S: libafl::state::HasRand + std::fmt::Debug,
        {
            fn mutate(&self, data: &mut $name, state: &mut S) {
                let mutation = state.rand_mut().choose(&self.mutations).unwrap();
                mutation.mutate(data, state)
            }
        }

        impl<S> Default for $mutator_name<S>
        where
            S: libafl::state::HasRand + std::fmt::Debug,
        {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

impl_int_default_mutator!(U8StructuredMutator, u8, U8InterestingMutator, U8IncMutator);
