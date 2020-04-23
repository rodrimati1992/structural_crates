/// Provides shared access to an enum variant field.
///
/// The `V` and `F` type parameters are expected to be [TStr](../struct.TStr.html).
///
/// Every instance of "the `F` field"/"the `V` variant" in the docs mean
/// "in the field/variant named by the `F`/`V` type parameter"
///
/// # Safety
///
/// `IsVariant<V>` and `GetVariantField<V, F>` must agree on what variant the enum currently is.
/// If `IsVariant` returns true for a particular `V` variant,
/// then `get_vfield_` must return `Some(_)` for the same variant.
///
/// If overriden, the `*_unchecked` methods must diverge
/// (abort, panic, or call the equivalent of `std::hint::unreachable_unchecked`)
/// if the enum is not currently the `V` variant,
/// and return the same field as the checked equivalents if the enum
/// is currently the `V` variant.
///
/// # Example: Use as bound
///
/// ```
/// use structural::field::GetVariantField;
/// use structural::for_examples::{Variants,WithBar};
/// use structural::{StructuralExt,TS,fp};
///
/// fn example(this: impl GetVariantField<TS!(Bar),TS!(0),Ty= &'static str>){
///     assert_eq!( this.field_(fp!(::Bar.0)), Some(&"why?") );
///
///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
///     // an enum variant.
///     assert_eq!( this.fields(fp!(::Bar=>0,0)), Some((&"why?",&"why?")) );
///
///     assert_eq!( this.cloned_fields(fp!(::Bar=>0,0)), Some(("why?","why?")) );
///
///     assert_eq!( this.is_variant(fp!(Bar)), true );
/// }
///
/// example(Variants::Bar("why?"));
/// example(WithBar::Bar("why?"));
///
/// ```
///
/// # Example: Manual implementation
///
/// While this trait is better derived, it can be implemented manually.
///
/// Note that the derive macro also declares trait aliases for the traits implemented here.
///
/// ```rust
/// use structural::{
///     FieldType, GetVariantField, FP, TS,
///     StructuralExt,fp,structural_alias,
/// };
/// use structural::enums::{IsVariant, VariantCount};
///
/// // The `FooBounds` trait is defined below.
/// fn using_enum(bar: &dyn FooBounds, baz: &dyn FooBounds){
///     assert_eq!( bar.fields(fp!(::Bar=>0,1)), Some((&34, &51)) );
///     assert_eq!( bar.is_variant(fp!(Bar)), true );
///     assert_eq!( bar.is_variant(fp!(Baz)), false );
///
///     assert_eq!( baz.fields(fp!(::Bar=>0,1)), None );
///     assert_eq!( baz.is_variant(fp!(Bar)), false );
///     assert_eq!( baz.is_variant(fp!(Baz)), true );
/// }
///
/// # fn main(){
///
/// using_enum(&Foo::Bar(34,51), &Foo::Baz);
///
/// # }
///
/// enum Foo{
///     Bar(u32,u64),
///     Baz,
/// }
///
/// unsafe impl VariantCount for Foo{
///     type Count=TS!(2);
/// }
///
/// unsafe impl IsVariant<TS!(Bar)> for Foo {
///     fn is_variant_(&self,_:TS!(Bar))->bool{
///         match self {
///             Foo::Bar{..}=>true,
///             _=>false,
///         }
///     }
/// }
///
/// impl FieldType<FP!(::Bar.0)> for Foo{
///     type Ty=u32;
/// }
///
/// unsafe impl GetVariantField<TS!(Bar),TS!(0)> for Foo{
///     fn get_vfield_(&self, _:TS!(Bar), _:TS!(0)) -> Option<&u32>{
///         match self {
///             Foo::Bar(ret,_)=>Some(ret),
///             _=>None,
///         }
///     }
/// }
///
///
/// impl FieldType<FP!(::Bar.1)> for Foo{
///     type Ty=u64;
/// }
///
/// unsafe impl GetVariantField<TS!(Bar),TS!(1)> for Foo{
///     fn get_vfield_(&self, _:TS!(Bar), _:TS!(1)) -> Option<&u64>{
///         match self {
///             Foo::Bar(_,ret)=>Some(ret),
///             _=>None,
///         }
///     }
/// }
///
///
/// unsafe impl IsVariant<TS!(Baz)> for Foo {
///     fn is_variant_(&self,_:TS!(Baz))->bool{
///         match self {
///             Foo::Baz{..}=>true,
///             _=>false,
///         }
///     }
/// }
///
/// structural_alias!{
///     trait FooBounds{
///         ref Bar(u32,u64),
///         ref Baz,
///     }
/// }
///
/// ```
///
pub unsafe trait GetVariantField<V, F>:
    IsVariant<V> + FieldType<VariantField<V, F>>
{
    /// Accesses the `F` field in the `V` variant by reference.
    fn get_vfield_(&self, variant: V, field: F) -> Option<&Self::Ty>;

    /// Accesses the `F` field in the `V` variant by reference,
    /// without checking that the enum is currently the `V` variant.
    ///
    /// # Safety
    ///
    /// The enum must be the `V` variant.
    #[inline(always)]
    unsafe fn get_vfield_unchecked(&self, variant: V, field: F) -> &Self::Ty {
        match self.get_vfield_(variant, field) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }
}

