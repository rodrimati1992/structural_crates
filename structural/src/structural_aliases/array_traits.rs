/*!
Traits (including structural aliases) for arrays.

# `Array*` traits

The `Array*` traits alias the accessor traits for arrays,
with shared,mutable,and by value access to every element of the array.

These traits can be used with any array at least as large as the size indicated
by the trait.<br>
You can,for example,use `Array3` with any array type from `[T;3]` to `[T;32]` inclusive.


### Homogeneous tuples

You can pass homogeneous tuples to functions expecting `Array*` implementing types.


```
use structural::structural_aliases::array_traits::Array4;
use structural::{StructuralExt,fp};

fn takes_array(array:impl Array4<u32>){
    assert_eq!( array.field_(fp!(0)), &3 );
    assert_eq!( array.field_(fp!(1)), &5 );
    assert_eq!( array.field_(fp!(2)), &8 );
    assert_eq!( array.field_(fp!(3)), &13 );
}

takes_array( (3,5,8,13) );

// Tuples only have to be homogeneous up to the size of the expected array.
takes_array( (3,5,8,13,"foo") );
takes_array( (3,5,8,13,"foo",vec!["bar"]) );

```

# `Array*` Example

```
use structural::structural_aliases::array_traits::Array3;
use structural::{StructuralExt,fp};

use std::fmt::Debug;

fn print_first_3<T>(array:impl Array3<T>)
where
    T:Debug,
{
    println!("{:?}",array.fields(fp!(0,1,2)))
}

print_first_3( [3,5,8] );
print_first_3( [3,5,8,13] );
print_first_3( [3,5,8,13,21]);
print_first_3( [3,5,8,13,21,34] );
print_first_3( ["foo";7] );
print_first_3( ["bar";31] );
print_first_3( ["baz";32] );



```

# `Array*Variant` Example

Demonstrates that you can use the `Array*Variant` trait with enums.

```
use structural::structural_aliases::Array2Variant;
use structural::{StructuralExt,Structural,TS,fp};

use std::fmt::Debug;

fn first_2<T>(mut foo:T, mut not_foo:T)
where
    T: Array2Variant<u8,TS!(Foo)> + Copy
{
    {
        assert_eq!( foo.fields(fp!(::Foo=>0,1)), Some(( &101, &202 )) );

        assert_eq!( foo.fields_mut(fp!(::Foo=>0,1)), Some(( &mut 101, &mut 202 )) );

        assert_eq!( foo.into_field(fp!(::Foo.0)), Some(101) );
        assert_eq!( foo.into_field(fp!(::Foo.1)), Some(202) );

        assert_eq!( foo.is_variant(fp!(Foo)), true );
    }
    {
        assert_eq!( not_foo.fields(fp!(::Foo=>0,1)), None );

        assert_eq!( not_foo.fields_mut(fp!(::Foo=>0,1)), None );

        assert_eq!( not_foo.into_field(fp!(::Foo.0)), None );
        assert_eq!( not_foo.into_field(fp!(::Foo.1)), None );

        assert_eq!( not_foo.is_variant(fp!(Foo)), false );
    }
}

first_2( Enum::Foo(101,202), Enum::Bar );
first_2( OtherEnum::Foo(101,202,"303"), OtherEnum::Bar );

#[derive(Structural,Copy,Clone)]
# #[struc(no_trait)]
enum Enum{
    Foo(u8,u8),
    Bar,
}

#[derive(Structural,Copy,Clone)]
# #[struc(no_trait)]
enum OtherEnum{
    Foo(u8,u8,&'static str),
    Bar,
}

```





*/


#![allow(non_camel_case_types)]

use crate::{
    field::{IntoField, IntoFieldMut, IntoVariantFieldMut},
    path::array_paths::{
        I0, I1, I10, I11, I12, I13, I14, I15, I16, I17, I18, I19, I2, I20, I21, I22, I23, I24, I25,
        I26, I27, I28, I29, I3, I30, I31, I4, I5, I6, I7, I8, I9,
    },
};

