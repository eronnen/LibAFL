use core::mem::size_of;

use libafl::mutators::mutations::{INTERESTING_8, INTERESTING_16, INTERESTING_32};
use libafl_bolts::rands::Rand;

use crate::StructuredMutator;

/// Interesting 64-bit values, including both 32-bit edge cases and 64-bit specific values
pub const INTERESTING_64: [i64; 46] = [
    // Small integers
    -128,
    -1,
    0,
    1,
    16,
    32,
    64,
    100,
    127,
    // 16-bit boundaries
    -32768,
    -129,
    128,
    255,
    256,
    512,
    1000,
    1024,
    4096,
    32767,
    // 32-bit boundaries
    -2147483648,
    -100663046,
    -32769,
    32768,
    65535,
    65536,
    100663045,
    2147483647,
    // 64-bit specific values
    -9223372036854775808, // i64::MIN
    9223372036854775807,  // i64::MAX
    -9223372036854775807, // i64::MIN + 1
    9223372036854775806,  // i64::MAX - 1
    4611686018427387904,  // i64::MAX / 2
    -4611686018427387904, // i64::MIN / 2
    9223372036854775552,  // i64::MAX - 255
    -9223372036854775552, // i64::MIN + 255
    4294967295,           // u32::MAX
    4294967296,           // u32::MAX + 1
    8589934592,           // 1 << 33
    17179869184,          // 1 << 34
    281474976710656,      // 1 << 48
    562949953421312,      // 1 << 49
    1152921504606846976,  // 1 << 60
    2305843009213693952,  // 1 << 61
    4611686018427387904,  // 1 << 62
    6917529027641081856,  // 3/4 of i64::MAX
    -6917529027641081856, // 3/4 of i64::MIN
];

macro_rules! impl_int_inc_mutator {
    ($muator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $muator_name;

        impl<S> StructuredMutator<$name, S> for $muator_name {
            fn mutate(&mut self, data: &mut $name, _state: &mut S) -> bool {
                *data = data.wrapping_add(1);
                true
            }
        }
    };
}

impl_int_inc_mutator!(U8IncMutator, u8);
impl_int_inc_mutator!(I8IncMutator, i8);
impl_int_inc_mutator!(U16IncMutator, u16);
impl_int_inc_mutator!(I16IncMutator, i16);
impl_int_inc_mutator!(U32IncMutator, u32);
impl_int_inc_mutator!(I32IncMutator, i32);
impl_int_inc_mutator!(U64IncMutator, u64);
impl_int_inc_mutator!(I64IncMutator, i64);

macro_rules! impl_int_dec_mutator {
    ($muator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $muator_name;

        impl<S> StructuredMutator<$name, S> for $muator_name {
            fn mutate(&mut self, data: &mut $name, _state: &mut S) -> bool {
                *data = data.wrapping_sub(1);
                true
            }
        }
    };
}

impl_int_dec_mutator!(U8DecMutator, u8);
impl_int_dec_mutator!(I8DecMutator, i8);
impl_int_dec_mutator!(U16DecMutator, u16);
impl_int_dec_mutator!(I16DecMutator, i16);
impl_int_dec_mutator!(U32DecMutator, u32);
impl_int_dec_mutator!(I32DecMutator, i32);
impl_int_dec_mutator!(U64DecMutator, u64);
impl_int_dec_mutator!(I64DecMutator, i64);

macro_rules! impl_int_interesting_mutator {
    ($muator_name:ident, $name:ty, $interesting:ident) => {
        #[derive(Default, Debug)]
        pub struct $muator_name;

        impl<S> StructuredMutator<$name, S> for $muator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let val = *state.rand_mut().choose(&$interesting).unwrap() as $name;
                *data = val;
                true
            }
        }
    };
}

impl_int_interesting_mutator!(U8InterestingMutator, u8, INTERESTING_8);
impl_int_interesting_mutator!(I8InterestingMutator, i8, INTERESTING_8);
impl_int_interesting_mutator!(U16InterestingMutator, u16, INTERESTING_16);
impl_int_interesting_mutator!(I16InterestingMutator, i16, INTERESTING_16);
impl_int_interesting_mutator!(U32InterestingMutator, u32, INTERESTING_32);
impl_int_interesting_mutator!(I32InterestingMutator, i32, INTERESTING_32);
impl_int_interesting_mutator!(U64InterestingMutator, u64, INTERESTING_64);
impl_int_interesting_mutator!(I64InterestingMutator, i64, INTERESTING_64);

macro_rules! impl_int_bit_flip_mutator {
    ($mutator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $mutator_name;

        impl<S> StructuredMutator<$name, S> for $mutator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let bit_pos = state.rand_mut().below_or_zero(size_of::<$name>() * 8);
                *data ^= 1 << bit_pos;
                true
            }
        }
    };
}

