#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter{
    ( 
        $(unsafe)?
        impl[$($typarams:tt)*]
            GetField <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::GetField<$name_param> for $self_
        $( where $($where_)* )?
        {
            type Ty=$field_ty;

            fn get_field_(&self)->&Self::Ty{
                &self.$field_name
            }
        }
    };
    ( 
        impl[$($typarams:tt)*]
            GetFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] GetField<$field_name:$field_ty,$name_param> for $self_
            $( where[$($where_)*] )?
        }
    
        impl<$($typarams)*> $crate::GetFieldMut<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                &mut self.$field_name
            }
        }
    };
    ( 
        $(unsafe)?
        impl[$($typarams:tt)*]
            IntoField <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] 
                GetField<$field_name:$field_ty,$name_param> 
            for $self_
            $( where[$($where_)*] )?
        }
    
        impl<$($typarams)*> $crate::IntoField<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn into_field_(self)->Self::Ty{
                self.$field_name
            }
            $crate::impl_box_into_field_method!{$name_param}
        }
    };
    ( 
        impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] 
                GetFieldMut<$field_name:$field_ty,$name_param> 
            for $self_
            $( where[$($where_)*] )?
        }
    
        impl<$($typarams)*> $crate::IntoField<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn into_field_(self)->Self::Ty{
                self.$field_name
            }
            $crate::impl_box_into_field_method!{$name_param}
        }
    };
} 


/// For use in manual implementations of the IntoField trait.
/// 
/// Implements the `IntoField::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoField](./field_traits/trait.IntoField.html)
#[macro_export]
#[cfg(not(feature="alloc"))]
macro_rules! impl_box_into_field_method {
    ($($anything:tt)*) => ()
}

/// For use in manual implementations of the IntoField trait.
/// 
/// Implements the `IntoField::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoField](./field_traits/trait.IntoField.html)
#[macro_export]
#[cfg(feature="alloc")]
macro_rules! impl_box_into_field_method {
    ($field_name:ty) => (
        #[cfg(feature="alloc")]
        fn box_into_field_(self:structural::alloc::boxed::Box<Self>)->Self::Ty{
            $crate::IntoField::<$field_name>::into_field_(*self)
        }
    )
}



#[doc(hidden)]
#[macro_export]
macro_rules! impl_structural{
    (
        impl[$($typarams:tt)*] Structural for $self_:ty 
        where[$($where_:tt)*]
        {
            field_names=[$( ($field_name:expr,$renamed:expr) ,)*]
        }
    )=>{
        impl<$($typarams)*> $crate::Structural for $self_
        where $($where_)*
        {
            const FIELDS:&'static[$crate::structural_trait::FieldInfo]={
                use $crate::structural_trait::FieldInfo;

                &[
                    $( 
                        FieldInfo{
                            original_name:$field_name,
                            accessor_name:$renamed,
                        },
                    )*
                ]
            };
        }

        impl<$($typarams)*> $crate::structural_trait::StructuralDyn for $self_
        where $($where_)*
        {
            fn fields_info(&self)->&'static[$crate::structural_trait::FieldInfo]{
                <Self as $crate::Structural>::FIELDS
            }
        }

    }
}



/// Implements StructuralDyn for some type,by delegating to Structural. 
#[macro_export]
macro_rules! impl_structural_dyn{
    (
        impl[$($typarams:tt)*] $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        impl<$($typarams)*> $crate::structural_trait::StructuralDyn for $self_
        $( where $($where_)* )?
        {
            fn fields_info(&self)->&'static[$crate::structural_trait::FieldInfo]{
                <Self as $crate::Structural>::FIELDS
            }
        }
    }
}



