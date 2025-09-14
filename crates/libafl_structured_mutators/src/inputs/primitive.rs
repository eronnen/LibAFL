use core::any::TypeId;

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
                if TypeId::of::<$primitive>() == TypeId::of::<D2>() {
                    1
                } else {
                    0
                }
            }

            fn sample_data<D2>(&self, _idx: u32) -> Option<D2>
            where
                D2: StructuredInput,
            {
                if TypeId::of::<$primitive>() == TypeId::of::<D2>() {
                    let cloned = self.clone();
                    let casted = unsafe { core::ptr::read(&cloned as *const Self as *const D2) };
                    Some(casted)
                } else {
                    None
                }
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