/// Gets the type of a variant field,
/// most useful when the variant and field [`TStr`]s are passed separately.
///
/// The `Variant` and `Field` type parameters are expected to be [`TStr`].
///
/// Example: `GetVariantFieldType<This, TS!(Foo), TS!(0)>`
///
/// If you want to write the type parameter as `FP!(::Foo.bar)`,
/// then you can use [`GetFieldType`] instead.
///
/// [`GetFieldType`]: ./type.GetFieldType.html
///
/// # Example
///
/// ```
/// use structural::{StructuralExt,TS,fp};
/// use structural::field::GetVariantFieldType;
/// use structural::for_examples::Variants;
///
/// let this = Variants::Foo(8,13);
///
/// let value: u32= this.into_field(fp!(::Foo.0)).unwrap();
///
/// let value: GetVariantFieldType<Variants, TS!(Foo), TS!(0)>= value;
///
/// assert_eq!( value, 8_u32 );
///
/// ```
///
/// [`TStr`]: ../struct.TStr.html
pub type GetVariantFieldType<This, Variant, Field> =
    <This as FieldType<VariantField<Variant, Field>>>::Ty;

/// Provides shared and mutable access to an enum variant field.
///
/// The `V` and `F` type parameters are expected to be [TStr](../struct.TStr.html).
///
/// Every instance of "the `F` field"/"the `V` variant" in the docs mean
/// "in the field/variant named by the `F`/`V` type parameter"
///
/// # Safety
///
/// Implementors ought not mutate fields inside their accessor trait impls,
/// or the accessor trait impls of other fields.
///
/// The safety requirements for
/// [`GetFielfMut::get_field_raw_mut`](./trait.GetFieldMut.html#raw_mut_properties)
/// also apply to
/// [`GetVariantFieldMut::get_vfield_raw_mut_`](#tymethod.get_vfield_raw_mut_).
///
/// `IsVariant<V>` and `GetVariantFieldMut<V, F>`
/// must agree on what variant the enum currently is.
/// If `IsVariant` returns true for a particular `V` variant,
/// then `get_vfield_mut_` and `get_vfield_raw_mut_`
/// must return `Some(_)` for the same variant.
///
/// If overriden, the `*_unchecked` methods must diverge
/// (abort, panic, or call the equivalent of `std::hint::unreachable_unchecked`)
/// if the enum is not currently the `V` variant,
/// and return the same field as the checked equivalents if the enum
/// is currently the `V` variant.
///
/// # Example: Use as bound.
///
/// ```
/// use structural::field::GetVariantFieldMut;
/// use structural::for_examples::{Bomb,WithBoom};
/// use structural::{StructuralExt,TS,fp};
///
/// fn example<T>(this: &mut T)
/// where
///     T: GetVariantFieldMut<TS!(Boom),TS!(a),Ty= &'static str>+
///        GetVariantFieldMut<TS!(Boom),TS!(b),Ty= &'static [u16]>,
/// {
///     assert_eq!( this.field_(fp!(::Boom.a)), Some(&"why?") );
///     assert_eq!( this.field_mut(fp!(::Boom.a)), Some(&mut "why?") );
///
///     // You can use `fp!(::Foo=>bar,baz)` to access multiple fields inside
///     // an enum variant.
///     assert_eq!( this.fields(fp!(::Boom=>a,b)), Some(( &"why?", &&[0,1,2][..] )) );
///
///     assert_eq!( this.cloned_fields(fp!(::Boom=>a,b)), Some(( "why?", &[0,1,2][..] )) );
///
///     assert_eq!(
///         this.fields_mut(fp!(::Boom=>a,b)),
///         Some(( &mut "why?", &mut &[0,1,2][..] )),
///     );
///
///     assert_eq!( this.is_variant(fp!(Boom)), true );
/// }
///
/// example(&mut WithBoom::Boom{ a:"why?", b:&[0,1,2] });
/// example(&mut Bomb::Boom{ a:"why?", b:&[0,1,2] });
///
/// ```
///
/// <span id="manual-impl-example"></span>
/// # Example: Manual implementation
///
/// While this trait is better derived, it can be implemented manually.
///
/// Note that the derive macro also declares trait aliases for the traits implemented here.
///
/// ```rust
/// use structural::{
///     FieldType, GetVariantField, GetVariantFieldMut, FP, TS,
///     StructuralExt,fp,structural_alias,
/// };
/// use structural::enums::{IsVariant, VariantCount};
///
/// // The `FooBounds` trait is defined below.
/// fn using_enum(bar: &mut dyn FooBounds, baz: &mut dyn FooBounds){
///     assert_eq!( bar.fields(fp!(::Bar=>0,1)), Some((&34, &51)) );
///     assert_eq!( bar.fields_mut(fp!(::Bar=>0,1)), Some((&mut 34, &mut 51)) );
///     assert_eq!( bar.is_variant(fp!(Bar)), true );
///     assert_eq!( bar.is_variant(fp!(Baz)), false );
///
///     assert_eq!( baz.fields(fp!(::Bar=>0,1)), None );
///     assert_eq!( baz.fields_mut(fp!(::Bar=>0,1)), None );
///     assert_eq!( baz.is_variant(fp!(Bar)), false );
///     assert_eq!( baz.is_variant(fp!(Baz)), true );
/// }
///
/// # fn main(){
///
/// using_enum(&mut Foo::Bar(34,51), &mut Foo::Baz);
///
/// # }
///
/// enum Foo{
///     Bar(u32,u64),
///     Baz,
/// }
///
/// unsafe impl VariantCount for Foo{
///     type Count=TS!(2);
/// }
///
/// unsafe impl IsVariant<TS!(Bar)> for Foo {
///     fn is_variant_(&self,_:TS!(Bar))->bool{
///         match self {
///             Foo::Bar{..}=>true,
///             _=>false,
///         }
///     }
/// }
///
/// impl FieldType<FP!(::Bar.0)> for Foo{
///     type Ty=u32;
/// }
///
/// unsafe impl GetVariantField<TS!(Bar),TS!(0)> for Foo{
///     fn get_vfield_(&self, _:TS!(Bar), _:TS!(0)) -> Option<&u32>{
///         match self {
///             Foo::Bar(ret,_)=>Some(ret),
///             _=>None,
///         }
///     }
/// }
///
/// unsafe impl GetVariantFieldMut<TS!(Bar),TS!(0)> for Foo {
///     fn get_vfield_mut_(&mut self, _:TS!(Bar), _:TS!(0))->Option<&mut u32>{
///         match self {
///             Foo::Bar(ret,_)=>Some(ret),
///             _=>None
///         }
///     }
///
///     unsafe fn get_vfield_raw_mut_(
///         this:*mut  (),
///         _:TS!(Bar),
///         _:TS!(0),
///     )->Option<std::ptr::NonNull<u32>> {
///         structural::z_raw_borrow_enum_field!(this as *mut  Self, Foo::Bar.0 : u32)
///     }
///
///     structural::z_unsafe_impl_get_vfield_raw_mut_fn!{
///         Self= Self,
///         variant_tstr= TS!(Bar),
///         field_tstr= TS!(0),
///         field_type= u32,
///     }
/// }
///
///
/// impl FieldType<FP!(::Bar.1)> for Foo{
///     type Ty=u64;
/// }
///
/// unsafe impl GetVariantField<TS!(Bar),TS!(1)> for Foo{
///     fn get_vfield_(&self, _:TS!(Bar), _:TS!(1)) -> Option<&u64>{
///         match self {
///             Foo::Bar(_,ret)=>Some(ret),
///             _=>None,
///         }
///     }
/// }
///
/// unsafe impl GetVariantFieldMut<TS!(Bar),TS!(1)> for Foo {
///     fn get_vfield_mut_(&mut self, _:TS!(Bar), _:TS!(1))->Option<&mut u64>{
///         match self {
///             Foo::Bar(_,ret)=>Some(ret),
///             _=>None
///         }
///     }
///
///     unsafe fn get_vfield_raw_mut_(
///         this:*mut  (),
///         _:TS!(Bar),
///         _:TS!(1),
///     )->Option<std::ptr::NonNull<u64>> {
///         structural::z_raw_borrow_enum_field!(this as *mut  Self, Foo::Bar.1 : u64)
///     }
///
///     structural::z_unsafe_impl_get_vfield_raw_mut_fn!{
///         Self= Self,
///         variant_tstr= TS!(Bar),
///         field_tstr= TS!(1),
///         field_type= u64,
///     }
/// }
///
///
/// unsafe impl IsVariant<TS!(Baz)> for Foo {
///     fn is_variant_(&self,_:TS!(Baz))->bool{
///         match self {
///             Foo::Baz{..}=>true,
///             _=>false,
///         }
///     }
/// }
///
/// structural_alias!{
///     trait FooBounds{
///         mut Bar(u32,u64),
///         mut Baz,
///     }
/// }
///
/// ```
///
///
pub unsafe trait GetVariantFieldMut<V, F>: GetVariantField<V, F> {
    /// Accesses the `F` field in the `V` variant by mutable reference.
    fn get_vfield_mut_(&mut self, variant: V, field: F) -> Option<&mut Self::Ty>;

