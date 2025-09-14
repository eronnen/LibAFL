use crate::StructuredInput;

macro_rules! impl_structured_input_for_primitive {
    ($primitive:ty) => {
        impl StructuredInput for $primitive {
            fn complexity(&self) -> u64 {
                1
            }

            fn count_data<D2>(&self) -> u32
            where
                D2: StructuredInput,
            {
                0
            }

            fn sample_data<S, D2>(&self, _state: &mut S) -> Option<D2>
            where
                S: libafl::state::HasRand,
                D2: StructuredInput,
            {
                None
            }
        }
    };
}

impl_structured_input_for_primitive!(u8);
impl_structured_input_for_primitive!(i8);
impl_structured_input_for_primitive!(u16);
impl_structured_input_for_primitive!(i16);
impl_structured_input_for_primitive!(u32);
impl_structured_input_for_primitive!(i32);
impl_structured_input_for_primitive!(u64);
impl_structured_input_for_primitive!(i64);
