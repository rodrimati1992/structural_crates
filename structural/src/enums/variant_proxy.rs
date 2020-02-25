use crate::{
    field_path::{FieldPath, IsTStr, TStr, UncheckedVariantField, VariantField},
    field_traits::{
        variant_field::{GetVariantFieldImpl, GetVariantFieldMutImpl, IntoVariantFieldImpl},
        FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl,
    },
};

#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
};

/// Enum wrapper,for accessing the fields of a particular variant
/// (determined by the `V` type parameter).
///
/// Example type: `VariantProxy<Enum,FP!(Foo)>`,`Foo` being the name of the variant.
///
/// # Construction
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
/// This example constructs a `VariantProxy` safely.
///
/// ```
/// use structural::{fp,field_path_aliases,GetFieldExt,Structural};
/// use structural::enums::{EnumExt,VariantProxy};
///
/// field_path_aliases!{
///     mod paths{ Bar }
/// }
///
/// with_bar(Foo::Bar(0,"hello"));
///
/// with_bar(Foo2::Bar(0,"hello",true));
///
/// /// Runs some assertions.
/// ///
/// /// # Panics
/// ///
/// /// This functin panics if you pass an enum whose current variant is **not** `Bar`.
/// fn with_bar<T>(this: T)
/// where
///     // `Foo_ESI` was generates by the `Structural` derive,
///     // it aliases the accessor traits for `Foo`,
///     // and also requires an enum with the same amount of variants as `Foo`
///     //
///     // `Debug` is required to print the enum in the `.expect(...)` error.
///     T: Foo_ESI + Copy + std::fmt::Debug
/// {
///     let mut proxy: VariantProxy<T, paths::Bar>=
///         this.into_variant(paths::Bar)
///             .expect("Expected the `Bar` variant to be passed in");
///    
///     assert_eq!( proxy.field_(fp!(0)), &0);
///     assert_eq!( proxy.field_(fp!(1)), &"hello");
///    
///     assert_eq!( proxy.field_mut(fp!(0)), &mut 0);
///     assert_eq!( proxy.field_mut(fp!(1)), &mut "hello");
///    
///     assert_eq!( proxy.into_field(fp!(0)), 0);
///     assert_eq!( proxy.into_field(fp!(1)), "hello");
/// }
///
/// #[derive(Debug,PartialEq,Structural,Copy,Clone)]
/// enum Foo{
///     Bar(u32,&'static str),
///     Baz(u32),
/// }
///
/// #[derive(Debug,PartialEq,Structural,Copy,Clone)]
/// # #[struc(no_trait)]
/// enum Foo2{
///     Bar(u32,&'static str,bool),
///     Baz(u32,u64),
/// }
///
/// ```
///
/// # Example
///
/// This example uses an `unsafe` constructor.
///
/// ```
/// use structural::{fp,field_path_aliases,GetFieldExt,Structural};
/// use structural::enums::VariantProxy;
///
/// field_path_aliases!{
///     mod paths{ Bar }
/// }
///
/// unsafe{
///     with_bar(Foo::Bar(0,"hello"));
///
///     with_bar(Foo2::Bar(0,"hello",true));
/// }
///
/// /// Runs some assertions.
/// ///
/// /// # Safety
/// ///
/// /// You must pass an enum whose current variant is `Bar`,
/// /// `this.is_variant(fp!(Foo))` (the method is from `GetFieldExt`) must return true.
/// unsafe fn with_bar<T>(this: T)
/// where
///     // `Foo_ESI` was generates by the `Structural` derive,
///     // it aliases the accessor traits for `Foo`,
///     // and also requires an enum with the same amount of variants as `Foo`
///     T: Foo_ESI + Copy
/// {
///     let mut proxy: VariantProxy<T, paths::Bar>=
///         VariantProxy::new(this, paths::Bar);
///    
///     assert_eq!( proxy.field_(fp!(0)), &0);
///     assert_eq!( proxy.field_(fp!(1)), &"hello");
///    
///     assert_eq!( proxy.field_mut(fp!(0)), &mut 0);
///     assert_eq!( proxy.field_mut(fp!(1)), &mut "hello");
///    
///     assert_eq!( proxy.into_field(fp!(0)), 0);
///     assert_eq!( proxy.into_field(fp!(1)), "hello");
/// }
///
/// #[derive(Debug,PartialEq,Structural,Copy,Clone)]
/// enum Foo{
///     Bar(u32,&'static str),
///     Baz(u32),
/// }
///
/// #[derive(Debug,PartialEq,Structural,Copy,Clone)]
/// # #[struc(no_trait)]
/// enum Foo2{
///     Bar(u32,&'static str,bool),
///     Baz(u32,u64),
/// }
///
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct VariantProxy<T: ?Sized, V> {
    _marker: PhantomData<V>,
    value: T,
}