macro_rules! impl_int_byte_flip_mutator {
    ($mutator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $mutator_name;

        impl<S> StructuredMutator<$name, S> for $mutator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let byte_pos = state.rand_mut().below_or_zero(size_of::<$name>());
                *data ^= (0xffu8 as $name) << (byte_pos * 8);
                true
            }
        }
    };
}

macro_rules! impl_int_byte_neg_mutator {
    ($mutator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $mutator_name;

        impl<S> StructuredMutator<$name, S> for $mutator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let byte_pos = state.rand_mut().below_or_zero(size_of::<$name>());
                let mask = (0xffu8 as $name) << (byte_pos * 8);
                let selected_byte = (*data & mask) >> (byte_pos * 8);
                let negated = (!selected_byte & 0xffu8 as $name) as $name;
                *data = (*data & !mask) | (negated << (byte_pos * 8));
                true
            }
        }
    };
}

macro_rules! impl_int_byte_rand_mutator {
    ($mutator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $mutator_name;

        impl<S> StructuredMutator<$name, S> for $mutator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let byte_pos = state.rand_mut().below_or_zero(size_of::<$name>());
                let rand_byte = state.rand_mut().below_or_zero(256) as $name;
                let mask = (0xffu8 as $name) << (byte_pos * 8);
                *data = (*data & !mask) | (rand_byte << (byte_pos * 8));
                true
            }
        }
    };
}

macro_rules! impl_int_byte_add_mutator {
    ($mutator_name:ident, $name:ty) => {
        #[derive(Default, Debug)]
        pub struct $mutator_name;

        impl<S> StructuredMutator<$name, S> for $mutator_name
        where
            S: libafl::state::HasRand,
        {
            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let byte_pos = state.rand_mut().below_or_zero(size_of::<$name>());
                let add_val = state.rand_mut().below_or_zero(256) as $name;
                let mask = (0xffu8 as $name) << (byte_pos * 8);
                let selected_byte = (*data & mask) >> (byte_pos * 8);
                let added = (selected_byte.wrapping_add(add_val) & 0xffu8 as $name) as $name;
                *data = (*data & !mask) | (added << (byte_pos * 8));
                true
            }
        }
    };
}

// Implement new mutators for each integer type
impl_int_bit_flip_mutator!(U8BitFlipMutator, u8);
impl_int_bit_flip_mutator!(I8BitFlipMutator, i8);
impl_int_bit_flip_mutator!(U16BitFlipMutator, u16);
impl_int_bit_flip_mutator!(I16BitFlipMutator, i16);
impl_int_bit_flip_mutator!(U32BitFlipMutator, u32);
impl_int_bit_flip_mutator!(I32BitFlipMutator, i32);
impl_int_bit_flip_mutator!(U64BitFlipMutator, u64);
impl_int_bit_flip_mutator!(I64BitFlipMutator, i64);

impl_int_byte_flip_mutator!(U8ByteFlipMutator, u8);
impl_int_byte_flip_mutator!(I8ByteFlipMutator, i8);
impl_int_byte_flip_mutator!(U16ByteFlipMutator, u16);
impl_int_byte_flip_mutator!(I16ByteFlipMutator, i16);
impl_int_byte_flip_mutator!(U32ByteFlipMutator, u32);
impl_int_byte_flip_mutator!(I32ByteFlipMutator, i32);
impl_int_byte_flip_mutator!(U64ByteFlipMutator, u64);
impl_int_byte_flip_mutator!(I64ByteFlipMutator, i64);

impl_int_byte_neg_mutator!(U8ByteNegMutator, u8);
impl_int_byte_neg_mutator!(I8ByteNegMutator, i8);
impl_int_byte_neg_mutator!(U16ByteNegMutator, u16);
impl_int_byte_neg_mutator!(I16ByteNegMutator, i16);
impl_int_byte_neg_mutator!(U32ByteNegMutator, u32);
impl_int_byte_neg_mutator!(I32ByteNegMutator, i32);
impl_int_byte_neg_mutator!(U64ByteNegMutator, u64);
impl_int_byte_neg_mutator!(I64ByteNegMutator, i64);

impl_int_byte_rand_mutator!(U8ByteRandMutator, u8);
impl_int_byte_rand_mutator!(I8ByteRandMutator, i8);
impl_int_byte_rand_mutator!(U16ByteRandMutator, u16);
impl_int_byte_rand_mutator!(I16ByteRandMutator, i16);
impl_int_byte_rand_mutator!(U32ByteRandMutator, u32);
impl_int_byte_rand_mutator!(I32ByteRandMutator, i32);
impl_int_byte_rand_mutator!(U64ByteRandMutator, u64);
impl_int_byte_rand_mutator!(I64ByteRandMutator, i64);

