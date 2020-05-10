use crate::{
    enums::IsVariant,
    field::{
        DropFields, FieldType, GetField, GetFieldMut, GetFieldRawMutFn, GetVariantField,
        GetVariantFieldMut, IntoField, IntoVariantField, MovedOutFields, SpecGetFieldMut,
    },
    path::{IsTStr, TStr, VariantField},
};

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
};

/// Enum wrapper,for accessing the fields of a particular variant
/// (determined by the `V` type parameter).
///
/// The `V` type parameter is a [TStr](../struct.TStr.html).
///
/// Example type: `VariantProxy<Enum,TS!(Foo)>`,`Foo` being the name of the variant.
///
/// # Construction
///
/// These are 3 ways to construct a VariantProxy:
///
/// - Safe: Calling StructuralExt methods with `fp!(::VariantName)` as an argument,
/// which returns an `Option<VariantProxy<Self,TS!(VariantName)>>`
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
/// `V` is the name of the wrapped variant (example type:`TS!(Bar)`).
///
///
/// # Example
///
/// This example constructs a `VariantProxy` safely.
///
/// ```
/// use structural::{fp,ts,StructuralExt,Structural,TS};
/// use structural::enums::{EnumExt,VariantProxy};
///
/// with_bar(Foo::Bar(0,"hello"));
///
/// with_bar(Foo2::Bar(0,"hello",true));
///
/// /// Runs some assertions.
/// ///
/// /// # Panics
/// ///
/// /// This function panics if you pass an enum whose current variant is **not** `Bar`.
/// fn with_bar<T>(this: T)
/// where
///     // `Foo_ESI` was generated by the `Structural` derive,
///     // it aliases the accessor traits for `Foo`,
///     // and also requires an enum with the same amount of variants as `Foo`
///     //
///     // `Debug` is required to print the enum in the `.expect(...)` error.
///     T: Foo_ESI + Copy + std::fmt::Debug
/// {
///     let mut proxy: VariantProxy<T, TS!(Bar)>=
///         this.into_variant(ts!(Bar))
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
/// use structural::{fp,ts,TS,StructuralExt,Structural};
/// use structural::enums::VariantProxy;
///
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
/// /// `this.is_variant(ts!(Bar))` (the method is from `StructuralExt`) must return true.
/// unsafe fn with_bar<T>(this: T)
/// where
///     // `Foo_ESI` was generated by the `Structural` derive,
///     // it aliases the accessor traits for `Foo`,
///     // and also requires an enum with the same amount of variants as `Foo`
///     T: Foo_ESI + Copy
/// {
///     let mut proxy: VariantProxy<T, TS!(Bar)>=
///         VariantProxy::new(this, ts!(Bar));
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
/// This example demonstrates how you can use a `VariantProxy` to treat a
/// newtype variant as the struct that it wraps.
///
/// ```rust
/// use structural::{StructuralExt, Structural, TS, fp};
/// use structural::enums::VariantProxy;
///
/// // `Point_SI` was generated by the `Structural` derive macro on `Point`,
/// // aliasing the accessor traits that `Point` implements.
/// fn with_bar(this: &impl Point_SI){
///     assert_eq!( this.fields(fp!(x,y)), (&21, &34) );
/// }
///
/// let point=Point{x:21, y:34};
/// with_bar(&point);
///
/// let variant=Foo::Point(point);
/// // The type annotation here isn't necessary.
/// let proxy: &VariantProxy<Foo, TS!(Point)>=
///     variant.field_(fp!(::Point)).expect("it was just constructed as a Foo::Point");
/// with_bar(proxy);
///
///
/// #[derive(Structural)]
/// // The `#[struc(no_trait)]` attribute disables the generation of
/// // the `*_SI` and `*_ESI` traits for this type.
/// #[struc(no_trait)]
/// pub enum Foo{
///     // You would write this as `#[struc(newtype(bounds = "Point_VSI<@variant>"))]`
///     // if you didn't use the  `#[struc(no_trait)]` attribute.
///     #[struc(newtype)]
///     Point(Point),
/// }
///
/// #[derive(Structural)]
/// pub struct Point{
///     pub x:u32,
///     pub y:u32,
/// }
/// ```
///
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
    /// use structural::{fp,ts,TS,field_path_aliases,StructuralExt,Structural};
    /// use structural::enums::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u64),
    /// }
    ///
    /// let proxy: VariantProxy<Foo, TS!(Bar)>= unsafe{
    ///     VariantProxy::new(Foo::Bar(0), ts!(Bar))
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

    /// Constructs this VariantProxy from a reference to an enum.
    ///
    /// # Safety
    ///
    /// `V` must be the name of the wrapped enum variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
    /// use structural::enums::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32,bool),
    ///     Baz(u32),
    /// }
    ///
    /// let proxy: &VariantProxy<Foo, TS!(Bar)>= unsafe{
    ///     VariantProxy::from_ref(&Foo::Bar(99,false), ts!(Bar))
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
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
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
    /// let proxy: &mut VariantProxy<Foo, TS!(Baz)>= unsafe{
    ///     VariantProxy::from_mut(&mut this, ts!(Baz))
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
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
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
    /// let proxy: *mut VariantProxy<Foo, TS!(Baz)>= unsafe{
    ///     VariantProxy::from_raw_mut(&mut this as *mut Foo, ts!(Baz))
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
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
    /// use structural::enums::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u32),
    /// }
    ///
    /// let proxy: VariantProxy<Foo, TS!(Bar)>= unsafe{
    ///     VariantProxy::new(Foo::Bar(0), ts!(Bar))
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
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
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
    /// let proxy: &mut VariantProxy<Foo, TS!(Baz)>= unsafe{
    ///     VariantProxy::from_mut(&mut this, ts!(Baz))
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
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
    /// use structural::enums::VariantProxy;
    ///
    /// #[derive(Debug,PartialEq,Structural)]
    /// #[struc(no_trait)]
    /// enum Foo{
    ///     Bar(u32),
    ///     Baz(u32),
    /// }
    ///
    /// let proxy: VariantProxy<Foo, TS!(Bar)>= unsafe{
    ///     VariantProxy::new(Foo::Bar(0), ts!(Bar))
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
    /// use structural::{fp,ts,TS,StructuralExt,Structural};
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
    /// let proxy: *mut VariantProxy<Foo, TS!(Baz)>= unsafe{
    ///     VariantProxy::from_raw_mut(&mut this as *mut Foo, ts!(Baz))
    /// };
    ///
    /// assert_eq!(unsafe{  &mut *VariantProxy::as_raw_mut(proxy) }, &mut Foo::Baz(0));
    ///
    /// ```
    // WTF is the convention for *mut Self -> *mut T conversions
    #[allow(clippy::wrong_self_convention)]
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