impl<T: ?Sized, V> VariantProxy<T, TStr<V>> {
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
    /// use structural::enums::VariantProxy;
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
    pub const unsafe fn new(value: T, name: TStr<V>) -> Self
    where
        T: Sized,
    {
        let _ = ManuallyDrop::new(name);
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
    /// use structural::enums::VariantProxy;
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
    pub unsafe fn from_box(value: Box<T>, vari: TStr<V>) -> VariantProxy<Box<T>, TStr<V>> {
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
    /// use structural::enums::VariantProxy;
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
    pub unsafe fn from_ref(reference: &T, _: TStr<V>) -> &Self {
        &*(reference as *const T as *const VariantProxy<T, TStr<V>>)
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
    /// use structural::enums::VariantProxy;
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
    pub unsafe fn from_mut(reference: &mut T, vari: TStr<V>) -> &mut Self {
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
    /// use structural::enums::VariantProxy;
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
    pub const unsafe fn from_raw_mut(ptr: *mut T, name: TStr<V>) -> *mut Self {
        let _ = ManuallyDrop::new(name);
        ptr as *mut VariantProxy<T, TStr<V>>
    }

    /// Gets a reference to the wrapped enum.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt,Structural};
    /// use structural::enums::VariantProxy;
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
    /// use structural::enums::VariantProxy;
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
    /// use structural::enums::VariantProxy;
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
    /// use structural::enums::VariantProxy;
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

impl<T: ?Sized, V> Deref for VariantProxy<T, V>
where
    V: IsTStr,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, V> Debug for VariantProxy<T, V>
where
    T: ?Sized + Debug,
    V: IsTStr,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VariantProxy")
            .field("value", &&self.value)
            .finish()
    }
}

impl<T: ?Sized, V> crate::IsStructural for VariantProxy<T, V> where V: IsTStr {}

impl<T, V, F> FieldType<F> for VariantProxy<T, V>
where
    T: ?Sized + FieldType<VariantField<V, F>>,
    V: IsTStr,
{
    type Ty = T::Ty;
}

impl<T, V, F> GetFieldImpl<F> for VariantProxy<T, V>
where
    T: ?Sized + GetVariantFieldImpl<V, F>,
    V: IsTStr,
{
    type Err = T::Err;

    #[inline(always)]
    fn get_field_(&self, path: F, _: ()) -> Result<&T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.get_field_(
                VariantField::new(V::MTVAL, path),
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }
}

unsafe impl<T, V, F> GetFieldMutImpl<F> for VariantProxy<T, V>
where
    T: ?Sized + GetVariantFieldMutImpl<V, F>,
    V: IsTStr,
{
    #[inline(always)]
    fn get_field_mut_(&mut self, path: F, _: ()) -> Result<&mut T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.get_field_mut_(
                VariantField::new(V::MTVAL, path),
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    default_if! {
        #[inline(always)]
        cfg(feature="specialization")
        unsafe fn get_field_raw_mut(
            this: *mut (),
            path: F,
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
                VariantField::new(V::MTVAL, path),
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    #[inline(always)]
    fn get_field_raw_mut_func(&self) -> GetFieldRawMutFn<F, (), T::Ty, T::Err> {
        // safety:
        // This transmute should be sound,
        // since every parameter of `GetFieldMutImpl::get_field_mut_`
        // except for `this: *mut ()` is an zero sized type,
        // and this converts those parameters to other zero sized types.
        unsafe {
            std::mem::transmute::<
                GetFieldRawMutFn<VariantField<V, F>, UncheckedVariantField<V, F>, T::Ty, T::Err>,
                GetFieldRawMutFn<F, (), T::Ty, T::Err>,
            >((**self).get_field_raw_mut_func())
        }
    }
}

#[cfg(feature = "specialization")]
unsafe impl<T, V, F> GetFieldMutImpl<F> for VariantProxy<T, V>
where
    T: GetVariantFieldMutImpl<V, F>,
    V: IsTStr,
{
    unsafe fn get_field_raw_mut(this: *mut (), path: F, _: ()) -> Result<*mut T::Ty, T::Err> {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        T::get_field_raw_mut(
            this,
            VariantField::new(V::MTVAL, path),
            UncheckedVariantField::<V, F>::new(),
        )
    }
}

impl<T, V, F> IntoFieldImpl<F> for VariantProxy<T, V>
where
    T: IntoVariantFieldImpl<V, F>,
    V: IsTStr,
{
    #[inline(always)]
    fn into_field_(self, path: F, _: ()) -> Result<T::Ty, T::Err>
    where
        Self: Sized,
    {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe {
            self.value.into_field_(
                VariantField::new(V::MTVAL, path),
                UncheckedVariantField::<V, F>::new(),
            )
        }
    }

    z_impl_box_into_field_method! {F,T::Ty,T::Err}
}