/*


fn main() {
    use std::fmt::Write;

    const S:&'static str="    ";

    let mut reg=String::new();

    const MAX:usize=32;
    const CHUNK_SIZE:usize=8;
    const CHUNK_COUNT:usize=MAX/CHUNK_SIZE;

    println!("declare_array_traits!{{");
    for i in 0..CHUNK_COUNT {
        let prev=i.saturating_sub(1)*8;
        let curr=i*8;
        let next=(i+1)*8;
        let next=if next==MAX { MAX+1 }else{ next };

        if next<MAX {
            print!("{S}(Array_{0}_{1} [",curr,next,S=S);
            if i!=0 {
                print!("Array_{}_{} ",prev,curr);
            }
            print!("] [");
            for field in curr..next {
                print!("I{0} ",field);
            }
            println!("] )");
        }
        for arr_len in curr..next {
            let _=write!(reg,"{S}(Array{0} [",arr_len,S=S);
            if i!=0 {
                let _=write!(reg,"Array_{}_{} ",prev,curr);
            }
            let _=write!(reg,"] [");
            for field in curr..arr_len {
                let _=write!(reg,"I{0} ",field);
            }
            let _=writeln!(reg,"] )");
        }
    }
    println!("{}",reg);
    println!("}}");
}



*/

macro_rules! declare_array_traits {
    (
        $((
            $(#[$meta:meta])*
            $trait_name:ident
            $trait_name_move:ident
            [$($super_trait:ident)*]
            [$($super_trait_move:ident)*]
            [$($field:ident)*]
        ))*
    ) => (
        $(
            /// A structural alias for an array of this size.
            /// With shared,mutable,and by value access to the elements.
            $(#[$meta])*
            pub trait $trait_name<T>:
                $($super_trait<T>+)*
                $(IntoFieldMut<$field,Ty=T> +)*
            {}

            impl<This,T>  $trait_name<T> for This
            where
                This:
                    ?Sized+
                    $($super_trait<T>+)*
                    $(IntoFieldMut<$field,Ty=T> +)*
            {}

            /// A structural alias for an array of this size.
            /// With shared and by value access to the elements.
            $(#[$meta])*
            pub trait $trait_name_move<T>:
                $($super_trait_move<T>+)*
                $(IntoField<$field,Ty=T> +)*
            {}

            impl<This,T>  $trait_name_move<T> for This
            where
                This:
                    ?Sized+
                    $($super_trait_move<T>+)*
                    $(IntoField<$field,Ty=T> +)*
            {}
        )*
    )
}

declare_array_traits! {
    (
        #[doc(hidden)]
        Array_0_8 ArrayMove_0_8
        [] []
        [I0 I1 I2 I3 I4 I5 I6 I7 ]
    )
    (
        #[doc(hidden)]
        Array_8_16 ArrayMove_8_16
        [Array_0_8] [ArrayMove_0_8]
        [I8 I9 I10 I11 I12 I13 I14 I15 ]
    )
    (
        #[doc(hidden)]
        Array_16_24 ArrayMove_16_24
        [Array_8_16] [ArrayMove_8_16]
        [I16 I17 I18 I19 I20 I21 I22 I23 ]
    )
    (Array0  ArrayMove0  [] [] [] )
    (Array1  ArrayMove1  [] [] [I0 ] )
    (Array2  ArrayMove2  [] [] [I0 I1 ] )
    (Array3  ArrayMove3  [] [] [I0 I1 I2 ] )
    (Array4  ArrayMove4  [] [] [I0 I1 I2 I3 ] )
    (Array5  ArrayMove5  [] [] [I0 I1 I2 I3 I4 ] )
    (Array6  ArrayMove6  [] [] [I0 I1 I2 I3 I4 I5 ] )
    (Array7  ArrayMove7  [] [] [I0 I1 I2 I3 I4 I5 I6 ] )
    (Array8  ArrayMove8  [Array_0_8] [ArrayMove_0_8 ] [] )
    (Array9  ArrayMove9  [Array_0_8] [ArrayMove_0_8 ] [I8 ] )
    (Array10 ArrayMove10 [Array_0_8] [ArrayMove_0_8 ] [I8 I9 ] )
    (Array11 ArrayMove11 [Array_0_8] [ArrayMove_0_8 ] [I8 I9 I10 ] )
    (Array12 ArrayMove12 [Array_0_8] [ArrayMove_0_8 ] [I8 I9 I10 I11 ] )
    (Array13 ArrayMove13 [Array_0_8] [ArrayMove_0_8 ] [I8 I9 I10 I11 I12 ] )
    (Array14 ArrayMove14 [Array_0_8] [ArrayMove_0_8 ] [I8 I9 I10 I11 I12 I13 ] )
    (Array15 ArrayMove15 [Array_0_8] [ArrayMove_0_8 ] [I8 I9 I10 I11 I12 I13 I14 ] )
    (Array16 ArrayMove16 [Array_8_16] [ArrayMove_8_16 ] [] )
    (Array17 ArrayMove17 [Array_8_16] [ArrayMove_8_16 ] [I16 ] )
    (Array18 ArrayMove18 [Array_8_16] [ArrayMove_8_16 ] [I16 I17 ] )
    (Array19 ArrayMove19 [Array_8_16] [ArrayMove_8_16 ] [I16 I17 I18 ] )
    (Array20 ArrayMove20 [Array_8_16] [ArrayMove_8_16 ] [I16 I17 I18 I19 ] )
    (Array21 ArrayMove21 [Array_8_16] [ArrayMove_8_16 ] [I16 I17 I18 I19 I20 ] )
    (Array22 ArrayMove22 [Array_8_16] [ArrayMove_8_16 ] [I16 I17 I18 I19 I20 I21 ] )
    (Array23 ArrayMove23 [Array_8_16] [ArrayMove_8_16 ] [I16 I17 I18 I19 I20 I21 I22 ] )
    (Array24 ArrayMove24 [Array_16_24] [ArrayMove_16_24 ] [] )
    (Array25 ArrayMove25 [Array_16_24] [ArrayMove_16_24 ] [I24 ] )
    (Array26 ArrayMove26 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 ] )
    (Array27 ArrayMove27 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 I26 ] )
    (Array28 ArrayMove28 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 I26 I27 ] )
    (Array29 ArrayMove29 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 I26 I27 I28 ] )
    (Array30 ArrayMove30 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 I26 I27 I28 I29 ] )
    (Array31 ArrayMove31 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 I26 I27 I28 I29 I30 ] )
    (Array32 ArrayMove32 [Array_16_24] [ArrayMove_16_24 ] [I24 I25 I26 I27 I28 I29 I30 I31 ] )

}