    /// Accesses the `F` field in the `V` variant by mutable reference,
    /// without checking that the enum is currently the `V` variant.
    ///
    /// # Safety
    ///
    /// The enum must be the `V` variant.
    #[inline(always)]
    unsafe fn get_vfield_mut_unchecked(&mut self, variant: V, field: F) -> &mut Self::Ty {
        match self.get_vfield_mut_(variant, field) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }

    /// Accesses the `F` field in the `V` variant by raw pointer.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut  Self` to `*mut  ()`,
    /// pointing to a fully initialized instance of the type.
    ///
    /// This function returns a `NonNull` purely as an optimization detail,
    /// functions that return raw pointers (`*mut _`) are also
    /// expected to return pointers to valid fields.
    ///
    unsafe fn get_vfield_raw_mut_(ptr: *mut (), variant: V, field: F) -> Option<NonNull<Self::Ty>>
    where
        Self: Sized;

    /// Accesses the `F` field in the `V` variant by raw pointer,
    /// without checking that the enum is currently the `V` variant.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut  Self` to `*mut  ()`,
    /// pointing to a fully initialized instance of the type.
    ///
    /// The enum must also be the `V` variant (type parameter of this trait).
    ///
    // This function takes only the `F` parameter so that its parameters are
    // the same as `GetFieldMut::get_field_raw_mut`.
    #[inline(always)]
    unsafe fn get_vfield_raw_mut_unchecked(ptr: *mut (), field: F) -> *mut Self::Ty
    where
        Self: Sized,
        V: ConstDefault,
    {
        match Self::get_vfield_raw_mut_(ptr, V::DEFAULT, field) {
            Some(x) => x.as_ptr(),
            None => crate::utils::unreachable_unchecked(),
        }
    }

