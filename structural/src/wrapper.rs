use crate::{
    convert::{IntoStructural, TryFromError, TryIntoStructural},
    enums::IsVariant,
    field::{
        GetField, GetFieldMut, NormalizeFields, NormalizeFieldsOut, RevGetFieldImpl,
        RevGetFieldMutImpl, RevGetMultiField, RevGetMultiFieldMut, RevGetMultiFieldMutOut,
        RevGetMultiFieldOut, RevIntoFieldImpl, RevIntoMultiField, RevIntoMultiFieldOut,
    },
    path::IsTStr,
};

use core_extensions::{
    collection_traits::{Cloned, ClonedOut},
    ConstDefault,
};

use std_::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

/// A wrapper-type alternative to [`StructuralExt`],
/// with methods for accessing fields in structural types.
///
/// # Example: Struct
///
/// ```rust
/// use structural::{StrucWrapper, Structural, fp};
/// use structural::structural_aliases::Array3;
///
/// let mut this=Point{x:3, y:5, z:8};
/// let mut tuple=(13,21,34);
///
/// rotate_tuple(&mut this);
/// rotate_tuple(&mut tuple);
///
/// assert_eq!( this, Point{x:8, y:3, z:5} );
/// assert_eq!( tuple, (34,13,21) );
///
/// fn rotate_tuple(tuple: &mut dyn Array3<u32>){
///     use std::mem::swap;
///
///     let mut tuple=StrucWrapper(tuple);
///     let (a,b,c)=tuple.muts(fp!(0,1,2));
///     swap(b,c);
///     swap(a,b);
/// }
///
/// #[derive(Debug,Structural,PartialEq)]
/// struct Point{
///     #[struc(rename="0")]
///     pub x: u32,
///     #[struc(rename="1")]
///     pub y: u32,
///     #[struc(rename="2")]
///     pub z: u32,
/// }
///
/// ```
///
/// # Example: Enum
///
/// ```rust
/// use structural::{StrucWrapper, Structural, fp};
///
/// assert_eq!( get_value(States::Initial), 1 );
/// assert_eq!( get_value(States::Open{how_much: 10}), 160+2 );
/// assert_eq!( get_value(States::Closed), 3 );
///
/// assert_eq!( get_value(UberStates::Initial), 1 );
/// assert_eq!( get_value(UberStates::Open{how_much: 10, throughput: 14}), 160+2 );
/// assert_eq!( get_value(UberStates::Open{how_much: 20, throughput: 55}), 320+2 );
/// assert_eq!( get_value(UberStates::Closed), 3 );
///
/// // `States_SI` was declared by the `Structural` derive macro on
/// // the `States` enum,aliasing its accessor trait impls.
/// fn get_value(this: impl States_SI)-> u64 {
///     let this=StrucWrapper(this);
///
///     if this.is_variant(fp!(Initial)) {
///         1
///     }else if let Some(how_much)= this.r(fp!(::Open.how_much)) {
///         2 + ((*how_much as u64) << 4)
///     }else if this.is_variant(fp!(Closed)) {
///         3
///     }else{
///         0
///     }
/// }
///
/// // This function is equivalent to `get_value`
/// //
/// // `States_SI` was declared by the `Structural` derive macro on
/// // the `States` enum,aliasing its accessor trait impls.
/// fn get_value_switch(this: impl States_SI)-> u64 {
///     structural::switch!{ref this;
///         Initial=>1,
///         Open{&how_much}=>2 + ((how_much as u64) << 4),
///         Closed=>3,
///         _=>0,
///     }
/// }
///
/// #[derive(Structural)]
/// enum States{
///     Initial,
///     Open{how_much: u32},
///     Closed,
/// }
///
/// #[derive(Structural)]
/// enum UberStates{
///     Initial,
///     Open{
///         how_much: u32,
///         throughput: u64,
///     },
///     Closed,
/// }
///
/// ```
///
/// [`StructuralExt`]: ./trait.StructuralExt.html
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StrucWrapper<T>(pub T);