#[doc(hidden)]
#[macro_export]
macro_rules! impl_getters_for_derive{
    (   
        impl $typarams:tt $self_:ty 
        where $where_preds:tt
        {
            $((
                $getter_trait:ident< 
                    $field_name:tt : $field_ty:ty,
                    $name_param_ty:ty,
                    $name_param_str:expr,
                > 
            ))*
        }
    )=>{

        $crate::impl_structural!{
            impl $typarams Structural for $self_
            where $where_preds
            {
                field_names=[ 
                    $( (stringify!($field_name),$name_param_str), )* 
                ]
            }
        }

        $(
            $crate::impl_getter!{
                impl $typarams 
                    $getter_trait<$field_name : $field_ty,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
    }
}


/// Gets a type-level string value
///
/// When passed comma separated string literals,this instantiates a `MultiTString`,
/// which is what's passed to the `GetFieldExt::fields` methods.
///
/// # Example
///
/// ```
/// use structural::{GetFieldExt,tstr};
///
/// let tup=("I","you","they");
///
/// assert_eq!( tup.field_(tstr!("0")), &"I" );
/// assert_eq!( tup.field_(tstr!("1")), &"you" );
/// assert_eq!( tup.field_(tstr!("2")), &"they" );
///
/// assert_eq!( tup.fields(tstr!("0","1")), (&"I",&"you") );
///
/// assert_eq!( tup.fields(tstr!("0","1","2")), (&"I",&"you",&"they") );
///
/// ```
#[macro_export]
macro_rules! tstr {
    ( $($strings:literal),* $(,)* ) => {{
        mod dummy{
            structural_derive::tstr_impl!{$($strings),*}
        }
        dummy::VALUE
    }};
}

/// Gets a type-level string for use as a generic parameter.
///
/// # Future Compatibility
///
/// This macro will continue supporting space separated characters 
/// even after string literals are usable as trait parameters.
///
/// # Examples
///
/// This demonstrates how one can bound types by the accessor traits in a where clause.
///
/// ```rust
/// use structural::{GetField,GetFieldExt,tstr,TStr};
///
/// fn greet_entity<This,S>(entity:&This)
/// where
///     This:GetField<TStr!(n a m e),Ty=S>,
///     S:AsRef<str>,
/// {
///     println!("Hello, {}!",entity.field_(tstr!("name")).as_ref() );
/// }
///
/// ```
#[macro_export]
macro_rules! TStr {
    ($($char:tt)*) => {
        $crate::type_level::TString<($($crate::TChar!($char),)*)>
    };
}


/*

Code to generate the non-default branches

fn main() {
    for b in 0..=255u8 {
        let c=b as char;
        if c.is_alphanumeric() && b<128 || c=='_' {
            println!("({})=>( $crate::chars::_{} );",c,c)
        }
    }
}

*/

#[doc(hidden)]
#[macro_export]
macro_rules! TChar{
    (0)=>( $crate::chars::_0 );
    (1)=>( $crate::chars::_1 );
    (2)=>( $crate::chars::_2 );
    (3)=>( $crate::chars::_3 );
    (4)=>( $crate::chars::_4 );
    (5)=>( $crate::chars::_5 );
    (6)=>( $crate::chars::_6 );
    (7)=>( $crate::chars::_7 );
    (8)=>( $crate::chars::_8 );
    (9)=>( $crate::chars::_9 );
    (A)=>( $crate::chars::_A );
    (B)=>( $crate::chars::_B );
    (C)=>( $crate::chars::_C );
    (D)=>( $crate::chars::_D );
    (E)=>( $crate::chars::_E );
    (F)=>( $crate::chars::_F );
    (G)=>( $crate::chars::_G );
    (H)=>( $crate::chars::_H );
    (I)=>( $crate::chars::_I );
    (J)=>( $crate::chars::_J );
    (K)=>( $crate::chars::_K );
    (L)=>( $crate::chars::_L );
    (M)=>( $crate::chars::_M );
    (N)=>( $crate::chars::_N );
    (O)=>( $crate::chars::_O );
    (P)=>( $crate::chars::_P );
    (Q)=>( $crate::chars::_Q );
    (R)=>( $crate::chars::_R );
    (S)=>( $crate::chars::_S );
    (T)=>( $crate::chars::_T );
    (U)=>( $crate::chars::_U );
    (V)=>( $crate::chars::_V );
    (W)=>( $crate::chars::_W );
    (X)=>( $crate::chars::_X );
    (Y)=>( $crate::chars::_Y );
    (Z)=>( $crate::chars::_Z );
    (_)=>( $crate::chars::__ );
    (a)=>( $crate::chars::_a );
    (b)=>( $crate::chars::_b );
    (c)=>( $crate::chars::_c );
    (d)=>( $crate::chars::_d );
    (e)=>( $crate::chars::_e );
    (f)=>( $crate::chars::_f );
    (g)=>( $crate::chars::_g );
    (h)=>( $crate::chars::_h );
    (i)=>( $crate::chars::_i );
    (j)=>( $crate::chars::_j );
    (k)=>( $crate::chars::_k );
    (l)=>( $crate::chars::_l );
    (m)=>( $crate::chars::_m );
    (n)=>( $crate::chars::_n );
    (o)=>( $crate::chars::_o );
    (p)=>( $crate::chars::_p );
    (q)=>( $crate::chars::_q );
    (r)=>( $crate::chars::_r );
    (s)=>( $crate::chars::_s );
    (t)=>( $crate::chars::_t );
    (u)=>( $crate::chars::_u );
    (v)=>( $crate::chars::_v );
    (w)=>( $crate::chars::_w );
    (x)=>( $crate::chars::_x );
    (y)=>( $crate::chars::_y );
    (z)=>( $crate::chars::_z );
    ($byte:ident)=>{
        $crate::chars::$byte
    }
} 


/**

The `structural_alias` defines a trait alias for multiple field accessors.

# The entire syntax

```
# use structural::structural_alias;
# pub trait SuperTrait{}

structural_alias!{
    pub trait Foo<'a,T:Copy>:SuperTrait
    where
        T:SuperTrait
    {
             a:u32,
        ref  b:T,
        mut  c:i64,
        move d:String,
        mut move e:String,
    }
}

# fn main(){}
```

Outside of the `{...}` the trait syntax is the same as the 
regular one,with the same meaning.

Inside the `{...}` is a list of fields,
each of which get turned into supertraits on `Foo`:

- `     a:u32`:
    Corresponds to the `GetField<TString<(_a,)>,Ty=u32>` shared reference 
    field accessor trait.

- `ref  b:T`
    Corresponds to the `GetField<TString<(_b,)>,Ty=T>` shared reference 
    field accessor trait.

- `mut  c:i64`:
    Corresponds to the `GetFieldMut<TString<(_c,)>,Ty=i64>` mutable reference 
    field accessor trait (which`itself implies `GetField`).

- `move d:String`:
    Corresponds to the `IntoField<TString<(_d,)>,Ty=String>` by value
    field accessor trait (which itself implies `GetField`).

- `mut move e:String`:
    Corresponds to the `IntoFieldMut<TString<(_e,)>,Ty=String>` trait,
    allowing shared,mutable,and by value access to the field.

# Examples

### Defining a Point trait alias

```rust
use structural::{
    structural_alias,
    tstr,
    GetFieldExt,
    Structural,
};

use core::{
    cmp::PartialEq,
    fmt::{Debug,Display},
};

structural_alias!{
    trait Point<T>{
        mut move x:T,
        mut move y:T,
    }
}

fn print_point<T,U>(value:&T)
where
    T:Point<U>,
    U:Debug+Display+PartialEq,
{
    // This gets references to the `x` and `y` fields.
    let (x,y)=value.fields(tstr!("x","y"));
    assert_ne!(x,y);
    println!("x={} y={}",x,y);
}

#[derive(Structural)]
#[struc(access="mut move")]
struct Point3D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
}

#[derive(Structural)]
#[struc(access="mut move")]
struct Rectangle<T>{
    pub x:T,
    pub y:T,
    pub w:T,
    pub h:T,
}

#[derive(Structural)]
#[struc(access="mut move")]
struct Entity{
    pub id:PersonId,
    pub x:f32,
    pub y:f32,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);

# fn main(){

print_point(&Point3D{ x:100, y:200, z:6000 });

print_point(&Rectangle{ x:100, y:200, w:300, h:400 });

print_point(&Entity{ x:100.0, y:200.0, id:PersonId(0xDEAD) });


# }

```

### Defining a trait aliases with all accessibilities

```
use structural::{
    structural_alias,
    tstr,
    GetFieldExt,
};

structural_alias!{
    trait Person{
        // shared access (a & reference to the field)
        id:PersonId,
        
        // shared access (a & reference to the field)
        name:String,

        // mutable access (a &mut reference to the field),as well as shared access.
        mut friends:Vec<PersonId>,

        // by value access to the field (as well as shared)
        move candy:Candy,

        // by value access to the field (as well as shared and mutable)
        mut move snack:Snack,
    }
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Seconds(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Candy;

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Snack;

# fn main(){}

```


*/
#[macro_export]
macro_rules! structural_alias{
    ( $($everything:tt)* )=>{
        structural_derive::structural_alias_impl!{ $($everything)* }
    }
}