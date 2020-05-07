use super::{DropFields, MovedOutFields, PrePostDropFields};

use std_::mem::ManuallyDrop;

/////////////////////////////////////////////////////////////////////////////////

/// Helper type for moving fields out of a Structural type.
///
/// # Drop behavior
///
/// The wrapped value is put inside a `ManuallyDrop` so that its destructor doesn't run.
///
/// When this is dropped,instead of running the destructor for the wrapped value,
/// this calls [`DropFields::drop_fields`] on it,
/// to drop the fields that haven't been moved out.
///
/// [`DropFields::drop_fields`]: ./trait.DropFields.html#tymethod.drop_fields
///
/// # Example
///
/// For an example that uses `IntoFieldsWrapper` there's the
/// [implementation example] for [`RevIntoMultiFieldImpl`]
///
/// [implementation example]:
/// ../multi_fields/trait.RevIntoMultiFieldImpl.html#implementation-example
///
/// [`RevIntoMultiFieldImpl`]: ../multi_fields/trait.RevIntoMultiFieldImpl.html
///
pub struct IntoFieldsWrapper<T: DropFields> {
    value: ManuallyDrop<T>,
    moved: MovedOutFields,
}

impl<T: DropFields> IntoFieldsWrapper<T> {
    /// Constructs this `IntoFieldsWrapper`,wrapping the `value`.
    ///
    /// Also calls `DropFields::pre_move` on the wrapped value
    ///
    #[inline(always)]
    pub fn new(mut value: T) -> Self {
        DropFields::pre_move(&mut value);
        Self {
            value: ManuallyDrop::new(value),
            moved: MovedOutFields::new(),
        }
    }

    /// Gets mutable references to the wrapped value,
    /// and the `MovedOutFields` that tracks which fields were moved out of it.
    ///
    /// # Safety
    ///
    /// The returned references must not be mutated,
    /// only passed to accessor traits declared in structural.
    ///
    /// Mutating `MovedOutFields` incorrectly can lead to leaks and double dropped fields.
    #[inline(always)]
    pub unsafe fn inner_and_moved_mut(&mut self) -> (&mut T, &mut MovedOutFields) {
        (&mut self.value, &mut self.moved)
    }

    /// Gets mutable pointers to the wrapped value,
    /// and the `MovedOutFields` that tracks which fields were moved out of it.
    ///
    /// # Safety
    ///
    /// The returned pointers must not be mutated,
    /// only passed to accessor traits declared in structural.
    ///
    /// Mutating `MovedOutFields` incorrectly can lead to leaks and double dropped fields.
    #[inline(always)]
    pub unsafe fn inner_and_moved_raw(&mut self) -> (*mut T, *mut MovedOutFields) {
        (&mut *self.value as *mut T, &mut self.moved as *mut _)
    }
}

impl<T: DropFields> Drop for IntoFieldsWrapper<T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            DropFields::drop_fields(&mut *self.value, self.moved);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

macro_rules! declare_run_on_drop {
    (
        $(#[$meta:meta])*
        struct $struct:ident
        $(where[$($where_preds:tt)*])?
        $(#[$new_meta:meta])*
        $(unsafe $(#$dummy:ident#)?)? fn new($($extra_var:ident : $extra_ty:ty),* $(,)?)
        this=$this:ident,
        fn drop(){$($drop:tt)*}
    ) => (
        $(#[$meta])*
        pub struct $struct<'a,T>
        $(where $($where_preds)*)?
        {
            mutref:&'a mut T,
            $($extra_var : $extra_ty,)*
        }

        impl<'a,T> $struct<'a,T>
        $(where $($where_preds)*)?
        {
            $(#[$new_meta])*
            /// # Drop order
            ///
            /// Remember that variables on the stack are dropped in the opposite order
            /// than they are declared.
            ///
            /// In this example:
            /// ```ignore
            /// let a=Foo;
            /// let b=Bar;
            /// let c=Baz;
            /// ```
            /// `c` gets dropped first,then `b`, then `a`.
            #[inline(always)]
            pub $(unsafe $(#$dummy#)?)?
            fn new(mutref:&'a mut T $(,$extra_var : $extra_ty)*)->Self{
                Self{
                    mutref,
                    $($extra_var,)*
                }
            }

            /// Reborrows the wrapped mutable reference.
            #[inline(always)]
            pub fn get_mut(&mut self)->&mut T{
                self.mutref
            }
        }

        impl<'a,T> Drop for  $struct<'a,T>
        $(where $($where_preds)*)?
        {
            #[inline(always)]
            fn drop(&mut self){
                let $this=self;
                $($drop)*
            }
        }

    )
}

declare_run_on_drop! {
    /// A guard that drops the value that a mutable reference points when *it* is dropped.
    struct RunDrop

    /// Constructs this RunDrop.
    ///
    /// # Safety
    ///
    /// In the destructor for this type,
    /// this drops the value that the mutable reference points to.
    ///
    /// Once the destructor for this type runs,the pointed-to value must not be used again,
    /// that includes the destructor for the value running.
    fn new()

    this=this,
    fn drop(){
        unsafe{
            std_::ptr::drop_in_place(this.mutref)
        }
    }
}

declare_run_on_drop! {
    /// A guard that calls [`PrePostDropFields::post_drop`] on the mutable reference
    /// when *it* is dropped.
    ///
    /// [`PrePostDropFields::post_drop`]: ./trait.PrePostDropFields.html#tymethod.post_drop
    struct RunPostDrop
    where[ T: PrePostDropFields ]

    /// Constructs this RunPostDrop.
    ///
    /// # Safety
    ///
    /// This has the same safety requirements as [`PrePostDropFields::post_drop`].
    ///
    /// [`PrePostDropFields::post_drop`]: ./trait.PrePostDropFields.html#tymethod.post_drop
    unsafe fn new()

    this=this,
    fn drop(){
        unsafe{
            PrePostDropFields::post_drop(this.mutref)
        }
    }
}

declare_run_on_drop! {
    /// A guard that calls [`DropFields::drop_fields`] on the mutable reference
    /// when *it* is dropped.
    ///
    /// [`DropFields::drop_fields`]: ./trait.DropFields.html#tymethod.drop_fields
    struct RunDropFields
    where[ T: DropFields ]

    /// Constructs this RunDropFields.
    ///
    /// # Safety
    ///
    /// This has the same safety requirements as [`DropFields::drop_fields`].
    ///
    /// [`DropFields::drop_fields`]: ./trait.DropFields.html#tymethod.drop_fields
    unsafe fn new(moved: MovedOutFields)

    this=this,
    fn drop(){
        unsafe{
            this.mutref.drop_fields(this.moved)
        }
    }
}

impl<'a, T> RunDropFields<'a, T>
where
    T: DropFields,
{
    /// Gets mutable references to the wrapped value,
    /// and the `MovedOutFields` that tracks which fields were moved out of it
    ///
    /// # Safety
    ///
    /// The returned references must not be mutated,
    /// only passed to accessor traits declared in structural.
    ///
    /// Mutating `MovedOutFields` incorrectly can lead to leaks and double dropped fields.
    pub unsafe fn get_mut_and_moved_fields(&mut self) -> (&mut T, &mut MovedOutFields) {
        (&mut self.mutref, &mut self.moved)
    }
}