    /// Gets a function pointer to the `get_vfield_raw_mut_` method.
    ///
    /// This exists so that the method can be called in `dyn Trait`s
    fn get_vfield_raw_mut_fn(&self) -> GetVFieldRawMutFn<V, F, Self::Ty>;

    /// Gets a function pointer to the `get_vfield_raw_mut_unchecked` method.
    ///
    /// This exists so that the method can be called in `dyn Trait`s
    fn get_vfield_raw_mut_unchecked_fn(&self) -> GetFieldRawMutFn<F, Self::Ty>;
}

/// The function pointer type for the `GetVariantFieldMut::get_vfield_raw_mut_` method.
pub type GetVFieldRawMutFn<VariantName, FieldName, FieldTy> =
    unsafe fn(*mut (), VariantName, FieldName) -> Option<NonNull<FieldTy>>;

/// Provides shared and by-value access to an enum variant field.
///
/// The `V` and `F` type parameters are expected to be [TStr](../struct.TStr.html).
///
/// Every instance of "the `F` field"/"the `V` variant" in the docs mean
/// "in the field/variant named by the `F`/`V` type parameter"
///
/// # Safety
///
/// `IsVariant<V>` and `IntoVariantField<V, F>` must agree on what variant
/// the enum currently is.
/// If `IsVariant` returns true for a particular `V` variant,
/// then `into_vfield_`,and `box_into_vfield_` must return `Some(_)`.
///
/// If overriden, the `*_unchecked` methods must diverge
/// (abort, panic, or call the equivalent of `std::hint::unreachable_unchecked`)
/// if the enum is not currently the `V` variant,
/// and return the same field as the checked equivalents if the enum
/// is currently the `V` variant.
///
/// ### Implementing `move_out_vfield_`
///
/// The way this method is expected to be implemented like this:
///
/// - Match on the enum,if it's the expected variant continue the steps,
/// otherwise return `None`.
///
/// - Move out the field using `std::ptr::read` or equivalent.
///
/// - Mark the field in the `moved_fields` parameter as being moved out using the
/// `set_moved_out` method,
/// with a `FieldBit` argument unique to this field in the variant
/// (fields from different variants can use the same `FieldBit` as fields in other variants).
///
/// Every implementation of `IntoVariantField::move_out_vfield_`
/// must return field(s) that no other implementation of 
/// `IntoVariantField` or `IntoField` for this type return.
///
/// The `DropFields::drop_fields` implementation for this type must then
/// call `is_moved_out` on its `MovedOutFields` parameter
/// to decide whether to drop the field, 
/// passing the same `FieldBit` argument as in the `move_out_vfield_` implementation.
/// If `is_moved_out` returns false, then the field must be dropped.
///
/// # Example
///
/// ```
/// use structural::field::IntoVariantField;
/// use structural::for_examples::{Bomb,WithBoom};
/// use structural::{StructuralExt,TS,fp};
///
/// fn example<T>(mut this: T)
/// where
///     T: IntoVariantField<TS!(Boom),TS!(a),Ty= &'static str>+
///        IntoVariantField<TS!(Boom),TS!(b),Ty= &'static [u16]>,
/// {
///     assert_eq!( this.is_variant(fp!(Boom)), true );
///
///     assert_eq!( this.field_(fp!(::Boom.a)), Some(&"Because.") );
///
///     assert_eq!( this.cloned_fields(fp!(::Boom=>a,b)), Some(( "Because.", &[13,21][..] )) );
///
///     assert_eq!( this.fields(fp!(::Boom=>a,b)), Some(( &"Because.", &&[13,21][..] )) );
///
///     assert_eq!( this.into_field(fp!(::Boom.a)), Some("Because.") );
/// }
///
/// example(WithBoom::Boom{ a:"Because.", b:&[13,21] });
/// example(Bomb::Boom{ a:"Because.", b:&[13,21] });
///
/// ```
///
/// <span id="manual-impl-example"></span>
/// <span id="manual-implementation-example"></span>
/// # Example: Manual implementation
///
/// While this trait is better derived, it can be implemented manually.
///
/// Note that the derive macro also declares trait aliases for the traits implemented here.
///
/// ```rust
/// use structural::{
///     field::ownership::{FieldBit, DropFields, MovedOutFields, RunDrop},
///     FieldType, GetVariantField, IntoVariantField, FP, TS,
///     StructuralExt,fp,structural_alias,
/// };
/// use structural::enums::{IsVariant, VariantCount};
///
/// // The `FooBounds` trait is defined below.
/// fn using_enum(bar: impl FooBounds, baz: impl FooBounds){
///     assert_eq!( bar.fields(fp!(::Bar=>0,1)), Some((&34, &51)) );
///     assert_eq!( bar.into_fields(fp!(::Bar=>0,1)), Some((34, 51)) );
///     assert_eq!( bar.is_variant(fp!(Bar)), true );
///     assert_eq!( bar.is_variant(fp!(Baz)), false );
///
///     assert_eq!( baz.fields(fp!(::Bar=>0,1)), None );
///     assert_eq!( baz.into_fields(fp!(::Bar=>0,1)), None );
///     assert_eq!( baz.is_variant(fp!(Bar)), false );
///     assert_eq!( baz.is_variant(fp!(Baz)), true );
/// }
///
/// # fn main(){
///
/// using_enum(Foo::Bar(34,51), Foo::Baz);
///
/// # }
///
/// #[derive(Copy,Clone)]
/// enum Foo{
///     Bar(u32,u64),
///     Baz,
/// }
///
/// const BAR_0_INDEX: FieldBit = FieldBit::new(0);
/// const BAR_1_INDEX: FieldBit = FieldBit::new(1);
///
/// unsafe impl VariantCount for Foo{
///     type Count=TS!(2);
/// }
///
/// unsafe impl IsVariant<TS!(Bar)> for Foo {
///     fn is_variant_(&self,_:TS!(Bar))->bool{
///         match self {
///             Foo::Bar{..}=>true,
///             _=>false,
///         }
///     }
/// }
///
/// impl FieldType<FP!(::Bar.0)> for Foo{
///     type Ty=u32;
/// }
///
/// unsafe impl GetVariantField<TS!(Bar),TS!(0)> for Foo{
///     fn get_vfield_(&self, _:TS!(Bar), _:TS!(0)) -> Option<&u32>{
///         match self {
///             Foo::Bar(ret,_)=>Some(ret),
///             _=>None,
///         }
///     }
/// }
///
/// unsafe impl IntoVariantField<TS!(Bar),TS!(0)> for Foo{
///     fn into_vfield_(self, _:TS!(Bar), _:TS!(0)) -> Option<u32>{
///         match self {
///             Foo::Bar(ret,_)=>Some(ret),
///             _=>None,
///         }
///     }
///
///     unsafe fn move_out_vfield_(
///         &mut self,
///         _: TS!(Bar),
///         _: TS!(0),
///         moved_fields: &mut MovedOutFields,
///     ) -> Option<u32> {
///         match self {
///             Foo::Bar(ret,_)=>{
///                 moved_fields.set_moved_out(BAR_0_INDEX);
///                 Some(std::ptr::read(ret))
///             },
///             _=>None,
///         }
///     }
/// }
///
///
/// impl FieldType<FP!(::Bar.1)> for Foo{
///     type Ty=u64;
/// }
///
/// unsafe impl GetVariantField<TS!(Bar),TS!(1)> for Foo{
///     fn get_vfield_(&self, _:TS!(Bar), _:TS!(1)) -> Option<&u64>{
///         match self {
///             Foo::Bar(_,ret)=>Some(ret),
///             _=>None,
///         }
///     }
/// }
///
/// unsafe impl IntoVariantField<TS!(Bar),TS!(1)> for Foo{
///     fn into_vfield_(self, _:TS!(Bar), _:TS!(1)) -> Option<u64>{
///         match self {
///             Foo::Bar(_,ret)=>Some(ret),
///             _=>None,
///         }
///     }
///
///     unsafe fn move_out_vfield_(
///         &mut self,
///         _: TS!(Bar),
///         _: TS!(1),
///         moved_fields: &mut MovedOutFields,
///     ) -> Option<u64> {
///         match self {
///             Foo::Bar(_,ret)=>{
///                 moved_fields.set_moved_out(BAR_1_INDEX);
///                 Some(std::ptr::read(ret))
///             },
///             _=>None,
///         }
///     }
/// }
///
/// unsafe impl DropFields for Foo{
///     unsafe fn drop_fields(&mut self, moved_fields: MovedOutFields){
///         match self {
///             Foo::Bar(field0, field1)=>{
///                 // RunDrop here ensures that the destructors for all fields are ran 
///                 // even if any of them panics.
///                 let _a;
///                 if moved_fields.is_moved_out(BAR_0_INDEX){
///                     _a = RunDrop::new(field0);
///                 }
///
///                 let _a;
///                 if moved_fields.is_moved_out(BAR_1_INDEX){
///                     _a = RunDrop::new(field1);
///                 }
///             }
///             Foo::Baz=>{}
///         }
///     }
/// }
///
/// unsafe impl IsVariant<TS!(Baz)> for Foo {
///     fn is_variant_(&self,_:TS!(Baz))->bool{
///         match self {
///             Foo::Baz{..}=>true,
///             _=>false,
///         }
///     }
/// }
///
/// structural_alias!{
///     trait FooBounds: Copy{
///         move Bar(u32,u64),
///         move Baz,
///     }
/// }
///
/// ```
///
///
pub unsafe trait IntoVariantField<V, F>: GetVariantField<V, F> + DropFields {
    /// Converts this into the `F` field in the `V` variant by value.
    fn into_vfield_(self, variant_name: V, field_name: F)->Option<Self::Ty>;
    
