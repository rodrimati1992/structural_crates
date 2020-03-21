use crate::{
    enums::IsVariant,
    field::{
        NormalizeFields, NormalizeFieldsOut, RevGetFieldImpl, RevGetFieldMutImpl, RevGetMultiField,
        RevGetMultiFieldMut, RevGetMultiFieldMutOut, RevGetMultiFieldOut, RevIntoFieldImpl,
    },
    path::IsTStr,
};

use core_extensions::collection_traits::{Cloned, ClonedOut};

/// A trait defining the primary way to call methods from structural traits.
pub trait GetFieldExt {
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
    ///
    /// # Enum Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// with_circle( &Shape::Circle{ x:3, y:5, radius:8 } );
    /// with_circle( &MoreShapes::Circle{ x:3, y:5, radius:8 } );
    ///
    /// fn with_circle<T>(circle:&T)
    /// where
    ///     // `Shape_SI` was generated for Shape by the `Structural` derive.
    ///     T: Shape_SI
    /// {
    ///     assert_eq!( circle.field_(fp!(::Circle.x)), Some(&3) );
    ///     assert_eq!( circle.field_(fp!(::Circle.y)), Some(&5) );
    ///     assert_eq!( circle.field_(fp!(::Circle.radius)), Some(&8) );
    ///
    ///     // Constructing the variant proxy is the only Option we have to handle here,
    ///     // instead of every access to the fields in the Circle variant being optional.
    ///     //
    ///     // For a more ergonomic alternative,
    ///     // you can look at the example for the `fields` method
    ///     let proxy=circle.field_(fp!(::Circle)).expect("Expected a circle");
    ///     assert_eq!( proxy.field_(fp!(x)), &3 );
    ///     assert_eq!( proxy.field_(fp!(y)), &5 );
    ///     assert_eq!( proxy.field_(fp!(radius)), &8 );
    /// }
    ///
    /// #[derive(Structural)]
    /// enum Shape{
    ///     Circle{x:u32,y:u32,radius:u32},
    ///     Square{x:u32,y:u32,width:u32},
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum MoreShapes{
    ///     Circle{x:u32,y:u32,radius:u32},
    ///     Square{x:u32,y:u32,width:u32},
    ///     Rectangle{x:u32,y:u32,width:u32,height:u32},
    /// }
    ///
    ///
    /// ```
    #[inline(always)]
    fn field_<'a, P>(&'a self, path: P) -> NormalizeFieldsOut<Result<&'a P::Ty, P::Err>>
    where
        P: RevGetFieldImpl<'a, Self>,
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
    ///
    /// # Enum Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// with_car( &Vehicle::Car{ name:"initial-c", km:9001 } );
    /// with_car( &MoreVehicles::Car{ name:"initial-c", km:9001 } );
    ///
    /// fn with_car<T>(car:&T)
    /// where
    ///     // `Vehicle_SI` was generated for Vehicle by the `Structural` derive.
    ///     T: Vehicle_SI
    /// {
    ///     assert_eq!(
    ///         car.fields(fp!(::Car.name, ::Car.km)),
    ///         ( Some(&"initial-c"), Some(&9001) )
    ///     );
    ///
    ///     // You can use `=>` to access multiple fields inside of a nested field(or a variant)
    ///     // this allows accessing multiple fields inside an enum variant without having to
    ///     // create an intermediate variant proxy
    ///     // (look at the next assert for what that looks like).
    ///     assert_eq!( car.fields(fp!(::Car=>name,km)), Some((&"initial-c",&9001)) );
    ///
    ///     assert_eq!(
    ///         // This is equivalent to the field access in the previous assert
    ///         car.field_(fp!(::Car)).map(|vp| vp.fields(fp!(name,km)) ),
    ///         Some((&"initial-c",&9001))
    ///     );
    ///
    ///     assert_eq!( car.cloned_fields(fp!(::Truck=>weight_kg,driven_km)), None);
    /// }
    ///
    /// #[derive(Structural)]
    /// enum Vehicle{
    ///     Car{name: &'static str, km:u32},
    ///     Truck{ weight_kg:u32, driven_km:u32 },
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum MoreVehicles{
    ///     Car{name: &'static str, km:u32},
    ///     Truck{ weight_kg:u32, driven_km:u32 },
    ///     Boat,
    /// }
    ///
    ///
    /// ```
    #[inline(always)]
    fn fields<'a, P>(&'a self, path: P) -> RevGetMultiFieldOut<'a, P, Self>
    where
        P: RevGetMultiField<'a, Self>,
    {
        path.rev_get_multi_field(self)
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
    ///
    /// # Enum Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// with_pc( &Device::Pc{ manufacturer:"dawn", year:2038 } );
    /// with_pc( &MoreDevices::Pc{ manufacturer:"dawn", year:2038 } );
    ///
    /// fn with_pc<T>(pc:&T)
    /// where
    ///     // `Device_SI` was generated for Device by the `Structural` derive.
    ///     T: Device_SI
    /// {
    ///     assert_eq!(
    ///         pc.cloned_fields(fp!(::Pc.manufacturer, ::Pc.year)),
    ///         ( Some("dawn"), Some(2038) )
    ///     );
    ///
    ///     // You can use `=>` to access multiple fields inside of a nested field(or a variant)
    ///     // this allows accessing multiple fields inside an enum variant without having to
    ///     // create an intermediate variant proxy
    ///     // (look at the next assert for what that looks like).
    ///     assert_eq!( pc.cloned_fields(fp!(::Pc=>manufacturer,year)), Some(("dawn",2038)) );
    ///
    ///     assert_eq!(
    ///         // This is equivalent to the field access in the previous assert
    ///         pc.field_(fp!(::Pc)).map(|vp| vp.cloned_fields(fp!(manufacturer,year)) ),
    ///         Some(("dawn",2038))
    ///     );
    ///
    ///     assert_eq!( pc.cloned_fields(fp!(::Phone=>number,charge)), None);
    /// }
    ///
    /// #[derive(Structural)]
    /// enum Device{
    ///     Pc{manufacturer: &'static str, year:u32},
    ///     Phone{number:&'static str,charge:u8},
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum MoreDevices{
    ///     Pc{manufacturer: &'static str, year:u32},
    ///     Phone{number:&'static str,charge:u8},
    ///     Tablet,
    /// }
    ///
    ///
    /// ```
    fn cloned_fields<'a, P>(&'a self, path: P) -> ClonedOut<RevGetMultiFieldOut<'a, P, Self>>
    where
        P: RevGetMultiField<'a, Self>,
        RevGetMultiFieldOut<'a, P, Self>: Cloned,
    {
        path.rev_get_multi_field(self).cloned_()
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
    /// # Enum Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// with_soda( &mut Beverage::Soda{ ml:600, cents:400 } );
    /// with_soda( &mut MoreBeverages::Soda{ ml:600, cents:400 } );
    ///
    /// fn with_soda<T>(soda:&mut T)
    /// where
    ///     // `Beverage_SI` was generated for Beverage by the `Structural` derive.
    ///     T: Beverage_SI
    /// {
    ///     assert_eq!( soda.field_mut(fp!(::Soda.ml)), Some(&mut 600) );
    ///     assert_eq!( soda.field_mut(fp!(::Soda.cents)), Some(&mut 400) );
    ///
    ///     // Constructing the variant proxy is the only Option we have to handle here,
    ///     // instead of every access to the fields in the Soda variant being optional.
    ///     let proxy=soda.field_mut(fp!(::Soda)).expect("Expected a soda");
    ///     assert_eq!( proxy.field_mut(fp!(ml)), &mut 600 );
    ///     assert_eq!( proxy.field_mut(fp!(cents)), &mut 400 );
    /// }
    ///
    /// #[derive(Structural)]
    /// enum Beverage{
    ///     Soda{ ml:u32, cents:u32 },
    ///     Water,
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum MoreBeverages{
    ///     Soda{ ml:u32, cents:u32 },
    ///     Water,
    ///     Beer,
    /// }
    ///
    ///
    /// ```
    #[inline(always)]
    fn field_mut<'a, P>(&'a mut self, path: P) -> NormalizeFieldsOut<Result<&'a mut P::Ty, P::Err>>
    where
        P: RevGetFieldMutImpl<'a, Self>,
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
    ///
    /// # Enum Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// with_book( &mut Medium::Book{ pages:500, title:"Dracular" } );
    /// with_book( &mut MoreMedia::Book{ pages:500, title:"Dracular" } );
    ///
    /// fn with_book<T>(book:&mut T)
    /// where
    ///     // `Medium_SI` was generated for Medium by the `Structural` derive.
    ///     T: Medium_SI
    /// {
    ///     assert_eq!(
    ///         book.fields_mut(fp!(::Book.pages, ::Book.title)),
    ///         ( Some(&mut 500), Some(&mut "Dracular") )
    ///     );
    ///
    ///     // You can use `=>` to access multiple fields inside of a nested field(or a variant)
    ///     // this allows accessing multiple fields inside an enum variant without having to
    ///     // create an intermediate variant proxy
    ///     // (look at the next assert for what that looks like).
    ///     assert_eq!(
    ///         book.fields_mut(fp!(::Book=>pages,title)),
    ///         Some((&mut 500,&mut "Dracular")),
    ///     );
    ///
    ///     assert_eq!(
    ///         // This is equivalent to the field access in the previous assert
    ///         book.field_mut(fp!(::Book)).map(|vp| vp.fields_mut(fp!(pages,title)) ),
    ///         Some((&mut 500,&mut "Dracular"))
    ///     );
    ///
    ///     assert_eq!( book.fields_mut(fp!(::Comic=>artist,in_color)), None);
    /// }
    ///
    /// #[derive(Structural)]
    /// enum Medium{
    ///     Book{ pages:u32, title:&'static str },
    ///     Comic{artist:&'static str,in_color:bool},
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum MoreMedia{
    ///     Book{ pages:u32, title:&'static str },
    ///     Comic{artist:&'static str,in_color:bool},
    ///     Television,
    /// }
    ///
    /// ```
    #[inline(always)]
    fn fields_mut<'a, P>(&'a mut self, path: P) -> RevGetMultiFieldMutOut<'a, P, Self>
    where
        P: RevGetMultiFieldMut<'a, Self>,
    {
        path.rev_get_multi_field_mut(self)
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
    ///
    /// # Enum Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// with_table( &Furniture::Table{ height_cm:101, width_cm:333 } );
    /// with_table( &MoreFurniture::Table{ height_cm:101, width_cm:333 } );
    ///
    /// fn with_table<T>(table:&T)
    /// where
    ///     // `Furniture_SI` was generated for Furniture by the `Structural` derive.
    ///     T: Furniture_SI + Clone
    /// {
    ///     assert_eq!( table.clone().into_field(fp!(::Table.height_cm)), Some(101) );
    ///     assert_eq!( table.clone().into_field(fp!(::Table.width_cm)), Some(333) );
    ///
    ///     // Constructing the variant proxy is the only Option we have to handle here,
    ///     // instead of every access to the fields in the Table variant being optional.
    ///     let proxy=table.clone().into_field(fp!(::Table)).expect("Expected a table");
    ///     assert_eq!( proxy.clone().into_field(fp!(height_cm)), 101 );
    ///     assert_eq!( proxy.clone().into_field(fp!(width_cm)), 333 );
    /// }
    ///
    /// #[derive(Structural,Clone)]
    /// enum Furniture{
    ///     Table{ height_cm:u32, width_cm:u32 },
    ///     Chair,
    /// }
    ///
    /// #[derive(Structural,Clone)]
    /// # #[struc(no_trait)]
    /// enum MoreFurniture{
    ///     Table{ height_cm:u32, width_cm:u32 },
    ///     Chair,
    ///     Sofa,
    /// }
    ///
    ///
    /// ```
    #[inline(always)]
    fn into_field<'a, P>(self, path: P) -> NormalizeFieldsOut<Result<P::Ty, P::Err>>
    where
        P: RevIntoFieldImpl<'a, Self>,
        P::Ty: Sized,
        Result<P::Ty, P::Err>: NormalizeFields,
        Self: Sized,
    {
        path.rev_into_field(self).normalize_fields()
    }

    /// Checks whether an enum is a particular variant.
    ///
    /// # Example
    ///
    /// ```
    /// use structural::{GetFieldExt,Structural,fp};
    ///
    /// check_colors( &Color::Red, &Color::Blue, &Color::Green );
    /// check_colors( &ColorPlus::Red, &ColorPlus::Blue, &ColorPlus::Green );
    ///
    /// fn check_colors<T>( red:&T, blue:&T, green:&T )
    /// where
    ///     // `Color_SI` was declared by the `Structural` derive on `Color`.
    ///     T: Color_SI
    /// {
    ///     assert!(  red.is_variant(fp!(Red)) );
    ///     assert!( !red.is_variant(fp!(Blue)) );
    ///     assert!( !red.is_variant(fp!(Green)) );
    ///
    ///     assert!( !blue.is_variant(fp!(Red)) );
    ///     assert!(  blue.is_variant(fp!(Blue)) );
    ///     assert!( !blue.is_variant(fp!(Green)) );
    ///
    ///     assert!( !green.is_variant(fp!(Red)) );
    ///     assert!( !green.is_variant(fp!(Blue)) );
    ///     assert!(  green.is_variant(fp!(Green)) );
    /// }
    ///
    /// #[derive(Structural)]
    /// enum Color{
    ///     Red,
    ///     Blue,
    ///     Green,
    /// }
    ///
    /// #[derive(Structural)]
    /// # #[struc(no_trait)]
    /// enum ColorPlus{
    ///     Red,
    ///     Blue,
    ///     Green,
    ///     Teal,
    ///     White,
    ///     Gray,
    ///     Black,
    /// }
    ///
    ///
    /// ```
    #[inline(always)]
    fn is_variant<P>(&self, _path: P) -> bool
    where
        P: IsTStr,
        Self: IsVariant<P>,
    {
        IsVariant::is_variant_(self, _path)
    }
}

impl<T: ?Sized> GetFieldExt for T {}