impl<T> StrucWrapper<T> {
    /// Gets a reference to a single field,determined by `path`.
    ///
    /// This function is equivalent to [`StructuralExt::field_`],
    /// which has more complete examples.
    ///
    /// # Example: Struct
    ///
    /// ```
    /// use structural::{StrucWrapper, fp};
    /// use structural::structural_aliases::Array5;
    ///
    /// assertions((0,0,13,0,34));
    /// assertions((0,0,13,0,34,""));
    /// assertions((0,0,13,0,34,"",false));
    ///
    /// assertions([0,0,13,0,34]);
    /// assertions([0,0,13,0,34,0]);
    /// assertions([0,0,13,0,34,0,0]);
    ///
    /// fn assertions(this: impl Array5<u64>){
    ///     let mut this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.r(fp!(2)), &13  );
    ///     assert_eq!( this.r(fp!(4)), &34 );
    /// }
    ///
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```rust
    /// use structural::{StrucWrapper, fp};
    /// use structural::for_examples::{
    ///     EnumWithNewtype, EnumWithoutNewtype, EnumWithNewtype_SI, RefWrapper,
    /// };
    ///
    /// assertions(EnumWithNewtype::U32(RefWrapper(0x100, &43370)));
    ///
    /// assertions(EnumWithoutNewtype::U32(0x100, &43370));
    ///
    /// // `EnumWithNewtype_SI` was declared by the `Structural` derive macro on
    /// // the `EnumWithNewtype` enum,aliasing its accessor trait impls.
    /// fn assertions<'a>(this: impl EnumWithNewtype_SI<'a>){
    ///     let mut this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.r(fp!(::U32.0)), Some(&0x100) );
    ///     assert_eq!( this.r(fp!(::U32.1)), Some(&&43370) );
    ///     assert_eq!( this.r(fp!(::U64.0)), None );
    /// }
    ///
    /// ```
    ///
    /// [`StructuralExt::field_`]: ./trait.StructuralExt.html#method.field_
    #[inline(always)]
    pub fn r<'a, P>(&'a self, path: P) -> NormalizeFieldsOut<Result<&'a P::Ty, P::Err>>
    where
        P: RevGetFieldImpl<'a, T>,
        Result<&'a P::Ty, P::Err>: NormalizeFields,
    {
        path.rev_get_field(&self.0).normalize_fields()
    }

    /// Gets a mutable reference to a single field,determined by `path`.
    ///
    /// This function is equivalent to [`StructuralExt::field_mut`],
    /// which has more complete examples.
    ///
    /// # Example: Struct
    ///
    /// ```rust
    /// use structural::{StrucWrapper, Structural, fp, make_struct};
    ///
    /// assertions(Puck{name:"John", surname:"Chickenbert"});
    ///
    /// assertions(make_struct!{
    ///     #![derive(Debug,Copy,Clone)]
    ///     name:"John",
    ///     surname:"Chickenbert"
    /// });
    ///
    /// // `Puck_SI` was declared by the `Structural` derive macro on the Puck struct,
    /// // aliasing its accessor trait impls.
    /// fn assertions(this: impl Puck_SI + Copy){
    ///     let mut this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.m(fp!(name)), &mut "John" );
    ///     assert_eq!( this.m(fp!(surname)), &mut "Chickenbert" );
    /// }
    ///
    /// #[derive(Structural,Copy,Clone)]
    /// pub struct Puck{
    ///     pub name: &'static str,
    ///     pub surname: &'static str,
    /// }
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```rust
    /// use structural::{StrucWrapper, fp};
    /// use structural::for_examples::{
    ///     EnumWithNewtype, EnumWithoutNewtype, EnumWithNewtype_SI, RefWrapper,
    /// };
    ///
    /// assertions(EnumWithNewtype::U32(RefWrapper(0x100, &43370)));
    ///
    /// assertions(EnumWithoutNewtype::U32(0x100, &43370));
    ///
    /// // `EnumWithNewtype_SI` was declared by the `Structural` derive macro on
    /// // the `EnumWithNewtype` enum,aliasing its accessor trait impls.
    /// fn assertions<'a>(this: impl EnumWithNewtype_SI<'a>){
    ///     let mut this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.m(fp!(::U32.0)), Some(&mut 0x100) );
    ///     assert_eq!( this.m(fp!(::U32.1)), Some(&mut &43370) );
    ///     assert_eq!( this.m(fp!(::U64.0)), None );
    /// }
    ///
    /// ```
    ///
    /// [`StructuralExt::field_mut`]: ./trait.StructuralExt.html#method.field_mut
    #[inline(always)]
    pub fn m<'a, P>(&'a mut self, path: P) -> NormalizeFieldsOut<Result<&'a mut P::Ty, P::Err>>
    where
        P: RevGetFieldMutImpl<'a, T>,
        Result<&'a mut P::Ty, P::Err>: NormalizeFields,
    {
        path.rev_get_field_mut(&mut self.0).normalize_fields()
    }

    /// Converts this into a single field by value,determined by `path`.
    ///
    /// This function is equivalent to [`StructuralExt::into_field`],
    /// which has more complete examples.
    ///
    /// # Example: Struct
    ///
    /// ```rust
    /// use structural::{StrucWrapper, Structural, fp, make_struct};
    ///
    /// assertions(Puck{name:"John", surname:"Chickenbert"});
    ///
    /// assertions(make_struct!{
    ///     #![derive(Debug,Copy,Clone)]
    ///     name:"John",
    ///     surname:"Chickenbert"
    /// });
    ///
    /// // `Puck_SI` was declared by the `Structural` derive macro on the Puck struct,
    /// // aliasing its accessor trait impls.
    /// fn assertions(this: impl Puck_SI + Copy){
    ///     let this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.v(fp!(name)), "John" );
    ///     assert_eq!( this.v(fp!(surname)), "Chickenbert" );
    /// }
    ///
    /// #[derive(Structural,Copy,Clone)]
    /// pub struct Puck{
    ///     pub name: &'static str,
    ///     pub surname: &'static str,
    /// }
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```rust
    /// use structural::{StrucWrapper, fp};
    /// use structural::for_examples::{WithBoom, WithBoom_SI, Bomb};
    ///
    /// assertions(WithBoom::Boom{a:"#eh#", b:&[5,8,13]});
    /// assertions(    Bomb::Boom{a:"#eh#", b:&[5,8,13]});
    ///
    /// // `WithBoom_SI` was declared by the `Structural` derive macro on the `WithBoom` enum,
    /// // aliasing its accessor trait impls.
    /// fn assertions(this: impl WithBoom_SI + Copy){
    ///     let this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.v(fp!(::Boom.a)), Some("#eh#") );
    ///     assert_eq!( this.v(fp!(::Boom.b)), Some(&[5,8,13][..]) );
    ///     assert!( this.v(fp!(::Nope)).is_none() );
    /// }
    ///
    /// ```
    ///
    /// [`StructuralExt::into_field`]: ./trait.StructuralExt.html#method.into_field
    #[inline(always)]
    pub fn v<P>(self, path: P) -> NormalizeFieldsOut<Result<P::Ty, P::Err>>
    where
        P: RevIntoFieldImpl<T>,
        P::Ty: Sized,
        Result<P::Ty, P::Err>: NormalizeFields,
    {
        path.rev_into_field(self.0).normalize_fields()
    }

    /// Gets references to multiple fields,determined by `path`.
    ///
    /// This function is equivalent to [`StructuralExt::fields`],
    /// which has more complete examples.
    ///
    /// # Example: Struct
    ///
    /// ```
    /// use structural::{StrucWrapper, fp, make_struct};
    /// use structural::for_examples::{Struct2, Struct2_SI, Struct3};
    ///
    /// assertions(Struct2{foo:Some("&"), bar:(true,false)});
    /// assertions(Struct3{foo:Some("&"), bar:(true,false), baz:&[()]});
    /// assertions(make_struct!{foo:Some("&"), bar:(true,false), boom:()});
    ///
    /// // `Struct2_SI` was declared by the `Structural` derive macro on the Struct2 struct,
    /// // aliasing its accessor trait impls.
    /// fn assertions(this: impl Struct2_SI<&'static str, (bool,bool)>){
    ///     let this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.refs(fp!(foo,bar)), (&Some("&"), &(true,false)) );
    /// }
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```
    /// use structural::{StrucWrapper, fp};
    /// use structural::for_examples::{WithBoom, WithBoom_SI, Bomb};
    ///
    /// assertions(WithBoom::Boom{a:"#eh#", b:&[5,8,13]});
    /// assertions(    Bomb::Boom{a:"#eh#", b:&[5,8,13]});
    ///
    /// // `WithBoom_SI` was declared by the `Structural` derive macro on the `WithBoom` enum,
    /// // aliasing its accessor trait impls.
    /// fn assertions(this: impl WithBoom_SI){
    ///     let this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.refs(fp!(::Boom=>a,b)), Some((&"#eh#", &&[5,8,13][..])) );
    ///     assert!( this.refs(fp!(::Nope=>)).is_none() );
    /// }
    ///
    /// ```
    ///
    /// [`StructuralExt::fields`]: ./trait.StructuralExt.html#method.fields
    #[inline(always)]
    pub fn refs<'a, P>(&'a self, path: P) -> RevGetMultiFieldOut<'a, P, T>
    where
        P: RevGetMultiField<'a, T>,
    {
        path.rev_get_multi_field(&self.0)
    }

    /// Gets clones of multiple fields,determined by `path`.
    ///
    /// This function is equivalent to [`StructuralExt::cloned_fields`],
    /// which has more complete examples.
    ///
    /// # Example: Struct
    ///
    /// ```
    /// use structural::{StrucWrapper, fp, make_struct};
    /// use structural::for_examples::{Struct2, Struct2_SI, Struct3};
    ///
    /// assertions(Struct2{foo:Some("&"), bar:(true,false)});
    /// assertions(Struct3{foo:Some("&"), bar:(true,false), baz:&[()]});
    /// assertions(make_struct!{foo:Some("&"), bar:(true,false), boom:()});
    ///
    /// fn assertions(this: impl Struct2_SI<&'static str, (bool,bool)>){
    ///     let this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.clones(fp!(foo,bar)), (Some("&"), (true,false)) );
    /// }
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```
    /// use structural::{StrucWrapper, fp};
    /// use structural::for_examples::{Enum2, Enum2_SI, Enum3, Enum4};
    ///
    /// use std::cmp::Ordering;
    ///
    /// assertions(Enum2::Bar(Ordering::Less, Some(1337)));
    /// assertions(Enum3::Bar(Ordering::Less, Some(1337)));
    /// assertions(Enum4::Bar(Ordering::Less, Some(1337)));
    ///
    /// fn assertions(this: impl Enum2_SI){
    ///     let this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.clones(fp!(::Bar=>0,1)), Some((Ordering::Less, Some(1337))) );
    ///     assert_eq!( this.clones(fp!(::Foo=>0,1)), None );
    /// }
    ///
    ///
    /// ```
    ///
    /// [`StructuralExt::cloned_fields`]: ./trait.StructuralExt.html#method.cloned_fields
    #[inline(always)]
    pub fn clones<'a, P>(&'a self, path: P) -> ClonedOut<RevGetMultiFieldOut<'a, P, T>>
    where
        P: RevGetMultiField<'a, T>,
        RevGetMultiFieldOut<'a, P, T>: Cloned,
    {
        path.rev_get_multi_field(&self.0).cloned_()
    }

    /// Gets mutable references to multiple fields,determined by `path`.
    ///
    /// This function is equivalent to [`StructuralExt::fields_mut`],
    /// which has more complete examples.
    ///
    /// # Example: Struct
    ///
    /// ```
    /// use structural::{StrucWrapper, fp};
    /// use structural::structural_aliases::Array5;
    ///
    /// assertions((0,0,8,0,21));
    /// assertions((0,0,8,0,21,""));
    /// assertions((0,0,8,0,21,"",false));
    ///
    /// assertions([0,0,8,0,21]);
    /// assertions([0,0,8,0,21,0]);
    /// assertions([0,0,8,0,21,0,0]);
    ///
    /// fn assertions(this: impl Array5<u64>){
    ///     let mut this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.muts(fp!(2,4)), (&mut 8, &mut 21) );
    /// }
    ///
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```
    /// use structural::{StrucWrapper, fp};
    /// use structural::for_examples::{Enum2, Enum2_SI, Enum3, Enum4};
    ///
    /// assertions(Enum2::Foo(27, 81));
    /// assertions(Enum3::Foo(27, 81));
    /// assertions(Enum4::Foo(27, 81));
    ///
    /// fn assertions(this: impl Enum2_SI){
    ///     let mut this=StrucWrapper(this);
    ///
    ///     assert_eq!( this.muts(fp!(::Foo=>0,1)), Some((&mut 27, &mut 81)) );
    ///     assert_eq!( this.muts(fp!(::Bar=>0)), None );
    /// }
    ///
    ///
    /// ```
    ///
    /// [`StructuralExt::fields_mut`]: ./trait.StructuralExt.html#method.fields_mut
    #[inline(always)]
    pub fn muts<'a, P>(&'a mut self, path: P) -> RevGetMultiFieldMutOut<'a, P, T>
    where
        P: RevGetMultiFieldMut<'a, T>,
    {
        path.rev_get_multi_field_mut(&mut self.0)
    }

    /// Converts this into multiple fields by value, determined by path.
    ///
    /// This function is equivalent to [`StructuralExt::into_fields`],
    /// which has more complete documentation.
    ///
    /// # Valid Paths
    ///
    /// As opposed to the other multiple fields accessor methods,
    /// this method only accepts field paths that refer to
    /// multiple non-nested fields inside some value (possibly a nested field itself).
    ///
    /// For examples of valid and invalid paths to pass as parameter
    /// [look here](./trait.StructuralExt.html#valid_into_field_paths).
    ///
    /// # Example: Struct
    ///
    /// ```rust
    /// use structural::{StrucWrapper, Structural, fp, make_struct};
    ///
    /// assert_eq!(
    ///     into_parts(Bicycle{
    ///         frame: Frame("wheeee"),
    ///         wheels: [Wheel("first"), Wheel("second")],
    ///         handle_bar: HandleBar("hands on me"),
    ///     }),
    ///     (Frame("wheeee"), [Wheel("first"), Wheel("second")], HandleBar("hands on me"))
    /// );
    ///
    /// assert_eq!(
    ///     into_parts(make_struct!{
    ///         frame: Frame("frame"),
    ///         wheels: [Wheel("1"), Wheel("4")],
    ///         handle_bar: HandleBar("9"),
    ///     }),
    ///     (Frame("frame"), [Wheel("1"), Wheel("4")], HandleBar("9"))
    /// );
    ///
    /// // The `Bicycle_SI` was generated for the `Bicycle` struct by the `Structural` derive,
    /// // aliasing it's accessor traits
    /// fn into_parts(this: impl Bicycle_SI)-> (Frame, [Wheel;2], HandleBar) {
    ///     let this = StrucWrapper(this);
    ///
    ///     this.vals(fp!(frame, wheels, handle_bar))
    /// }
    ///
    /// #[derive(Structural,Debug,Copy,Clone,PartialEq)]
    /// struct Bicycle{
    ///     pub frame: Frame,
    ///     pub wheels: [Wheel;2],
    ///     pub handle_bar: HandleBar,
    /// }
    ///
    /// # #[derive(Debug,Copy,Clone,PartialEq)]
    /// # struct Frame(&'static str);
    /// #
    /// # #[derive(Debug,Copy,Clone,PartialEq)]
    /// # struct Wheel(&'static str);
    /// #
    /// # #[derive(Debug,Copy,Clone,PartialEq)]
    /// # struct HandleBar(&'static str);
    ///
    /// ```
    ///
    /// # Example: Enum
    ///
    /// ```rust
    /// use structural::{StrucWrapper, Structural, TS, fp, make_struct};
    ///
    /// let human=Human{name: "bob".into(), gold: 600};
    /// assert_eq!( enum_into_human(Entities::Human(human.clone())), Some(human.clone()) );
    ///
    /// assert_eq!(
    ///     enum_into_human(MoreEntities::Human{name: "John".into(), gold: 1234}),
    ///     Some(Human{name: "John".into(), gold: 1234})
    /// );
    ///
    /// // The `Human_VSI` trait was generated for `Human` by the `Structural` derive macro,
    /// // to access variants with the same fields as Human.
    /// // The `TS!(Human)` argument makes it require the variant to be `Human`.
    /// fn enum_into_human(this: impl Human_VSI<TS!(Human)>)-> Option<Human> {
    ///     let this= StrucWrapper(this);
    ///     let (name, gold)= this.vals(fp!(::Human=>name,gold))?;
    ///     Some(Human{name, gold})
    /// }
    ///
    /// #[derive(Structural, Clone, Debug, PartialEq)]
    /// pub struct Human{
    ///     pub name: String,
    ///     pub gold: u64,
    /// }
    ///
    /// #[derive(Structural, Clone, Debug, PartialEq)]
    /// pub enum Entities {
    ///     #[struc(newtype(bounds = "Human_VSI<@variant>"))]
    ///     Human(Human),
    ///     Wolf,
    /// }
    ///
    /// #[derive(Structural, Clone, Debug, PartialEq)]
    /// # #[struc(no_trait)]
    /// pub enum MoreEntities {
    ///     Human{
    ///         name: String,
    ///         gold: u64,
    ///     },
    ///     Wolf,
    ///     Cat,
    ///     Dog,
    /// }
    ///
    ///
    /// ```
    ///
    /// [`StructuralExt::into_fields`]: ./trait.StructuralExt.html#method.into_fields
    #[inline(always)]
    pub fn vals<P>(self, path: P) -> RevIntoMultiFieldOut<P, T>
    where
        P: RevIntoMultiField<T>,
    {
        path.rev_into_multi_field(self.0)
    }

    /// Queries whether an enum is a particular variant.
    ///
    /// This function is equivalent to [`StructuralExt::is_variant`].
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{StrucWrapper, Structural, fp};
    ///
    /// assertions(
    ///     EnumOne::Bar,
    ///     EnumOne::Baz{x:0, y:100},
    ///     EnumOne::Qux("hello", "world"),
    /// );
    ///
    /// assertions(EnumTwo::Bar, EnumTwo::Baz, EnumTwo::Qux);
    ///
    /// fn assertions<T>(bar: T, baz: T, qux: T)
    /// where
    ///     T: EnumTwo_SI
    /// {
    ///     let bar=StrucWrapper(bar);
    ///     let baz=StrucWrapper(baz);
    ///     let qux=StrucWrapper(qux);
    ///
    ///     assert!( bar.is_variant(fp!(Bar)) );
    ///     assert!( baz.is_variant(fp!(Baz)) );
    ///     assert!( qux.is_variant(fp!(Qux)) );
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum EnumOne{
    ///     Bar,
    ///     Baz{
    ///         x:u32,
    ///         y:u32,
    ///     },
    ///     Qux(&'static str, &'static str),
    /// }
    ///
    /// #[derive(Structural)]
    /// enum EnumTwo{
    ///     Bar,
    ///     Baz,
    ///     Qux,
    /// }
    ///
    /// ```
    ///
    /// [`StructuralExt::is_variant`]: ./trait.StructuralExt.html#method.is_variant
    #[inline(always)]
    pub fn is_variant<P>(&self, _path: P) -> bool
    where
        P: IsTStr,
        T: IsVariant<P>,
    {
        IsVariant::is_variant_(&self.0, _path)
    }

    /// Converts this into another structural type,using `IntoStructural`.
    ///
    /// # Struct Example
    ///
    /// ```rust
    /// use structural::{
    ///     for_examples::{StructFoo, StructBar},
    ///     FP, IntoField, StrucWrapper, make_struct,
    /// };
    ///
    /// assert_eq!( into_foo(make_struct!{foo: 100}), StructFoo{foo: 100} );
    /// assert_eq!( into_foo(make_struct!{foo: 200}), StructFoo{foo: 200} );
    ///
    /// assert_eq!( into_bar(make_struct!{bar: 3}), StructBar{bar: 3} );
    /// assert_eq!( into_bar(make_struct!{bar: 5}), StructBar{bar: 5} );
    ///
    /// fn into_foo<T>(this: T)->StructFoo<T::Ty>
    /// where
    ///     T: IntoField<FP!(foo)>
    /// {
    ///     let this=StrucWrapper(this);
    ///     this.into_struc()
    /// }
    ///
    /// fn into_bar<T>(this: T)->StructBar<T::Ty>
    /// where
    ///     T: IntoField<FP!(bar)>
    /// {
    ///     let this=StrucWrapper(this);
    ///     this.into_struc()
    /// }
    ///
    /// ```
    ///
    /// # Enum Example
    ///
    /// ```rust
    /// use structural::{
    ///     convert::{EmptyTryFromError, FromStructural, TryFromError, TryFromStructural},
    ///     for_examples::Enum3,
    ///     Structural, StrucWrapper, make_struct, switch,
    /// };
    ///
    /// use std::cmp::Ordering;
    ///
    /// assert_eq!( into_v(Enum3::Foo(3, 8)), NoPayload::Foo );
    /// assert_eq!( into_v(Enum3::Bar(Ordering::Less, None)), NoPayload::Bar );
    /// assert_eq!( into_v(Enum3::Baz{foom: "what"}), NoPayload::Baz );
    ///
    /// assert_eq!( into_v(WithPayload::Foo(13)), NoPayload::Foo );
    /// assert_eq!( into_v(WithPayload::Bar(21)), NoPayload::Bar );
    /// assert_eq!( into_v(WithPayload::Baz(34)), NoPayload::Baz );
    ///
    /// assert_eq!( into_v(NoPayload::Foo), NoPayload::Foo );
    /// assert_eq!( into_v(NoPayload::Bar), NoPayload::Bar );
    /// assert_eq!( into_v(NoPayload::Baz), NoPayload::Baz );
    ///
    /// // `NoPayload_ESI` was generated by the `Structural` derive macro for `NoPayload`,
    /// // aliasing its accessor trait impls.
    /// fn into_v(this: impl NoPayload_ESI)->NoPayload {
    ///     let this = StrucWrapper(this);
    ///     this.into_struc()
    /// }
    ///
    /// #[derive(Debug,Structural,PartialEq)]
    /// enum WithPayload<T>{
    ///     Foo(T),
    ///     Bar(T),
    ///     Baz(T),
    /// }
    ///
    /// #[derive(Debug,Structural,PartialEq)]
    /// enum NoPayload{
    ///     Foo,
    ///     Bar,
    ///     Baz,
    /// }
    ///
    /// // This macro allows enums to only implement the `TryFromStructural` trait,
    /// // delegating the `FromStructural` trait to it.
    /// structural::z_impl_try_from_structural_for_enum!{
    ///     impl[F] TryFromStructural<F> for NoPayload
    ///     where[ F: NoPayload_SI, ]
    ///     {
    ///         type Error = EmptyTryFromError;
    ///
    ///          fn try_from_structural(this){
    ///              switch!{this;
    ///                  Foo => Ok(NoPayload::Foo),
    ///                  Bar => Ok(NoPayload::Bar),
    ///                  Baz => Ok(NoPayload::Baz),
    ///                  _ => Err(TryFromError::with_empty_error(this)),
    ///              }
    ///          }
    ///     }
    ///     FromStructural
    ///     where[ F: NoPayload_ESI, ]
    /// }
    ///
    ///
    /// ```
    #[inline(always)]
    pub fn into_struc<U>(self) -> U
    where
        T: IntoStructural<U>,
    {
        self.0.into_structural()
    }

    /// Performs a fallible conversion into another structural type using `TryIntoStructural`.
    ///
    /// # Enum example
    ///
    /// ```rust
    /// use structural::{
    ///     convert::{EmptyTryFromError, TryFromError},
    ///     for_examples::ResultLike,
    ///     Structural, StructuralExt, switch,
    /// };
    ///
    /// assert_eq!(
    ///     MoreCommand::Open(Door::Green).try_into_struc::<Command>(),
    ///     Ok(Command::Open(Door::Green)),
    /// );
    /// assert_eq!(
    ///     MoreCommand::Close(Door::Green).try_into_struc::<Command>(),
    ///     Ok(Command::Close(Door::Green)),
    /// );
    ///
    /// let lock_cmd = MoreCommand::Lock(Door::Green, Key(12345678));
    /// assert_eq!(
    ///     lock_cmd.clone().try_into_struc::<Command>(),
    ///     Err(TryFromError::with_empty_error(lock_cmd)),
    /// );
    ///
    ///
    /// #[derive(Debug,Clone,Structural,PartialEq)]
    /// enum Command{
    ///     Open(Door),
    ///     Close(Door),
    /// }
    ///
    /// #[derive(Debug,Clone,Structural,PartialEq)]
    /// enum MoreCommand{
    ///     Open(Door),
    ///     Close(Door),
    ///     Lock(Door, Key),
    /// }
    ///
    /// # #[derive(Debug,Copy,Clone,PartialEq)]
    /// # enum Door{
    /// #     Red,
    /// #     Blue,
    /// #     Green,
    /// # }
    /// #
    /// # #[derive(Debug,Copy,Clone,PartialEq)]
    /// # struct Key(u128);
    ///
    /// // This macro implements FromStructural in terms of TryFromStructural,
    /// structural::z_impl_try_from_structural_for_enum!{
    ///     impl[F] TryFromStructural<F> for Command
    ///     where[ F: Command_SI, ]
    ///     {
    ///         type Error = EmptyTryFromError;
    ///
    ///         fn try_from_structural(this){
    ///             switch! {this;
    ///                 Open(door) => Ok(Self::Open(door)),
    ///                 Close(door) => Ok(Self::Close(door)),
    ///                 _ => Err(TryFromError::with_empty_error(this)),
    ///             }
    ///         }
    ///     }
    ///
    ///     // `Command_ESI` was generated by the `Structural` derive for `Command`
    ///     // aliasing its accessor trait impls,
    ///     // and requires `F` to only have the `Open`,and `Close` variants.
    ///     FromStructural
    ///     where[ F: Command_ESI, ]
    /// }
    ///
    /// ```
    ///
    #[inline(always)]
    pub fn try_into_struc<U>(self) -> Result<U, TryFromError<T, T::Error>>
    where
        T: TryIntoStructural<U>,
    {
        self.0.try_into_structural()
    }
}