    /// Converts this into the `F` field in the `V` variant by value.
    ///
    /// # Safety
    ///
    /// The enum must be the `V` variant.
    #[inline(always)]
    unsafe fn into_vfield_unchecked_(self, variant_name: V, field_name: F)->Self::Ty
    where
        Self: Sized
    {
        match self.into_vfield_(variant_name, field_name) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }

    /// Moves out the `F` field from the `V` variant.
    ///
    /// # Safety
    ///
    /// The same instance of `MovedOutFields` must be passed to every call to
    /// `move_out_vfield_` on the same instance of this type,
    /// as well as not mutating that `MovedOutFields` instance outside of 
    /// methods of this trait for this type.
    /// 
    /// Each field must be moved with any method at most once on the same instance 
    /// of this type.
    /// 
    unsafe fn move_out_vfield_(
        &mut self,
        variant_name: V,
        field_name: F,
        moved_fields: &mut MovedOutFields,
    ) -> Option<Self::Ty>;

    /// Converts this into the `F` field in the `V` variant by value,
    /// without checking that the enum is currently the `V` variant.
    ///
    /// This method exists so that `Box<dyn Trait>` can be converted into a field by value.
    ///
    /// # Safety
    ///
    /// The enum must be the `V` variant.
    ///
    /// The same instance of `MovedOutFields` must be passed to every call to
    /// `move_out_vfield_unchecked_` on the same instance of this type,
    /// as well as not mutating that `MovedOutFields` instance outside of 
    /// methods of this trait for this type.
    /// 
    /// Each field must be moved with any method at most once on the same instance 
    /// of this type.
    /// 
    #[inline(always)]
    unsafe fn move_out_vfield_unchecked_(
        &mut self,
        variant_name: V,
        field_name: F,
        moved_fields: &mut MovedOutFields,
    ) -> Self::Ty {
        match self.move_out_vfield_(variant_name, field_name, moved_fields) {
            Some(x) => x,
            None => crate::utils::unreachable_unchecked(),
        }
    }
}