impl_int_byte_add_mutator!(U8ByteAddMutator, u8);
impl_int_byte_add_mutator!(I8ByteAddMutator, i8);
impl_int_byte_add_mutator!(U16ByteAddMutator, u16);
impl_int_byte_add_mutator!(I16ByteAddMutator, i16);
impl_int_byte_add_mutator!(U32ByteAddMutator, u32);
impl_int_byte_add_mutator!(I32ByteAddMutator, i32);
impl_int_byte_add_mutator!(U64ByteAddMutator, u64);
impl_int_byte_add_mutator!(I64ByteAddMutator, i64);

/// Implements a default mutator for integers that chooses a random basic integer mutation each time.
macro_rules! impl_int_default_mutator {
    ($mutator_name:ident, $name:ty, $($inner:ty),* $(,)?) => {
        #[derive(Debug)]
        pub struct $mutator_name<S>
        where
            S: core::fmt::Debug,
        {
            mutations: Vec<Box<dyn StructuredMutator<$name, S>>>,
        }

        impl<S> Default for $mutator_name<S>
        where
            S: libafl::state::HasRand + core::fmt::Debug,
        {
            fn default() -> Self {
                Self {
                    mutations: vec![
                        $(Box::new(<$inner>::default()),)*
                    ],
                }
            }
        }

        impl<S> crate::StructuredMutator<$name, S> for $mutator_name<S>
        where
            S: libafl::state::HasRand + core::fmt::Debug,
        {
            fn weight(&self, _data: & $name) -> usize {
                self.mutations.len()
            }

            fn mutate(&mut self, data: &mut $name, state: &mut S) -> bool {
                let mutation = state.rand_mut().choose(&mut self.mutations).unwrap();
                mutation.mutate(data, state)
            }
        }

        impl<S> crate::HasDefaultStructuredMutator<S> for $name
        where
            S: libafl::state::HasRand + core::fmt::Debug + 'static,
        {
            fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
                Box::new($mutator_name::default())
            }
        }
    };
}

impl_int_default_mutator!(
    U8StructuredMutator,
    u8,
    U8IncMutator,
    U8DecMutator,
    U8InterestingMutator,
    U8BitFlipMutator,
    U8ByteFlipMutator,
    U8ByteNegMutator,
    U8ByteRandMutator,
    U8ByteAddMutator,
);
impl_int_default_mutator!(
    I8StructuredMutator,
    i8,
    I8IncMutator,
    I8DecMutator,
    I8InterestingMutator,
    I8BitFlipMutator,
    I8ByteFlipMutator,
    I8ByteNegMutator,
    I8ByteRandMutator,
    I8ByteAddMutator,
);
impl_int_default_mutator!(
    U16StructuredMutator,
    u16,
    U16IncMutator,
    U16DecMutator,
    U16InterestingMutator,
    U16BitFlipMutator,
    U16ByteFlipMutator,
    U16ByteNegMutator,
    U16ByteRandMutator,
    U16ByteAddMutator,
);
impl_int_default_mutator!(
    I16StructuredMutator,
    i16,
    I16IncMutator,
    I16DecMutator,
    I16InterestingMutator,
    I16BitFlipMutator,
    I16ByteFlipMutator,
    I16ByteNegMutator,
    I16ByteRandMutator,
    I16ByteAddMutator,
);
impl_int_default_mutator!(
    U32StructuredMutator,
    u32,
    U32IncMutator,
    U32DecMutator,
    U32InterestingMutator,
    U32BitFlipMutator,
    U32ByteFlipMutator,
    U32ByteNegMutator,
    U32ByteRandMutator,
    U32ByteAddMutator,
);
impl_int_default_mutator!(
    I32StructuredMutator,
    i32,
    I32IncMutator,
    I32DecMutator,
    I32InterestingMutator,
    I32BitFlipMutator,
    I32ByteFlipMutator,
    I32ByteNegMutator,
    I32ByteRandMutator,
    I32ByteAddMutator,
);
impl_int_default_mutator!(
    U64StructuredMutator,
    u64,
    U64IncMutator,
    U64DecMutator,
    U64InterestingMutator,
    U64BitFlipMutator,
    U64ByteFlipMutator,
    U64ByteNegMutator,
    U64ByteRandMutator,
    U64ByteAddMutator,
);
impl_int_default_mutator!(
    I64StructuredMutator,
    i64,
    I64IncMutator,
    I64DecMutator,
    I64InterestingMutator,
    I64BitFlipMutator,
    I64ByteFlipMutator,
    I64ByteNegMutator,
    I64ByteRandMutator,
    I64ByteAddMutator,
);