impl<T> StrucWrapper<T> {
    /// Turns a `&StrucWrapper<T>` into a `StrucWrapper<&T>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use structural::{Structural, StrucWrapper, fp, make_struct};
    ///
    /// {
    ///     let this= StrucWrapper(Struct3{foo:Some(13), bar:21, baz:"34"});
    ///     with_struct3(this.as_ref());
    ///     // Because of the `.as_ref()`,`this` wasn't consumed
    ///     with_struct3(this.as_ref());
    /// }
    /// {
    ///     let this= StrucWrapper(make_struct!{foo:Some(13), bar:21, baz:"34", quax:false});
    ///     with_struct3(this.as_ref());
    ///     // Because of the `.as_ref()`,`this` wasn't consumed
    ///     with_struct3(this.as_ref());
    /// }
    ///
    /// // The `Struct3_SI` trait was declared for Struct3 by the Structural derive macro,
    /// // aliasing its accessor trait impls
    /// //
    /// // Also, notice how this also requires `Copy`,even though Struct3 doesn't implement it?
    /// // The call with Struct3 works because of the `.as_ref()`,
    /// // since `&` always implements `Copy`.
    /// fn with_struct3(this: StrucWrapper<impl Struct3_SI<u8, u16, &'static str> + Copy>){
    ///     assert_eq!( this.r(fp!(foo?)), Some(&13) );
    ///     assert_eq!( this.r(fp!(bar)), &21 );
    ///     assert_eq!( this.r(fp!(baz)), &"34" );
    /// }
    ///     
    /// #[derive(Structural, Debug)]
    /// // With this attribute, you can only access fields by shared reference.
    /// #[struc(access="ref")]
    /// pub struct Struct3<A, B, C> {
    ///     pub foo: Option<A>,
    ///     pub bar: B,
    ///     pub baz: C,
    /// }
    ///
    /// ```
    #[inline(always)]
    pub fn as_ref(&self) -> StrucWrapper<&T> {
        StrucWrapper(&self.0)
    }

