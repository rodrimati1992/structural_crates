#[cfg(feature = "alloc")]
use crate::alloc::boxed::Box;

use crate::{
    enums::{IsVariant, VariantProxy},
    type_level::FieldPath1,
};

/// Extension trait for enums.
///
/// This trait has these methods:
///
/// - `*_variant`: For fallibly converting an enum to a VariantProxy of a passed variant.
/// As opposed to calling GetFieldExt methods with a `fp!(::Foo)` argument,
/// this allows recovering the enum when it's not the passed variant.
///
pub trait EnumExt {
    /// Fallibly converts a reference to an enum into
    /// a reference of a VariantProxy of some variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt};
    /// use structural::enums::example_enums::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// let this=Variants::Foo(11,22);
    /// {
    ///     // `FP!(F o o)` can also be written as `FP!(Foo)` since Rust 1.40.0
    ///     let proxy: &VariantProxy<Variants,FP!(F o o)>=
    ///         this.as_variant(fp!(Foo)).unwrap();
    ///
    ///     assert_eq!( proxy.field_(fp!(0)), &11);
    ///     assert_eq!( proxy.field_(fp!(1)), &22);
    /// }
    /// {
    ///     assert_eq!( this.as_variant(fp!(Bar)), Err(&this) );
    /// }
    /// ```
    #[inline(always)]
    fn as_variant<V>(
        &self,
        vari: FieldPath1<V>,
    ) -> Result<&VariantProxy<Self, FieldPath1<V>>, &Self>
    where
        Self: IsVariant<FieldPath1<V>>,
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
    /// use structural::{fp,FP,GetFieldExt};
    /// use structural::enums::example_enums::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// let mut this=Variants::Bar("hello");
    /// let mut other=this.clone();
    ///
    /// {
    ///     // `FP!(B a r)` can also be written as `FP!(Bar)` since Rust 1.40.0
    ///     let proxy: &mut VariantProxy<Variants,FP!(B a r)>=
    ///         this.as_mut_variant(fp!(Bar)).unwrap();
    ///    
    ///     assert_eq!( proxy.field_(fp!(0)), &"hello");
    ///     assert_eq!( proxy.field_mut(fp!(0)), &mut"hello");
    /// }
    /// {
    ///     assert_eq!( this.as_mut_variant(fp!(Foo)), Err(&mut other) );
    ///     assert_eq!( this.as_mut_variant(fp!(Baz)), Err(&mut other) );
    ///     assert_eq!( this.as_mut_variant(fp!(Boom)), Err(&mut other) );
    /// }
    /// ```
    #[inline(always)]
    fn as_mut_variant<V>(
        &mut self,
        vari: FieldPath1<V>,
    ) -> Result<&mut VariantProxy<Self, FieldPath1<V>>, &mut Self>
    where
        Self: IsVariant<FieldPath1<V>>,
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
    /// use structural::{fp,FP,GetFieldExt};
    /// use structural::enums::example_enums::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// let mut this=Variants::Baz(None);
    ///
    /// unsafe{
    ///     // `FP!(B a z)` can also be written as `FP!(Baz)` since Rust 1.40.0
    ///     let proxy: *mut VariantProxy<Variants,FP!(B a z)>=
    ///         Variants::as_raw_mut_variant(&mut this,fp!(Baz)).unwrap();
    ///    
    ///     assert_eq!( (*proxy).field_(fp!(0)), None);;
    ///     assert_eq!( (*proxy).field_mut(fp!(0)), None);;
    /// }
    /// unsafe{
    ///     assert_eq!(
    ///         Variants::as_raw_mut_variant(&mut this,fp!(Foo)),
    ///         Err(&mut this as *mut Variants)
    ///     );
    ///     assert_eq!(
    ///         Variants::as_raw_mut_variant(&mut this,fp!(Bar)),
    ///         Err(&mut this as *mut Variants)
    ///     );
    ///     assert_eq!(
    ///         Variants::as_raw_mut_variant(&mut this,fp!(Boom)),
    ///         Err(&mut this as *mut Variants)
    ///     );
    /// }
    ///
    /// ```
    #[inline(always)]
    unsafe fn as_raw_mut_variant<V>(
        this: *mut Self,
        vari: FieldPath1<V>,
    ) -> Result<*mut VariantProxy<Self, FieldPath1<V>>, *mut Self>
    where
        Self: IsVariant<FieldPath1<V>>,
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
    /// use structural::{fp,FP,GetFieldExt};
    /// use structural::enums::example_enums::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// use std::cmp::Ordering;
    ///
    /// let this=Variants::Baz(Some(Ordering::Less));
    ///
    /// {
    ///     // `FP!(B a z)` can also be written as `FP!(Baz)` since Rust 1.40.0
    ///     let mut proxy: VariantProxy<Variants,FP!(B a z)>=
    ///         this.into_variant(fp!(Baz)).unwrap();
    ///    
    ///     assert_eq!( proxy.field_(fp!(0)), Some(&Ordering::Less));
    ///     assert_eq!( proxy.field_mut(fp!(0)), Some(&mut Ordering::Less));
    ///     assert_eq!( proxy.into_field(fp!(0)), Some(Ordering::Less));
    /// }
    /// {
    ///     assert_eq!(this.into_variant(fp!(Foo)), Err(this));
    ///     assert_eq!(this.into_variant(fp!(Bar)), Err(this));
    ///     assert_eq!(this.into_variant(fp!(Boom)), Err(this));
    /// }
    ///
    /// ```
    #[inline(always)]
    fn into_variant<V>(self, vari: FieldPath1<V>) -> Result<VariantProxy<Self, FieldPath1<V>>, Self>
    where
        Self: IsVariant<FieldPath1<V>> + Sized,
    {
        if IsVariant::is_variant_(&self, vari) {
            unsafe { Ok(VariantProxy::new(self, vari)) }
        } else {
            Err(self)
        }
    }

