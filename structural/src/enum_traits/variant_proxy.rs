use crate::{
    field_traits::{
        variant_field::{GetVariantFieldImpl, GetVariantFieldMutImpl, IntoVariantFieldImpl},
        FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl,
    },
    type_level::{FieldPath1, UncheckedVariantField, VariantFieldPath},
};

#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Deref,
};

/// Wraps an enum,guaranteeing that it's a particular variant.
///
/// These are 3 ways to construct a VariantProxy:
///
/// - Safe: Calling GetFieldExt methods with `fp!(::VariantName)` as an argument,
/// which returns an `Option<VariantProxy<Self,FP!(VariantName)>>`
/// (reference methods return a reference to a VariantProxy).
///
/// - Safe: Calling an EnumExt method with `fp!(VariantName)` as an argument
///
/// - Unsafe: Calling a VariantProxy constructor function.
///
/// # Generic parameters
///
/// `T` is the enum this wraps.
///
/// `V` is the name of the wrapped variant (example type:`FP!(Bar)`).
///
///
/// # Example
///
/// ```
/// use structural::{fp,field_path_aliases,GetFieldExt,Structural};
/// use structural::enum_traits::VariantProxy;
///
/// #[derive(Debug,PartialEq,Structural,Copy,Clone)]
/// #[struc(no_trait)]
/// enum Foo{
///     Bar(u32,&'static str),
///     Baz(u32),
/// }
///
/// field_path_aliases!{
///     mod paths{ Bar,field_0=0, field_1=1 }
/// }
///
/// let this=Foo::Bar(0,"hello");
/// let mut proxy: VariantProxy<Foo, paths::Bar>= unsafe{
///     VariantProxy::new(this, paths::Bar)
/// };
///
/// assert_eq!( proxy.field_(paths::field_0), &0);
/// assert_eq!( proxy.field_(paths::field_1), &"hello");
///
/// assert_eq!( proxy.field_mut(paths::field_0), &mut 0);
/// assert_eq!( proxy.field_mut(paths::field_1), &mut "hello");
///
/// assert_eq!( proxy.into_field(paths::field_0), 0);
/// assert_eq!( proxy.into_field(paths::field_1), "hello");
///
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct VariantProxy<T: ?Sized, V> {
    _marker: PhantomData<V>,
    value: T,
}

impl<T: ?Sized, V> VariantProxy<T, FieldPath1<V>> {
    /// Constructs this VariantProxy from an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,field_path_aliases,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u64),
    /// }
    ///
    /// field_path_aliases!{
    ///     mod paths{ Bar }
    /// }
    ///
    /// let proxy: VariantProxy<Foo, paths::Bar>= unsafe{
    ///     VariantProxy::new(Foo::Bar(0), paths::Bar)
    /// };
    ///
    /// assert_eq!( proxy.into_field(fp!(0)), 0 );
    ///
    /// ```
    #[inline(always)]
    pub const unsafe fn new(value: T, _: FieldPath1<V>) -> Self
    where
        T: Sized,
    {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    /// Constructs this VariantProxy from a boxed enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,field_path_aliases,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(&'static str),
    /// }
    ///
    /// field_path_aliases!{
    ///     mod paths{ Baz }
    /// }
    ///
    /// let this=Box::new(Foo::Baz("hello"));
    /// let proxy: VariantProxy<Box<Foo>, paths::Baz>= unsafe{
    ///     VariantProxy::from_box(this, paths::Baz)
    /// };
    ///
    /// assert_eq!( proxy.into_field(fp!(0)), "hello" );
    ///
    /// ```
    #[inline(always)]
    #[cfg(feature = "alloc")]
    pub unsafe fn from_box(
        value: Box<T>,
        vari: FieldPath1<V>,
    ) -> VariantProxy<Box<T>, FieldPath1<V>> {
        VariantProxy::new(value, vari)
    }

    /// Constructs this VariantProxy from a reference to an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32,bool),
    ///     Baz(u32),
    /// }
    ///
    /// // FP!(B a r) can also be FP!(Bar) from Rust 1.40 onwards
    /// let proxy: &VariantProxy<Foo, FP!(B a r)>= unsafe{
    ///     VariantProxy::from_ref(&Foo::Bar(99,false), fp!(Bar))
    /// };
    ///
    /// assert_eq!( proxy.field_(fp!(0)), &99 );
    /// assert_eq!( proxy.field_(fp!(1)), &false );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn from_ref(reference: &T, _: FieldPath1<V>) -> &Self {
        &*(reference as *const T as *const VariantProxy<T, FieldPath1<V>>)
    }