    /// Turns a `&mut StrucWrapper<T>` into a `StrucWrapper<&mut T>`.
    /// Turns a `&StrucWrapper<T>` into a `StrucWrapper<&T>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use structural::{Structural, StrucWrapper, fp, make_struct};
    ///
    /// {
    ///     let mut this= StrucWrapper(Struct3{foo:Some(13), bar:21, baz:"34"});
    ///     with_struct3(this.as_mut());
    ///     // Because of the `.as_mut()`,`this` wasn't consumed
    ///     with_struct3(this.as_mut());
    /// }
    /// {
    ///     let mut this= StrucWrapper(make_struct!{foo:Some(13), bar:21, baz:"34", quax:false});
    ///     with_struct3(this.as_mut());
    ///     // Because of the `.as_mut()`,`this` wasn't consumed
    ///     with_struct3(this.as_mut());
    /// }
    ///
    /// // The `Struct3_SI` trait was declared for Struct3 by the Structural derive macro,
    /// // aliasing its accessor trait impls
    /// fn with_struct3(mut this: StrucWrapper<impl Struct3_SI<u8, u16, &'static str>>){
    ///     assert_eq!( this.r(fp!(foo?)), Some(&13) );
    ///     assert_eq!( this.r(fp!(bar)), &21 );
    ///     assert_eq!( this.r(fp!(baz)), &"34" );
    ///
    ///     assert_eq!( this.m(fp!(foo?)), Some(&mut 13) );
    ///     assert_eq!( this.m(fp!(bar)), &mut 21 );
    ///     assert_eq!( this.m(fp!(baz)), &mut "34" );
    /// }
    ///     
    /// #[derive(Structural, Debug)]
    /// // With this attribute, you can only access fields by shared or mutable reference.
    /// #[struc(access="mut")]
    /// pub struct Struct3<A, B, C> {
    ///     pub foo: Option<A>,
    ///     pub bar: B,
    ///     pub baz: C,
    /// }
    ///
    /// ```
    #[inline(always)]
    pub fn as_mut(&mut self) -> StrucWrapper<&mut T> {
        StrucWrapper(&mut self.0)
    }