    /// Fallibly converts a boxed enum into a VariantProxy of some variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{fp,FP,GetFieldExt};
    /// use structural::enums::example_enums::Variants;
    /// use structural::enums::{EnumExt,VariantProxy};
    ///
    /// use std::cmp::Ordering;
    ///
    /// let this=Box::new(Variants::Boom{
    ///     a: None,
    ///     b: &[3,5,8,13],
    /// });
    ///
    /// {
    ///     // `FP!(B o o m)` can also be written as `FP!(Boom)` since Rust 1.40.0
    ///     let mut proxy: VariantProxy<Box<Variants>,FP!(B o o m)>=
    ///         this.clone().box_into_variant(fp!(Boom)).unwrap();
    ///    
    ///     assert_eq!( proxy.field_(fp!(a)), &None);
    ///     assert_eq!( proxy.field_mut(fp!(a)), &mut None);
    ///     assert_eq!( proxy.clone().into_field(fp!(a)), None);
    ///    
    ///     assert_eq!( proxy.field_(fp!(b)), &&[3,5,8,13]);
    ///     assert_eq!( proxy.field_mut(fp!(b)), &mut [3,5,8,13]);
    ///     assert_eq!( proxy.clone().into_field(fp!(b)), [3,5,8,13]);
    /// }
    /// {
    ///     assert_eq!(this.clone().box_into_variant(fp!(Foo)), Err(this.clone()));
    ///     assert_eq!(this.clone().box_into_variant(fp!(Bar)), Err(this.clone()));
    ///     assert_eq!(this.clone().box_into_variant(fp!(Baz)), Err(this.clone()));
    /// }
    ///
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn box_into_variant<V>(
        self: Box<Self>,
        vari: FieldPath1<V>,
    ) -> Result<VariantProxy<Box<Self>, FieldPath1<V>>, Box<Self>>
    where
        Self: IsVariant<FieldPath1<V>>,
    {
        if IsVariant::is_variant_(&*self, vari) {
            unsafe { Ok(VariantProxy::from_box(self, vari)) }
        } else {
            Err(self)
        }
    }
}

impl<This: ?Sized> EnumExt for This {}
