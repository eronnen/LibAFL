use crate::StructuredInput;

impl<T> StructuredInput for alloc::vec::Vec<T>
where
    T: StructuredInput,
{
    fn complexity(&self) -> usize {
        1 + self.iter().map(|item| 1 + item.complexity()).sum::<usize>()
    }

    fn count_data<D2>(&self) -> u32
    where
        D2: StructuredInput,
    {
        let mut count =
            if core::any::TypeId::of::<alloc::vec::Vec<T>>() == core::any::TypeId::of::<D2>() {
                1
            } else {
                0
            };

        count += self.iter().map(|item| item.count_data::<D2>()).sum::<u32>();
        count
    }

    fn sample_data<D2>(&self, mut idx: u32) -> Option<D2>
    where
        D2: StructuredInput,
    {
        if core::any::TypeId::of::<alloc::vec::Vec<T>>() == core::any::TypeId::of::<D2>() {
            if idx == 0 {
                return Some(unsafe {
                    let ptr = self as *const _ as *const D2;
                    (*ptr).clone()
                });
            }
            idx -= 1;
        }

        for item in self {
            let current_sample_size = item.count_data::<D2>();
            if idx < current_sample_size {
                return item.sample_data::<D2>(idx);
            }
            idx -= current_sample_size;
        }

        None
    }
}
