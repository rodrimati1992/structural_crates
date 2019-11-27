#[macro_use]
mod delegate_structural;

#[macro_use]
mod list;

#[macro_use]
mod ident;

#[macro_use]
mod make_struct;


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
        unsafe impl[$($typarams:tt)*]
            GetFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            impl[$($typarams)*] GetField<$field_name:$field_ty,$name_param> for $self_
            $( where[$($where_)*] )?
        }
    
        unsafe impl<$($typarams)*> $crate::GetFieldMut<$name_param> for $self_ 
        $( where $($where_)* )?
        {
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                &mut self.$field_name
            }

            $crate::z_unsafe_impl_get_field_raw_mut_method!{
                Self,
                field_name=$field_name,
                name_generic=$name_param
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
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
    ( 
        unsafe impl[$($typarams:tt)*]
            IntoFieldMut <$field_name:tt : $field_ty:ty,$name_param:ty> 
        for $self_:ty 
        $( where[$($where_:tt)*] )?
    )=>{
        $crate::impl_getter!{
            unsafe impl[$($typarams)*] 
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
            $crate::z_impl_box_into_field_method!{$name_param}
        }
    };
} 


macro_rules! default_if {
    ( 
        $(#[$attr:meta])*
        cfg($($cfg_attr:tt)*) 
        $($default_impl:tt)*
    ) => (
        #[cfg($($cfg_attr)*)]
        $(#[$attr])*
        default $($default_impl)*

        #[cfg(not($($cfg_attr)*))]
        $(#[$attr])*
        $($default_impl)*
    )
}



/// For manual implementors of the GetFieldMut trait,
/// implementing the methods used for accession multiple mutable fields.
///
/// # Safety
///
/// This is an unsafe macro,
/// because it requires each invocation of it to borrow a different field for the type
/// (the `field_name=` argument),
/// otherwise this would cause undefined behavior because it would 
/// create multiple mutable borrows to the same field.
///
/// # Example
///
/// For an example where this macro is used,
/// you can look at the
/// [manual implementation example of the GetFieldMut trait
/// ](./field_traits/trait.GetFieldMut.html#manual-implementation-example)
#[macro_export]
macro_rules! z_unsafe_impl_get_field_raw_mut_method {
    ( $Self:ident,field_name=$field_name:tt,name_generic=$name_param:ty ) => (
        unsafe fn get_field_raw_mut(
            this:*mut (),
            _:$crate::pmr::PhantomData<$name_param>,
        )->*mut $Self::Ty{
            &mut (*(this as *mut $Self)).$field_name as *mut $Self::Ty
        }

        fn get_field_raw_mut_func(
            &self
        )->$crate::field_traits::GetFieldMutRefFn<$name_param,$Self::Ty>{
            <$Self as $crate::field_traits::GetFieldMut<$name_param>>::get_field_raw_mut
        }
    )
}



/// For use in manual implementations of the IntoField trait.
/// 
/// Implements the `IntoField::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoField
/// ](./field_traits/trait.IntoField.html#manual-implementation-example)
#[macro_export]
#[cfg(not(feature="alloc"))]
macro_rules! z_impl_box_into_field_method {
    ($($anything:tt)*) => ()
}

/// For use in manual implementations of the IntoField trait.
/// 
/// Implements the `IntoField::box_into_field_` method,
/// automatically handling conditional `#![no_std]` support in `structural`.
///
/// For an example of using this macro look at
/// [the documentation for IntoField
/// ](./field_traits/trait.IntoField.html#manual-implementation-example)
#[macro_export]
#[cfg(feature="alloc")]
macro_rules! z_impl_box_into_field_method {
    ($field_name:ty) => (
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
            field_names=[$( 
                (
                    $field_name:tt : $field_ty:ty,
                    $name_param_ty:ty,
                    $name_param_str:expr,
                ),
            )*]
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
                            original_name:stringify!($field_name),
                            accessor_name:$name_param_str,
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



#[doc(hidden)]
#[macro_export]
macro_rules! z_impl_structural_dyn{
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
                    $( 
                        (
                            $field_name : $field_ty,
                            $name_param_ty,
                            $name_param_str,
                        ),
                    )* 
                ]
            }
        }

        $(
            $crate::impl_getter!{
                unsafe impl $typarams 
                    $getter_trait<$field_name : $field_ty,$name_param_ty>
                for $self_
                where $where_preds
            }
        )*
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
        # /*
        i:impl Bar,
        # */
    }

    pub trait Bar{
        x:u32,
        y:u32,
        z:u32,
    }
}

# fn main(){}
```

Outside of the `{...}` the trait syntax is the same as the 
regular one,with the same meaning.

Inside the `{...}` is a list of fields,
each of which get turned into supertraits on `Foo`:

- `     a:u32`:
    Corresponds to the `IntoFieldMut<FP!(a),Ty=u32>` trait,
    allowing shared,mutable,and by value access to the field.

- `ref  b:T`:
    Corresponds to the `GetField<FP!(b),Ty=T>` shared reference 
    field accessor trait.

- `mut  c:i64`:
    Corresponds to the `GetFieldMut<FP!(c),Ty=i64>` mutable reference 
    field accessor trait (which`itself implies `GetField`).

