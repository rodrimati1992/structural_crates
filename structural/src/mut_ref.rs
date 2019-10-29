/*!
A wrapper type for a mutable pointer with a lifetime.
*/

use std_::marker::PhantomData;



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
    /// Constructs a MutRef from a mutable reference.
    #[inline]
    pub fn new(mut_ref:&'a mut T)->Self{
        Self{
            ptr:mut_ref,
            _marker:PhantomData,
        }
    }

    /// Constructs a MutRef from a mutable pointer.
    pub fn from_ptr(ptr:*mut T)->Self{
        Self{
            ptr,
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