macro_rules! declare_array_traits {
    (
        $((
            $(#[$meta:meta])*
            $variant_trait_name:ident
            [$($super_trait:ident)*]
            [$($field:ident)*]
        ))*
    ) => (
        $(
            /// A structural alias for a tuple variant that's homogeneous
            /// up to the size of the array).
            ///
            /// The `V` generic parameter is the name of the variant.
            /// Examples of the `V` parameter for a variant named `Foo`:
            /// - `TS!(Foo)`<br>
            /// - `TS!(Bar)`<br>
            /// - `TS!(0)`<br>
            $(#[$meta])*
            pub trait $variant_trait_name<T,V>:
                $($super_trait<T,V>+)*
                $(IntoVariantFieldMut<V,$field,Ty=T> +)*
            {}

            impl<This,V,T>  $variant_trait_name<T,V> for This
            where
                This:
                    ?Sized+
                    $($super_trait<T,V>+)*
                    $(IntoVariantFieldMut<V,$field,Ty=T> +)*
            {}
        )*
    )
}

declare_array_traits! {
    (#[doc(hidden)] ArrayVariant_0_8 [] [I0 I1 I2 I3 I4 I5 I6 I7 ] )
    (#[doc(hidden)] ArrayVariant_8_16 [ArrayVariant_0_8 ] [I8 I9 I10 I11 I12 I13 I14 I15 ] )
    (#[doc(hidden)] ArrayVariant_16_24 [ArrayVariant_8_16 ] [I16 I17 I18 I19 I20 I21 I22 I23 ] )
    (Array0Variant [] [] )
    (Array1Variant [] [I0 ] )
    (Array2Variant [] [I0 I1 ] )
    (Array3Variant [] [I0 I1 I2 ] )
    (Array4Variant [] [I0 I1 I2 I3 ] )
    (Array5Variant [] [I0 I1 I2 I3 I4 ] )
    (Array6Variant [] [I0 I1 I2 I3 I4 I5 ] )
    (Array7Variant [] [I0 I1 I2 I3 I4 I5 I6 ] )
    (Array8Variant [ArrayVariant_0_8 ] [] )
    (Array9Variant [ArrayVariant_0_8 ] [I8 ] )
    (Array10Variant [ArrayVariant_0_8 ] [I8 I9 ] )
    (Array11Variant [ArrayVariant_0_8 ] [I8 I9 I10 ] )
    (Array12Variant [ArrayVariant_0_8 ] [I8 I9 I10 I11 ] )
    (Array13Variant [ArrayVariant_0_8 ] [I8 I9 I10 I11 I12 ] )
    (Array14Variant [ArrayVariant_0_8 ] [I8 I9 I10 I11 I12 I13 ] )
    (Array15Variant [ArrayVariant_0_8 ] [I8 I9 I10 I11 I12 I13 I14 ] )
    (Array16Variant [ArrayVariant_8_16 ] [] )
    (Array17Variant [ArrayVariant_8_16 ] [I16 ] )
    (Array18Variant [ArrayVariant_8_16 ] [I16 I17 ] )
    (Array19Variant [ArrayVariant_8_16 ] [I16 I17 I18 ] )
    (Array20Variant [ArrayVariant_8_16 ] [I16 I17 I18 I19 ] )
    (Array21Variant [ArrayVariant_8_16 ] [I16 I17 I18 I19 I20 ] )
    (Array22Variant [ArrayVariant_8_16 ] [I16 I17 I18 I19 I20 I21 ] )
    (Array23Variant [ArrayVariant_8_16 ] [I16 I17 I18 I19 I20 I21 I22 ] )
    (Array24Variant [ArrayVariant_16_24 ] [] )
    (Array25Variant [ArrayVariant_16_24 ] [I24 ] )
    (Array26Variant [ArrayVariant_16_24 ] [I24 I25 ] )
    (Array27Variant [ArrayVariant_16_24 ] [I24 I25 I26 ] )
    (Array28Variant [ArrayVariant_16_24 ] [I24 I25 I26 I27 ] )
    (Array29Variant [ArrayVariant_16_24 ] [I24 I25 I26 I27 I28 ] )
    (Array30Variant [ArrayVariant_16_24 ] [I24 I25 I26 I27 I28 I29 ] )
    (Array31Variant [ArrayVariant_16_24 ] [I24 I25 I26 I27 I28 I29 I30 ] )
    (Array32Variant [ArrayVariant_16_24 ] [I24 I25 I26 I27 I28 I29 I30 I31 ] )
}

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use super::{Array32, Array32Variant};
    use crate::{Structural, StructuralExt, TS};

    fn with_array_32<A>(mut this: A)
    where
        A: Array32<u32> + Clone,
    {
        assert_eq!(
            this.fields(fp!(0, 1, 10, 11, 20, 21, 30, 31)),
            (&0, &10, &100, &110, &200, &210, &300, &310),
        );
        assert_eq!(
            this.fields_mut(fp!(0, 1, 10, 11, 20, 21, 30, 31)),
            (&mut 0, &mut 10, &mut 100, &mut 110, &mut 200, &mut 210, &mut 300, &mut 310),
        );
        assert_eq!(this.clone().into_field(fp!(0)), 0);
        assert_eq!(this.clone().into_field(fp!(1)), 10);
        assert_eq!(this.clone().into_field(fp!(10)), 100);
        assert_eq!(this.clone().into_field(fp!(11)), 110);
        assert_eq!(this.clone().into_field(fp!(20)), 200);
        assert_eq!(this.clone().into_field(fp!(21)), 210);
        assert_eq!(this.clone().into_field(fp!(30)), 300);
        assert_eq!(this.clone().into_field(fp!(31)), 310);
    }
    fn with_array_32_variant<A>(this: A)
    where
        A: Array32Variant<u32, TS!(Foo)> + Clone,
    {
        with_array_32(this.into_field(fp!(::Foo)).unwrap());
    }

    #[test]
    fn array_32_test() {
        let mut arr = [0u32; 32];
        for i in 0..32u32 {
            arr[i as usize] = i * 10;
        }

        with_array_32(arr);
        with_array_32_variant(Enum::Foo(arr));
    }

    #[derive(Structural, Clone)]
    enum Enum {
        #[struc(newtype)]
        Foo([u32; 32]),
    }
}
