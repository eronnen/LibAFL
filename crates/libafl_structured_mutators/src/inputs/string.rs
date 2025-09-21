use crate::StructuredInput;

impl StructuredInput for alloc::string::String {
    fn complexity(&self) -> u64 {
        1 + self.len() as u64
    }

    fn count_data<D2>(&self) -> u32
    where
        D2: StructuredInput,
    {
        if core::any::TypeId::of::<alloc::string::String>() == core::any::TypeId::of::<D2>() {
            1
        } else {
            0
        }
    }

    fn sample_data<D2>(&self, _idx: u32) -> Option<D2>
    where
        D2: StructuredInput,
    {
        if core::any::TypeId::of::<alloc::string::String>() == core::any::TypeId::of::<D2>() {
            Some(unsafe {
                let ptr = self as *const _ as *const D2;
                (*ptr).clone()
            })
        } else {
            None
        }
    }
}
