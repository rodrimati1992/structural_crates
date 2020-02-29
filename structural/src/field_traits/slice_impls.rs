use crate::{
    field_traits::{FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, OptionalField},
    type_level::to_value_traits::ToUsize,
};

#[cfg(test)]
mod tests;

impl<'a, T, P> FieldType<P> for [T] {
    type Ty = T;
}

impl<'a, T, P> GetFieldImpl<P> for [T]
where
    P: ToUsize,
{
    type Err = OptionalField;

    #[inline(always)]
    fn get_field_(&self, _: P, _: ()) -> Result<&T, OptionalField> {
        self.get(P::USIZE).ok_or(OptionalField)
    }
}

unsafe impl<'a, T, P> GetFieldMutImpl<P> for [T]
where
    P: ToUsize,
{
    fn get_field_mut_(&mut self, _: P, _: ()) -> Result<&mut T, OptionalField> {
        self.get_mut(P::USIZE).ok_or(OptionalField)
    }

    unsafe fn get_field_raw_mut(this: *mut *mut (), _: P, _: ()) -> Result<*mut T, OptionalField> {
        loop {}
    }

    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<P, (), T, OptionalField> {
        |mut this: *mut *mut (), name: P, _: ()| unsafe {
            let this = this as *mut *mut [T];
            get_raw_mut(*this, P::USIZE)
        }
    }
}

#[cfg(feature = "specialization")]
unsafe impl<'a, T: 'a, P> super::SpecGetFieldMut<P> for &'a mut [T]
where
    P: ToUsize,
{
    unsafe fn get_field_raw_mut_inner(
        this: *mut *mut (),
        name: P,
        _: (),
    ) -> Result<*mut T, OptionalField> {
        let ptr = **(this as *mut *mut *mut [T]);
        get_raw_mut(ptr, P::USIZE)
    }
}

unsafe fn get_raw_mut<T>(this: *mut [T], index: usize) -> Result<*mut T, OptionalField> {
    let len = (*this).len();
    let ptr = this as *mut T;
    if index < len {
        Ok(ptr.add(index))
    } else {
        Err(OptionalField)
    }
}
