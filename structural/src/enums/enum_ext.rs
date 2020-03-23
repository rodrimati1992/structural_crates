use crate::{
    enums::{IsVariant, VariantProxy},
    path::TStr,
};

/// Extension trait for enums.
///
/// This trait has these methods:
///
/// - `*_variant`: For fallibly converting an enum to a VariantProxy of a passed variant.
/// As opposed to calling StructuralExt methods with a `fp!(::Foo)` argument,
/// this allows recovering the enum when it's not the passed variant.
///
pub trait EnumExt {
    /// Fallibly converts a reference to an enum into
    /// a reference of a VariantProxy of some variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{ts,TS,StructuralExt};
    /// use structural::for_examples::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// let this=Variants::Foo(11,22);
    /// {
    ///     let proxy: &VariantProxy<Variants,TS!(Foo)>=
    ///         this.as_variant(ts!(Foo)).unwrap();
    ///
    ///     assert_eq!( proxy.field_(ts!(0)), &11);
    ///     assert_eq!( proxy.field_(ts!(1)), &22);
    /// }
    /// {
    ///     assert_eq!( this.as_variant(ts!(Bar)), Err(&this) );
    /// }
    /// ```
    #[inline(always)]
    fn as_variant<V>(&self, vari: TStr<V>) -> Result<&VariantProxy<Self, TStr<V>>, &Self>
    where
        Self: IsVariant<TStr<V>>,
    {
        if IsVariant::is_variant_(self, vari) {
            unsafe { Ok(VariantProxy::from_ref(self, vari)) }
        } else {
            Err(self)
        }
    }

    /// Fallibly converts a mutable reference to an enum into
    /// a mutable reference of a VariantProxy of some variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{ts,TS,StructuralExt};
    /// use structural::for_examples::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// let mut this=Variants::Bar("hello");
    /// let mut other=this.clone();
    ///
    /// {
    ///     let proxy: &mut VariantProxy<Variants,TS!(Bar)>=
    ///         this.as_mut_variant(ts!(Bar)).unwrap();
    ///    
    ///     assert_eq!( proxy.field_(ts!(0)), &"hello");
    ///     assert_eq!( proxy.field_mut(ts!(0)), &mut"hello");
    /// }
    /// {
    ///     assert_eq!( this.as_mut_variant(ts!(Foo)), Err(&mut other) );
    ///     assert_eq!( this.as_mut_variant(ts!(Baz)), Err(&mut other) );
    ///     assert_eq!( this.as_mut_variant(ts!(Boom)), Err(&mut other) );
    /// }
    /// ```
    #[inline(always)]
    fn as_mut_variant<V>(
        &mut self,
        vari: TStr<V>,
    ) -> Result<&mut VariantProxy<Self, TStr<V>>, &mut Self>
    where
        Self: IsVariant<TStr<V>>,
    {
        if IsVariant::is_variant_(&*self, vari) {
            unsafe { Ok(VariantProxy::from_mut(self, vari)) }
        } else {
            Err(self)
        }
    }

    /// Fallibly converts a raw pointer to an enum into
    /// a raw pointer of a VariantProxy of some variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{ts,TS,StructuralExt};
    /// use structural::for_examples::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// let mut this=Variants::Baz(None);
    ///
    /// unsafe{
    ///     let proxy: *mut VariantProxy<Variants,TS!(Baz)>=
    ///         Variants::as_raw_mut_variant(&mut this,ts!(Baz)).unwrap();
    ///    
    ///     assert_eq!( (*proxy).field_(ts!(0)), &None);;
    ///     assert_eq!( (*proxy).field_mut(ts!(0)), &None);;
    /// }
    /// unsafe{
    ///     assert_eq!(
    ///         Variants::as_raw_mut_variant(&mut this,ts!(Foo)),
    ///         Err(&mut this as *mut Variants)
    ///     );
    ///     assert_eq!(
    ///         Variants::as_raw_mut_variant(&mut this,ts!(Bar)),
    ///         Err(&mut this as *mut Variants)
    ///     );
    ///     assert_eq!(
    ///         Variants::as_raw_mut_variant(&mut this,ts!(Boom)),
    ///         Err(&mut this as *mut Variants)
    ///     );
    /// }
    ///
    /// ```
    ///
    /// # Safety
    ///
    /// You must pass a pointer to a fully initialized instance of `Self`.
    #[inline(always)]
    unsafe fn as_raw_mut_variant<V>(
        this: *mut Self,
        vari: TStr<V>,
    ) -> Result<*mut VariantProxy<Self, TStr<V>>, *mut Self>
    where
        Self: IsVariant<TStr<V>>,
    {
        if IsVariant::is_variant_(&*this, vari) {
            Ok(VariantProxy::from_raw_mut(this, vari))
        } else {
            Err(this)
        }
    }

    /// Fallibly converts an enum into a VariantProxy of some variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{ts,TS,StructuralExt};
    /// use structural::for_examples::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// use std::cmp::Ordering;
    ///
    /// let this=Variants::Baz(Some(Ordering::Less));
    ///
    /// {
    ///     let mut proxy: VariantProxy<Variants,TS!(Baz)>=
    ///         this.into_variant(ts!(Baz)).unwrap();
    ///    
    ///     assert_eq!( proxy.field_(ts!(0)), &Some(Ordering::Less));
    ///     assert_eq!( proxy.field_mut(ts!(0)), &mut Some(Ordering::Less));
    ///     assert_eq!( proxy.into_field(ts!(0)), Some(Ordering::Less));
    /// }
    /// {
    ///     assert_eq!(this.into_variant(ts!(Foo)), Err(this));
    ///     assert_eq!(this.into_variant(ts!(Bar)), Err(this));
    ///     assert_eq!(this.into_variant(ts!(Boom)), Err(this));
    /// }
    ///
    /// ```
    #[inline(always)]
    fn into_variant<V>(self, vari: TStr<V>) -> Result<VariantProxy<Self, TStr<V>>, Self>
    where
        Self: IsVariant<TStr<V>> + Sized,
    {
        if IsVariant::is_variant_(&self, vari) {
            unsafe { Ok(VariantProxy::new(self, vari)) }
        } else {
            Err(self)
        }
    }
}

impl<This: ?Sized> EnumExt for This {}