impl<T, V, F> FieldType<F> for VariantProxy<T, V>
where
    T: ?Sized + FieldType<VariantField<V, F>>,
    V: IsTStr,
{
    type Ty = T::Ty;
}

impl<T, V, F> GetField<F> for VariantProxy<T, V>
where
    T: ?Sized + GetVariantField<V, F>,
    V: IsTStr,
{
    #[inline(always)]
    fn get_field_(&self, fname: F) -> &T::Ty {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe { self.value.get_vfield_unchecked(V::DEFAULT, fname) }
    }
}

unsafe impl<T, V, F> GetFieldMut<F> for VariantProxy<T, V>
where
    T: ?Sized + GetVariantFieldMut<V, F>,
    V: IsTStr,
{
    #[inline(always)]
    fn get_field_mut_(&mut self, fname: F) -> &mut T::Ty {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe { self.value.get_vfield_mut_unchecked(V::DEFAULT, fname) }
    }

    #[inline(always)]
    unsafe fn get_field_raw_mut(this: *mut (), fname: F) -> *mut T::Ty
    where
        Self: Sized,
    {
        <Self as SpecGetFieldMut<F>>::get_field_raw_mut_inner(this, fname)
    }

    #[inline(always)]
    fn get_field_raw_mut_fn(&self) -> GetFieldRawMutFn<F, T::Ty> {
        // safety:
        // This transmute should be sound,
        // since the single parameter of `GetFieldMut::get_field_mut_`
        // other than `this: *mut  ()` is a PhantomData.
        (**self).get_vfield_raw_mut_unchecked_fn()
    }
}

unsafe impl<T, V, F> SpecGetFieldMut<F> for VariantProxy<T, V>
where
    T: ?Sized + GetVariantFieldMut<V, F>,
    V: IsTStr,
{
    default_if! {
        #[inline(always)]
        cfg(feature="specialization")
        unsafe fn get_field_raw_mut_inner(
            this: *mut (),
            fname: F,
        ) -> *mut T::Ty
        where
            Self: Sized
        {
            let func=(**(this as *mut Self)).get_vfield_raw_mut_unchecked_fn();
            // safety: VariantProxy<T,V> guarantees that it wraps an enum
            // with the variant that `V` names.
            func(
                this,
                fname,
            )

        }
    }
}
#[cfg(feature = "specialization")]
unsafe impl<T, V, F> SpecGetFieldMut<F> for VariantProxy<T, V>
where
    T: GetVariantFieldMut<V, F>,
    V: IsTStr,
{
    unsafe fn get_field_raw_mut_inner(this: *mut (), fname: F) -> *mut T::Ty {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        // Because it's a `#[repr(transparent)]` wrapper around `T`,
        // it can pass this to `<T as GetFieldMut<_,_>>::get_field_raw_mut`.
        T::get_vfield_raw_mut_unchecked(this, fname)
    }
}

unsafe impl<T, V, F> IntoField<F> for VariantProxy<T, V>
where
    T: IntoVariantField<V, F>,
    V: IsTStr,
{
    #[inline(always)]
    fn into_field_(self, fname: F) -> T::Ty
    where
        Self: Sized,
    {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        unsafe { self.value.into_vfield_unchecked_(V::DEFAULT, fname) }
    }

    #[inline(always)]
    unsafe fn move_out_field_(&mut self, fname: F, moved: &mut MovedOutFields) -> T::Ty
    where
        Self: Sized,
    {
        // safety: VariantProxy<T,V> guarantees that it wraps an enum
        // with the variant that `V` names.
        self.value
            .move_out_vfield_unchecked_(V::DEFAULT, fname, moved)
    }
}

unsafe impl<T, V> DropFields for VariantProxy<T, V>
where
    T: IsVariant<V> + DropFields,
    V: IsTStr,
{
    fn pre_move(&mut self) {
        let mut this = crate::utils::RunOnDrop::new(
            &mut self.value,
            #[inline(always)]
            |this| {
                // This is necessary because the enum can mutate itself into a different variant,
                // invalidating the safety invariant of this type.
                if !this.is_variant_(V::DEFAULT) {
                    abort!(
                        "\n\n\n\
                        The enum changed the active variant in `<{} as DropFields>::pre_move`\
                        \n\n\n",
                        std_::any::type_name::<Self>(),
                    );
                }
            },
        );

        <T as DropFields>::pre_move(this.reborrow_mut());
    }

    unsafe fn drop_fields(&mut self, moved: MovedOutFields) {
        <T as DropFields>::drop_fields(&mut self.value, moved)
    }
}
