use libafl::mutators::mutations::{INTERESTING_8, INTERESTING_16, INTERESTING_32};
use libafl_bolts::rands::Rand;

use crate::StructuredMutator;

macro_rules! impl_int_interesting_mutator {
    ($muator_name:ident, $name:ty, $interesting:ident) => {
        #[derive(Default, Debug)]
        pub struct $muator_name;

        impl StructuredMutator<$name> for $muator_name {
            fn mutate<S>(data: &mut $name, state: &mut S)
            where
                S: libafl::state::HasRand,
            {
                let val = *state.rand_mut().choose(&$interesting).unwrap() as $name;
                *data = val;
            }
        }
    };
}

impl_int_interesting_mutator!(U8InterestingMutator, u8, INTERESTING_8);
impl_int_interesting_mutator!(U16InterestingMutator, u16, INTERESTING_16);
impl_int_interesting_mutator!(U32InterestingMutator, u32, INTERESTING_32);