    /// Constructs this VariantProxy from a mutable reference to the enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(Option<usize>,Vec<String>),
    /// }
    ///
    /// let mut this=Foo::Baz(Some(1),vec![]);
    ///
    /// // FP!(B a z) can also be FP!(Baz) from Rust 1.40 onwards
    /// let proxy: &mut VariantProxy<Foo, FP!(B a z)>= unsafe{
    ///     VariantProxy::from_mut(&mut this, fp!(Baz))
    /// };
    ///
    /// assert_eq!( proxy.field_mut(fp!(0)), &mut Some(1) );
    /// assert_eq!( proxy.field_mut(fp!(1)), &mut Vec::<String>::new() );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn from_mut(reference: &mut T, vari: FieldPath1<V>) -> &mut Self {
        &mut *Self::from_raw_mut(reference, vari)
    }

    /// Constructs this VariantProxy from a raw pointer to the enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// use std::cmp::Ordering;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(Ordering,&'static [u8]),
    /// }
    ///
    /// let mut this=Foo::Baz( Ordering::Less, &[0,1,2,3] );
    ///
    /// // FP!(B a z) can also be FP!(Baz) from Rust 1.40 onwards
    /// let proxy: *mut VariantProxy<Foo, FP!(B a z)>= unsafe{
    ///     VariantProxy::from_raw_mut(&mut this as *mut Foo, fp!(Baz))
    /// };
    ///
    /// unsafe{
    ///     assert_eq!( (*proxy).field_mut(fp!(0)), &mut Ordering::Less );
    ///     assert_eq!( (*proxy).field_mut(fp!(1)), &mut &[0,1,2,3] );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub const unsafe fn from_raw_mut(ptr: *mut T, _: FieldPath1<V>) -> *mut Self {
        ptr as *mut VariantProxy<T, FieldPath1<V>>
    }

    /// Gets a reference to the wrapped enum.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u32),
    /// }
    ///
    /// // FP!(B a r) can also be FP!(Bar) from Rust 1.40 onwards
    /// let proxy: VariantProxy<Foo, FP!(B a r)>= unsafe{
    ///     VariantProxy::new(Foo::Bar(0), fp!(Bar))
    /// };
    ///
    /// assert_eq!(proxy.get() , &Foo::Bar(0));
    ///
    /// ```
    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Gets a mutable reference to the wrapped enum.
    ///
    /// # Safety
    ///
    /// You must not change the variant of the wrapped enum,
    /// since VariantProxy relies on it being the one that the `V`
    /// generic parmeter specifies
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u32),
    /// }
    ///
    /// let mut this=Foo::Baz(0);
    ///
    /// // FP!(B a z) can also be FP!(Baz) from Rust 1.40 onwards
    /// let proxy: &mut VariantProxy<Foo, FP!(B a z)>= unsafe{
    ///     VariantProxy::from_mut(&mut this, fp!(Baz))
    /// };
    ///
    /// assert_eq!(unsafe{ proxy.get_mut() }, &mut Foo::Baz(0));
    ///
    /// ```
    pub unsafe fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Unwraps this VariantProxy into the enum it wraps.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u32),
    /// }
    ///
    /// // FP!(B a r) can also be FP!(Bar) from Rust 1.40 onwards
    /// let proxy: VariantProxy<Foo, FP!(B a r)>= unsafe{
    ///     VariantProxy::new(Foo::Bar(0), fp!(Bar))
    /// };
    ///
    /// assert_eq!(proxy.into_inner() , Foo::Bar(0));
    ///
    /// ```
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.value
    }

    /// Gets a mutable raw pointer to the wrapped enum.
    ///
    /// # Safety
    ///
    /// You must not change the variant of the wrapped enum,
    /// because VariantProxy relies on it being the one that the `V`
    /// generic parmaetere specifies
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enum_traits::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u32),
    /// }
    ///
    /// let mut this=Foo::Baz(0);
    ///
    /// // FP!(B a z) can also be FP!(Baz) from Rust 1.40 onwards
    /// let proxy: *mut VariantProxy<Foo, FP!(B a z)>= unsafe{
    ///     VariantProxy::from_raw_mut(&mut this as *mut Foo, fp!(Baz))
    /// };
    ///
    /// assert_eq!(unsafe{  &mut *VariantProxy::as_raw_mut(proxy) }, &mut Foo::Baz(0));
    ///
    /// ```
    pub unsafe fn as_raw_mut(this: *mut Self) -> *mut T {
        this as *mut T
    }
}

