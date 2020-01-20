use super::*;

use crate::{
    enum_traits::IsVariant,
    field_traits::{
        multi_fields::{RevGetMultiFieldMutOut, RevGetMultiFieldOut},
        RevGetField, RevGetFieldMut, RevIntoField,
    },
    IsStructural,
};

use core_extensions::collection_traits::Cloned;

/// A trait defining the primary way to call methods from structural traits.
pub trait GetFieldExt: IsStructural {
    /// Gets a reference to a field,determined by `path`.
    ///
    /// This is named `field_` instead of `field`
    /// because `field` collides with the `DebugTuple`/`DebugStruct` method
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp,structural_alias};
    ///
    /// structural_alias!{
    ///     trait EvenFields<A,B,C>{
    ///         0:A,
    ///         2:B,
    ///         4:C,
    ///     }
    /// }
    ///
    /// fn with_even<T>(this:&T)
    /// where
    ///     T:EvenFields<u32,u32,u32>
    /// {
    ///     assert_eq!( this.field_(fp!(0)), &1 );
    ///     assert_eq!( this.field_(fp!(2)), &2 );
    ///     assert_eq!( this.field_(fp!(4)), &5 );
    /// }
    ///
    /// fn main(){
    ///     with_even( &(1,0,2,0,5) );
    ///     with_even( &(1,0,2,0,5,0) );
    ///     with_even( &(1,0,2,0,5,0,0) );
    ///     with_even( &(1,0,2,0,5,0,0,0) );
    /// }
    ///
    /// ```
    #[inline(always)]
    fn field_<'a, P>(&'a self, path: P) -> NormalizeFieldsOut<Result<&'a P::Ty, P::Err>>
    where
        P: IsFieldPath,
        P: RevGetField<'a, Self>,
        Result<&'a P::Ty, P::Err>: NormalizeFields,
    {
        path.rev_get_field(self).normalize_fields()
    }

    /// Gets references to multiple fields,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp,structural_alias};
    ///
    /// structural_alias!{
    ///     trait OddFields<A,B,C>{
    ///         1:A,
    ///         3:B,
    ///         5:C,
    ///     }
    /// }
    ///
    /// fn with_even(this:&impl OddFields<u32,u32,u32>){
    ///     assert_eq!( this.fields(fp!(1,3,5)), (&22,&44,&77) );
    /// }
    ///
    /// fn main(){
    ///     with_even( &(0,22,0,44,0,77) );
    ///     with_even( &(0,22,0,44,0,77,0) );
    ///     with_even( &(0,22,0,44,0,77,0,0) );
    ///     with_even( &(0,22,0,44,0,77,0,0,0) );
    /// }
    ///
    /// ```
    #[inline(always)]
    fn fields<'a, P>(&'a self, path: P) -> NormalizeFieldsOut<RevGetMultiFieldOut<'a, P, Self>>
    where
        P: RevGetMultiField<'a, Self>,
    {
        path.rev_get_multi_field(self).normalize_fields()
    }

    /// Gets clones of multiple fields,determined by `path`.
    ///
    /// # Example
    ///
    /// This example also uses the reexported `IntoArray` trait,
    /// which allows converting homogeneous tuples to arrays,
    /// in this case it's used to iterate over the fields.
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp,make_struct};
    /// use structural::reexports::IntoArray;
    ///
    /// // The `Fruits_SI` trait was declared by the `Structural` derive on `Fruits`.
    /// fn total_fruit_count(fruits:&dyn Fruits_SI)->u32{
    ///     fruits
    ///         .cloned_fields(fp!( apples, oranges, tangerines, tomatoes ))
    ///         .into_array()
    ///         .iter()
    ///         .sum()
    /// }
    ///
    /// {
    ///     let fruits=Fruits{
    ///         apples:1,
    ///         oranges:2,
    ///         tangerines:3,
    ///         tomatoes:5,
    ///     };
    ///     
    ///     assert_eq!( total_fruit_count(&fruits), 11 );
    /// }
    ///
    /// {
    ///     let fruits=make_struct!{
    ///         apples:8,
    ///         oranges:13,
    ///         tangerines:21,
    ///         tomatoes:34,
    ///     };
    ///     
    ///     assert_eq!( total_fruit_count(&fruits), 76 );
    /// }
    ///
    /// #[derive(Structural)]
    /// // We only get read access to the fields.
    /// #[struc(public,access="ref")]
    /// struct Fruits{
    ///     apples:u32,
    ///     oranges:u32,
    ///     tangerines:u32,
    ///     tomatoes:u32,
    /// }
    ///
    /// ```
    fn cloned_fields<'a, P>(
        &'a self,
        path: P,
    ) -> <NormalizeFieldsOut<RevGetMultiFieldOut<'a, P, Self>> as Cloned>::Cloned
    where
        P: RevGetMultiField<'a, Self>,
        NormalizeFieldsOut<RevGetMultiFieldOut<'a, P, Self>>: Cloned,
    {
        path.rev_get_multi_field(self).normalize_fields().cloned_()
    }

    /// Gets a mutable reference to a field,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,fp,make_struct,Structural};
    ///
    /// #[derive(Structural)]
    /// struct Human{
    ///     pub x:i32,
    ///     pub y:i32,
    ///     pub health:u32,
    ///     flags:u32,
    /// }
    ///
    /// // The `Human_SI` trait was declared by the `Structural` derive on `Human`.
    /// fn move_human( this:&mut dyn Human_SI, dx:i32, dy:i32 ){
    ///     *this.field_mut(fp!(x))+=dx;
    ///     *this.field_mut(fp!(y))+=dy;
    /// }
    ///
    /// {
    ///     let mut entity=make_struct!{
    ///         x: 0,
    ///         y: 0,
    ///         health: 100,
    ///     };
    ///     move_human(&mut entity,-100,300);
    ///     assert_eq!( entity.fields(fp!(x,y,health)), (&-100,&300,&100) )
    /// }
    /// {
    ///     let mut entity=Human{
    ///         x: -1000,
    ///         y: 1000,
    ///         health: 1,
    ///         flags: 0b11111,
    ///     };
    ///     
    ///     move_human(&mut entity,500,-200);
    ///
    ///     assert_eq!( entity.x, -500 );
    ///     assert_eq!( entity.y, 800 );
    ///     assert_eq!( entity.health, 1 );
    ///     assert_eq!( entity.flags, 0b11111 );
    /// }
    ///
    /// ```
    ///
    #[inline(always)]
    fn field_mut<'a, P>(&'a mut self, path: P) -> NormalizeFieldsOut<Result<&'a mut P::Ty, P::Err>>
    where
        P: IsFieldPath,
        P: RevGetFieldMut<'a, Self>,
        Result<&'a mut P::Ty, P::Err>: NormalizeFields,
    {
        path.rev_get_field_mut(self).normalize_fields()
    }

    /// Gets mutable references to multiple fields,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{
    ///     GetFieldExt,GetFieldMut,GetFieldType,Structural,
    ///     fp,field_path_aliases,
    /// };
    ///
    /// field_path_aliases!{
    ///     mod names{x,y}
    /// }
    ///
    /// fn swap_coordinates<T,U>(this:&mut T)
    /// where
    ///     T:GetFieldMut<names::x,Ty=U>,
    ///     T:GetFieldMut<names::y,Ty=U>,
    /// {
    ///     let (x,y)=this.fields_mut(fp!(x,y));
    ///     std::mem::swap(x,y);
    /// }
    ///
    /// {
    ///     let mut this=Point2D{ x:100, y:300 };
    ///     swap_coordinates(&mut this);
    ///     assert_eq!( this.x, 300 );
    ///     assert_eq!( this.y, 100 );
    /// }
    ///
    /// {
    ///     let mut this=Point3D{ x:30, y:0, z:500 };
    ///     swap_coordinates(&mut this);
    ///     assert_eq!( this.x, 0 );
    ///     assert_eq!( this.y, 30 );
    ///     assert_eq!( this.z, 500 );
    /// }
    ///
    /// #[derive(Structural)]
    /// struct Point2D<T>{
    ///     pub x:T,
    ///     pub y:T,
    /// }
    ///
    /// #[derive(Structural)]
    /// struct Point3D<T>{
    ///     pub x:T,
    ///     pub y:T,
    ///     pub z:T,
    /// }
    ///
    ///
    /// ```
    ///
    /// # Example
    ///
    /// An example of how this method does not allow multiple mutable borrows
    /// of the same field.
    ///
    /// ```compile_fail
    /// use structural::{GetFieldExt,fp};
    ///
    /// let mut tup=(1,1,2,3,5,8);
    ///
    /// let _=tup.fields_mut(fp!(4,4));
    ///
    /// ```
    #[inline(always)]
    fn fields_mut<'a, P>(
        &'a mut self,
        path: P,
    ) -> NormalizeFieldsOut<RevGetMultiFieldMutOut<'a, P, Self>>
    where
        P: IsFieldPathSet<PathUniqueness = UniquePaths>,
        P: RevGetMultiFieldMut<'a, Self>,
    {
        path.rev_get_multi_field_mut(self).normalize_fields()
    }

    /// Converts ´self´ into a field,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    ///
    /// #[derive(Structural,Clone)]
    /// #[struc(public,access="move")]
    /// struct Tupled<T>(T,T,T,T);
    ///
    /// // The `Tupled_SI` trait was declared by the `Structural` derive on `Tupled`.
    /// fn pick_index<T>(this:impl Tupled_SI<T>,which_one:u32)->T{
    ///     match which_one % 4 {
    ///         0=>this.into_field(fp!(0)),
    ///         1=>this.into_field(fp!(1)),
    ///         2=>this.into_field(fp!(2)),
    ///         _=>this.into_field(fp!(3)),
    ///     }
    /// }
    ///
    /// {
    ///     let tup=Tupled(13,21,34,55);
    ///     
    ///     assert_eq!( pick_index(tup.clone(),0), 13 );
    ///     assert_eq!( pick_index(tup.clone(),1), 21 );
    ///     assert_eq!( pick_index(tup.clone(),2), 34 );
    ///     assert_eq!( pick_index(tup        ,3), 55 );
    /// }
    ///
    /// {
    ///     let array=[13,21,34,55];
    ///     
    ///     assert_eq!( pick_index(array.clone(),0), 13 );
    ///     assert_eq!( pick_index(array.clone(),1), 21 );
    ///     assert_eq!( pick_index(array.clone(),2), 34 );
    ///     assert_eq!( pick_index(array        ,3), 55 );
    /// }
    ///
    /// ```
    #[inline(always)]
    fn into_field<'a, P>(self, path: P) -> NormalizeFieldsOut<Result<P::Ty, P::Err>>
    where
        P: IsFieldPath,
        P: RevIntoField<'a, Self>,
        P::Ty: Sized,
        Result<P::Ty, P::Err>: NormalizeFields,
        Self: Sized,
    {
        path.rev_into_field(self).normalize_fields()
    }

    /// Converts a boxed ´self´ into a field,determined by `path`.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp,make_struct,structural_alias};
    ///
    /// structural_alias!{
    ///     trait Pair<T>{
    ///         left :T,
    ///         right:T,
    ///     }
    /// }
    ///
    /// fn pick_from_pair<T>(this:Box<dyn Pair<T>>,which_one:u32)->T{
    ///     match which_one%2 {
    ///         0=>this.box_into_field(fp!(left )),
    ///         _=>this.box_into_field(fp!(right)),
    ///     }
    /// }
    ///
    /// fn main(){
    ///     let this=Box::new(make_struct!{
    ///         #![derive(Clone)]
    ///         left: "foo".to_string(),
    ///         right: "bar".to_string(),
    ///     });
    ///
    ///     assert_eq!( pick_from_pair(this.clone(),0), "foo" );
    ///     assert_eq!( pick_from_pair(this        ,1), "bar" );
    /// }
    ///
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn box_into_field<'a, P>(
        self: crate::alloc::boxed::Box<Self>,
        path: P,
    ) -> NormalizeFieldsOut<Result<P::BoxedTy, P::Err>>
    where
        P: RevIntoField<'a, Self>,
        P::BoxedTy: Sized,
        Result<P::BoxedTy, P::Err>: NormalizeFields,
    {
        path.rev_box_into_field(self).normalize_fields()
    }

    /// Checks whether an enum is a particular variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// #[derive(Structural)]
    /// enum Color{
    ///     Red,
    ///     Blue,
    ///     Green,
    /// }
    ///
    /// fn main(){
    ///     assert!(  Color::Red.is_variant(fp!(Red)) );
    ///     assert!( !Color::Red.is_variant(fp!(Blue)) );
    ///     assert!( !Color::Red.is_variant(fp!(Green)) );
    ///
    ///     assert!( !Color::Blue.is_variant(fp!(Red)) );
    ///     assert!(  Color::Blue.is_variant(fp!(Blue)) );
    ///     assert!( !Color::Blue.is_variant(fp!(Green)) );
    ///
    ///     assert!( !Color::Green.is_variant(fp!(Red)) );
    ///     assert!( !Color::Green.is_variant(fp!(Blue)) );
    ///     assert!(  Color::Green.is_variant(fp!(Green)) );
    /// }
    ///
    /// ```
    #[inline(always)]
    fn is_variant<P>(&self, _path: P) -> bool
    where
        P: IsFieldPath,
        Self: IsVariant<P>,
    {
        IsVariant::is_variant_(self, _path)
    }
}

impl<T: ?Sized + IsStructural> GetFieldExt for T {}
