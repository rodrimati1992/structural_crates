use std::marker::PhantomData;



/// A wrapper type that associates a mutable raw pointer with a lifetime.
#[repr(transparent)]
#[derive(Debug)]
pub struct MutRef<'a,T:?Sized>{
    pub ptr:*mut T,
    _marker:PhantomData<&'a mut T>,
}

impl<'a,T:?Sized> Clone for MutRef<'a,T>{
    fn clone(&self)->Self{
        Self{
            ptr:self.ptr,
            _marker:PhantomData,
        }
    }
}

impl<'a,T:?Sized> MutRef<'a,T>{
    #[inline]
    pub fn new(mut_ref:&'a mut T)->Self{
        Self{
            ptr:mut_ref,
            _marker:PhantomData,
        }
    }
}

impl<'a,T:?Sized> From<&'a mut T> for MutRef<'a,T>{
    #[inline]
    fn from(mutref:&'a mut T)->Self{
        Self::new(mutref)
    }
}