impl<T: ?Sized, V> Deref for VariantProxy<T, V> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, V> Debug for VariantProxy<T, V>
where
    T: ?Sized + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantProxy")
            .field("value", &&self.value)
            .finish()
    }
}

impl<T: ?Sized, V> crate::IsStructural for VariantProxy<T, FieldPath1<V>> {}

impl<T, V, F> FieldType<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: ?Sized + FieldType<VariantFieldPath<V, F>>,
{
    type Ty = T::Ty;
}

impl<T, V, F> GetFieldImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: ?Sized + GetVariantFieldImpl<V, F>,
{
    type Err = T::Err;

    #[inline(always)]
    fn get_field_(&self, _: FieldPath1<F>, _: ()) -> Result<&T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.get_field_(
                VariantFieldPath::<V, F>::NEW,
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }
}

unsafe impl<T, V, F> GetFieldMutImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: ?Sized + GetVariantFieldMutImpl<V, F>,
{
    #[inline(always)]
    fn get_field_mut_(&mut self, _: FieldPath1<F>, _: ()) -> Result<&mut T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.get_field_mut_(
                VariantFieldPath::<V, F>::NEW,
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    default_if! {
        #[inline(always)]
        cfg(feature="specialization")
        unsafe fn get_field_raw_mut(
            this: *mut (),
            _: FieldPath1<F>,
            _: (),
        ) -> Result<*mut T::Ty, T::Err>
        where
            Self: Sized
        {
            let func=(&**(this as *mut Self)).get_field_raw_mut_func();
            // safety: VariantProxy<T,V> guarantees that it wraps an enum
            // with the variant that `V` names.
            func(
                this,
                VariantFieldPath::<V, F>::new(),
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    #[inline(always)]
    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<FieldPath1<F>, (), T::Ty, T::Err> {
        // safety:
        // This transmute should be sound,
        // since every parameter of `GetFieldMutImpl::get_field_mut_`
        // except for `this: *mut ()` is an zero sized type,
        // and this converts those parameters to other zero sized types.
        unsafe {
            std::mem::transmute::<
                GetFieldRawMutFn<
                    VariantFieldPath<V, F>,
                    UncheckedVariantField<V, F>,
                    T::Ty,
                    T::Err,
                >,
                GetFieldRawMutFn<FieldPath1<F>, (), T::Ty, T::Err>,
            >((**self).get_field_raw_mut_func())
        }
    }
}

#[cfg(feature = "specialization")]
unsafe impl<T, V, F> GetFieldMutImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: GetVariantFieldMutImpl<V, F>,
{
    unsafe fn get_field_raw_mut(
        this: *mut (),
        _: FieldPath1<F>,
        _: (),
    ) -> Result<*mut T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        T::get_field_raw_mut(
            this,
            VariantFieldPath::<V, F>::new(),
            UncheckedVariantField::<V, F>::new(),
        )
    }
}

impl<T, V, F> IntoFieldImpl<FieldPath1<F>> for VariantProxy<T, FieldPath1<V>>
where
    T: IntoVariantFieldImpl<V, F>,
{
    #[inline(always)]
    fn into_field_(self, _: FieldPath1<F>, _: ()) -> Result<T::Ty, T::Err>
    where
        Self: Sized,
    {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.into_field_(
                VariantFieldPath::<V, F>::NEW,
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    z_impl_box_into_field_method! {FieldPath1<F>,T::Ty,T::Err}
}