    /// Transforms the wrapped value with the `func` function.
    ///
    #[inline(always)]
    pub fn map<F, U>(self, f: F) -> StrucWrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        StrucWrapper(f(self.0))
    }

    /// Calls `func` with `self`,rewrapping its return value in a `StrucWrapper<U>`
    ///
    #[inline(always)]
    pub fn then<F, U>(self, f: F) -> StrucWrapper<U>
    where
        F: FnOnce(Self) -> U,
    {
        StrucWrapper(f(self))
    }
}

impl<'a, T> StrucWrapper<&'a T> {
    /// Turns a `StrucWrapper<&T>` into a `&StrucWrapper<T>`.
    ///
    /// Note that this only works if `T: Sized`,
    /// which means that you can't call this method on a `StrucWrapper<&dyn Trait>`.
    #[inline(always)]
    pub fn reref(self) -> &'a StrucWrapper<T> {
        // `Self` is a `#[repr(transparent)]` wrapper around `T`
        unsafe { &*(self.0 as *const T as *const StrucWrapper<T>) }
    }
}

impl<'a, T> StrucWrapper<&'a mut T> {
    /// Turns a `StrucWrapper<&mut T>` into a `&mut StrucWrapper<T>`.
    ///
    /// Note that this only works if `T: Sized`,
    /// which means that you can't call this method on a `StrucWrapper<&mut dyn Trait>`.
    #[inline(always)]
    pub fn remut(self) -> &'a mut StrucWrapper<T> {
        // `Self` is a `#[repr(transparent)]` wrapper around `T`
        unsafe { &mut *(self.0 as *mut T as *mut StrucWrapper<T>) }
    }
}

