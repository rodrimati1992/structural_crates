use crate::{
    field_traits::{
        FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl, OptionalField,
    },
    type_level::to_value_traits::ToUsize,
    IsStructural,
};

#[cfg(test)]
mod tests;

macro_rules! slice_getter_impls {
    (
        impl[$($impl_params:tt)*] GetFieldImpl for $self:ty { type Ty=$ty:ty; }
    ) => (
        impl<$($impl_params)*> IsStructural for $self{}

        impl<$($impl_params)* P> FieldType<P> for $self {
            type Ty=$ty;
        }

        impl<$($impl_params)* P> GetFieldImpl<P> for $self
        where
            P: ToUsize,
        {
            type Err=OptionalField;

            #[inline(always)]
            fn get_field_(&self,_:P,_:())->Result<&$ty,OptionalField>{
                self.get(P::USIZE).ok_or(OptionalField)
            }
        }
    );
    (
        impl[$($impl_params:tt)*] GetFieldMutImpl for $self:ty {
            type Ty=$ty:ty;

            unsafe fn get_field_raw_mut($this:ident,$name:ident){
                $($gfrm_impl:tt)*
            }
        }
    ) => (
        slice_getter_impls!{
            impl[$($impl_params)*] GetFieldImpl for $self {
                type Ty=$ty;
            }
        }

        unsafe impl<$($impl_params)* P> GetFieldMutImpl<P> for $self
        where
            P: ToUsize,
        {
            fn get_field_mut_(&mut self,_: P,_: ()) -> Result<&mut $ty, OptionalField>{
                self.get_mut(P::USIZE).ok_or(OptionalField)
            }

            unsafe fn get_field_raw_mut(
                this: *mut (),
                $name:P,
                _:()
            ) -> Result<*mut $ty, OptionalField>{
                let $this=this as *mut Self;
                $($gfrm_impl)*
            }

            fn get_field_raw_mut_func(
                &self
            ) -> GetFieldRawMutFn<P, (), $ty, OptionalField>{
                <Self as GetFieldMutImpl<P>>::get_field_raw_mut
            }

        }
    );
}

slice_getter_impls! {
    impl['a,T:'a,] GetFieldImpl for &'a [T] {
        type Ty=T;
    }
}

slice_getter_impls! {
    impl['a,T:'a,] GetFieldMutImpl for &'a mut [T] {
        type Ty=T;

        unsafe fn get_field_raw_mut(this,index){
            get_raw_mut( *(this as *mut *mut [T]), <P as ToUsize>::USIZE )
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;

    use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};

    slice_getter_impls! {
        impl[T,] GetFieldMutImpl for Box<[T]> {
            type Ty=T;

            unsafe fn get_field_raw_mut(this,index){
                get_raw_mut(
                    *(this as *mut Box<[T]> as *mut *mut [T]),
                    <P as ToUsize>::USIZE,
                )
            }
        }
    }

    slice_getter_impls! {
        impl[T,] GetFieldImpl for Rc<[T]> {
            type Ty=T;
        }
    }

    slice_getter_impls! {
        impl[T,] GetFieldImpl for Arc<[T]> {
            type Ty=T;
        }
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