- `move d:String`:
    Corresponds to the `IntoField<FP!(d),Ty=String>` by value
    field accessor trait (which itself implies `GetField`).

- `mut move e:String`:
    Corresponds to the `IntoFieldMut<FP!(e),Ty=String>` trait,
    allowing shared,mutable,and by value access to the field.

- `i:impl Bar`:
    Corresponds to the `IntoFieldMut<FP!(i),Ty:Bar>` trait,
    allowing shared,mutable,and by value access to 
    a field that implements the Bar trait.<br>
    This requires the `nightly_impl_fields` or `impl_fields` cargo feature.

# Supertraits

### Structural aliases as supertraits

Structural aliases are regular traits,
so you can use them as supertraits in your own traits.

```
use structural::{GetFieldExt,structural_alias,fp};

structural_alias!{
    trait Fields{
        ref foo:usize,
        ref bar:String,
    }
}

trait MyTrait:Fields{
    fn multiply_foo(&self,n:usize)->usize{
        n * self.field_(fp!(foo))
    }
    fn print_bar(&self){
        println!("{}", self.field_(fp!(bar)) );
    }
}


# fn main(){}

```

### Same field names

Structural aliases can have other structural aliases as supertraits,
even ones with the same fields.

In this example:

```rust
use structural::structural_alias;

structural_alias!{
    trait Point<T>{
        move x:T,
        move y:T,
    }

    trait Rectangle<T>:Point<T>{
        ref x:T,
        ref y:T,
        ref w:T,
        ref h:T,
    }
}

# fn main(){}
```
It is legal to repeat the `x` and `y` fields in subtraits,
and those fields get the most permissive access specified,
which here is shared and by value access to both `x` and `y`.


<br>

It is not legal is to redeclare the field with an incompatible type:

```compile_fail
use structural::structural_alias;

structural_alias!{
    trait Point<T>{
        x:T,
        y:T,
    }

    trait Rectangle<T>:Point<T>{
        x:usize,
        y:T,
        w:T,
        h:T,
    }
}

# fn main(){}
```


# impl Trait fields

This requires the `nightly_impl_fields` cargo feature
(or `impl_fields` if associated type bounds stabilized after the latest release).

You can declare a field with `impl Bar` as its type to declare that the field 
implements Bar,without specifying a particular type.

Using `impl Trait` fields makes a `Foo` structural alias unusable as a `dyn Foo`.

### Example

This demonstrates using impl trait fields.

*/
#[cfg_attr(not(feature="nightly_impl_fields"),doc="```ignore")]
#[cfg_attr(feature="nightly_impl_fields",doc="```rust")]
/**
// Remove this if associated type bounds (eg: `T: Iterator<Item: Debug>`) 
// work without it.
#![feature(associated_type_bounds)]

use structural::{structural_alias,fp,make_struct,GetFieldExt};

structural_alias!{
    trait Foo{
        foo:impl Bar,
    }

    trait Bar{
        dimension:impl Dim<u32>
    }

    trait Dim<T>{
        width:T,
        height:T,
    }
}

fn with_foo(this:&impl Foo){
    let dim=this.field_(fp!(foo.dimension));
    assert_eq!( dim.field_(fp!(width)), &200 );
    assert_eq!( dim.field_(fp!(height)), &201 );
}


fn main(){
    with_foo(&make_struct!{
        foo:make_struct!{
            dimension:make_struct!{
                width:200,
                height:201,
            }
        }
    });
}


```

# Examples

### Defining a Point trait alias

```rust
use structural::{structural_alias,fp,GetFieldExt,Structural};

use core::{
    cmp::PartialEq,
    fmt::{Debug,Display},
};


structural_alias!{
    trait Point<T>{
        // Using `ref` because we just want to read the fields
        ref x:T,
        ref y:T,
    }
}

fn print_point<T,U>(value:&T)
where
    T:Point<U>,
    U:Debug+Display+PartialEq,
{
    // This gets references to the `x` and `y` fields.
    let (x,y)=value.fields(fp!(x,y));
    assert_ne!(x,y);
    println!("x={} y={}",x,y);
}

fn main(){

    print_point(&Point3D{ x:100, y:200, z:6000 });

    print_point(&Rectangle{ x:100, y:200, w:300, h:400 });

    print_point(&Entity{ x:100.0, y:200.0, id:PersonId(0xDEAD) });

}



#[derive(Structural)]
struct Point3D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
}

#[derive(Structural)]
struct Rectangle<T>{
    pub x:T,
    pub y:T,
    pub w:T,
    pub h:T,
}

#[derive(Structural)]
struct Entity{
    pub id:PersonId,
    pub x:f32,
    pub y:f32,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);


```

### Defining a trait aliases with all accessibilities

```
use structural::{
    structural_alias,
    fp,
    GetFieldExt,
};

structural_alias!{
    trait Person{
        // shared,mutable,and by value access to the field)
        id:PersonId,
        
        // shared access (a & reference to the field)
        ref name:String,

        // mutable access (a &mut reference to the field),as well as shared access.
        mut friends:Vec<PersonId>,

        // by value access to the field (as well as shared)
        move candy:Candy,

        // shared,mutable,and by value access to the field)
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
        $crate::structural_alias_impl!{ $($everything)* }
    }
}