/// Gets a reference to a non-nested struct field
///
/// # Example
///
/// ```
/// use structural::{StrucWrapper, fp};
/// use structural::structural_aliases::Array4;
///
/// assertions(["hello","world","foo","bar"]);
/// assertions(["hello","world","foo","bar","baz"]);
///
/// assertions(("hello","world","foo","bar"));
/// assertions(("hello","world","foo","bar","baz"));
/// assertions(("hello","world","foo","bar","baz","qux"));
///
/// fn assertions(this: impl Array4<&'static str> ){
///     let this=StrucWrapper(this);
///
///     assert_eq!( this[fp!(1)], "world" );
///     assert_eq!( this[fp!(2)], "foo" );
///     assert_eq!( this[fp!(3)], "bar" );
/// }
///
/// ```
impl<F, T> Index<F> for StrucWrapper<T>
where
    T: GetField<F>,
{
    type Output = T::Ty;

    #[inline(always)]
    fn index(&self, path: F) -> &T::Ty {
        self.0.get_field_(path)
    }
}

impl<T> ConstDefault for StrucWrapper<T>
where
    T: ConstDefault,
{
    const DEFAULT: Self = StrucWrapper(T::DEFAULT);
}

/// Gets a mutable reference to a non-nested struct field
///
/// # Example
///
/// ```
/// use structural::{StructuralExt, StrucWrapper, fp, make_struct};
/// use structural::for_examples::{Struct3, Struct3_SI};
///
/// let mut this=Struct3{ foo:Some(33), bar:55, baz:77 };
/// let mut anon=make_struct!{ foo:Some(0), bar:0, baz:0, extra:"extra" };
///
/// fn mutator(this:&mut impl Struct3_SI<u32,u32,u32>){
///     let mut this=StrucWrapper(this);
///     
///     this[fp!(bar)]+=20;
///     this[fp!(baz)]+=30;
/// }
///
/// mutator(&mut this);
/// mutator(&mut anon);
///
/// assert_eq!( this.cloned_fields(fp!(foo, bar, baz)), (Some(33), 75, 107) );
/// assert_eq!( anon.cloned_fields(fp!(foo, bar, baz, extra)), (Some(0), 20, 30, "extra") );
///
/// ```
impl<F, T> IndexMut<F> for StrucWrapper<T>
where
    T: GetFieldMut<F>,
{
    #[inline(always)]
    fn index_mut(&mut self, path: F) -> &mut T::Ty {
        self.0.get_field_mut_(path)
    }
}

unsafe_delegate_structural_with! {
    impl[T,] StrucWrapper<T>
    where[]

    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;

    GetField { &this.0 }

    GetFieldMut{
        &mut this.0
    }
    as_delegating_raw{
        this as *mut T
    }

    IntoField{
        this.0
    }
    move_out_field{
        &mut this.0
    }

    DropFields = { dropped_fields[] }

    FromStructural {
        constructor = StrucWrapper;
    }
}
