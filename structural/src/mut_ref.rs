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

impl<'a,T:?Sized> Copy for MutRef<'a,T>{}

impl<'a,T:?Sized> Clone for MutRef<'a,T>{
    #[inline(always)]
    fn clone(&self)->Self{
        *self
    }
}

impl<'a,T:?Sized> MutRef<'a,T>{
    /// Constructs a MutRef from a mutable reference.
    #[inline(always)]
    pub fn new(mut_ref:&'a mut T)->Self{
        Self{
            ptr:mut_ref,
            _marker:PhantomData,
        }
    }

    /// Constructs a MutRef from a mutable pointer.
    #[inline(always)]
    pub fn from_ptr(ptr:*mut T)->Self{
        Self{
            ptr,
            _marker:PhantomData,
        }
    }

    /// An unchecked cast from `MutRef<'a,T>` to `MutRef<'a,U>`.
    #[inline(always)]
    pub fn cast<U>(self)->MutRef<'a,U>{
        MutRef{
            ptr:self.ptr as *mut U,
            _marker:PhantomData,
        }
    }
}

impl<'a,T:?Sized> From<&'a mut T> for MutRef<'a,T>{
    #[inline(always)]
    fn from(mutref:&'a mut T)->Self{
        Self::new(mutref)
    }
}