use crate::StructuredInput;

impl<T> StructuredInput for Option<T>
where
    T: StructuredInput,
{
    fn complexity(&self) -> usize {
        match self {
            Some(inner) => 1 + inner.complexity(),
            None => 1,
        }
    }

    fn count_data<D2>(&self) -> u32
    where
        D2: StructuredInput,
    {
        let mut count = if core::any::TypeId::of::<Option<T>>() == core::any::TypeId::of::<D2>() {
            1
        } else {
            0
        };

        if let Some(inner) = self {
            count += inner.count_data::<D2>();
        }

        count
    }

    fn sample_data<D2>(&self, mut idx: u32) -> Option<D2>
    where
        D2: StructuredInput,
    {
        if core::any::TypeId::of::<Option<T>>() == core::any::TypeId::of::<D2>() {
            if idx == 0 {
                return Some(unsafe {
                    let ptr = self as *const _ as *const D2;
                    (*ptr).clone()
                });
            }
            idx -= 1;
        }

        if let Some(inner) = self {
            return inner.sample_data::<D2>(idx);
        }

        None
    }
}