///////////////////////////

/// A bound for shared,mutable,and by-value access to the field `F` inside of the `V` variant.
///
/// The `V` and `F` type parameters are expected to be [TStr](../struct.TStr.html).
///
/// Every instance of "the `F` field"/"the `V` variant" in the docs mean
/// "in the field/variant named by the `F`/`V` type parameter"
///
/// # Example
///
/// ```
/// use structural::field::IntoVariantFieldMut;
/// use structural::for_examples::{Bomb,WithBoom};
/// use structural::{StructuralExt,TS,fp};
///
/// fn example<T>(mut this: T)
/// where
///     T: IntoVariantFieldMut<TS!(Boom),TS!(a),Ty= &'static str>+
///        IntoVariantFieldMut<TS!(Boom),TS!(b),Ty= &'static [u16]>,
/// {
///     assert_eq!( this.is_variant(fp!(Boom)), true );
///
///     assert_eq!( this.field_(fp!(::Boom.a)), Some(&"Because.") );
///
///     assert_eq!( this.cloned_fields(fp!(::Boom=>a,b)), Some(( "Because.", &[13,21][..] )) );
///
///     assert_eq!( this.fields(fp!(::Boom=>a,b)), Some(( &"Because.", &&[13,21][..] )) );
///
///     assert_eq!( this.field_mut(fp!(::Boom.a)), Some(&mut "Because.") );
///
///     assert_eq!( this.fields_mut(
///         fp!(::Boom=>a,b)),
///         Some(( &mut "Because.", &mut &[13,21][..] )),
///     );
///
///     assert_eq!( this.into_fields(fp!(::Boom=>a,b)), Some(( "Because.", &[13,21][..] )) );
/// }
///
/// example(WithBoom::Boom{ a:"Because.", b:&[13,21] });
/// example(Bomb::Boom{ a:"Because.", b:&[13,21] });
///
/// ```
pub trait IntoVariantFieldMut<V, F>: GetVariantFieldMut<V, F> + IntoVariantField<V, F> {}

impl<This, V, F> IntoVariantFieldMut<V, F> for This where
    This: ?Sized + GetVariantFieldMut<V, F> + IntoVariantField<V, F>
{
}

///////////////////////////

/// A `GetVariantFieldMut` specifically used for specialization internally.
///
/// By specializing a separate trait,the compiler shows the same error message
/// with specialization and without.
///
/// Every instance of "the `F` field"/"the `V` variant" in the docs mean
/// "in the field/variant named by the `F`/`V` type parameter"
///
/// # Safety
///
/// This trait has the same safety requirements as `GetVariantFieldMut`.
#[doc(hidden)]
pub unsafe trait SpecGetVariantFieldMut<V, F>: GetVariantField<V, F> {
    /// Accesses the `F` field in the `V` variant by raw pointer.
    ///
    /// # Safety
    ///
    /// You must pass a pointer casted from `*mut  Self` to `*mut  ()`,
    /// pointing to a fully initialized instance of the type.
    unsafe fn get_vfield_raw_mut_inner(
        ptr: *mut (),
        variant_name: V,
        field_name: F,
    ) -> Option<NonNull<Self::Ty>>
    where
        Self: Sized;
}
