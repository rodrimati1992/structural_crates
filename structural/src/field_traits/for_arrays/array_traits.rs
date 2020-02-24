#![allow(non_camel_case_types)]
/*!
Structural aliases for even array size up to 32.
*/

use super::names::{
    I0, I1, I10, I11, I12, I13, I14, I15, I16, I17, I18, I19, I2, I20, I21, I22, I23, I24, I25,
    I26, I27, I28, I29, I3, I30, I31, I4, I5, I6, I7, I8, I9,
};
use crate::field_traits::{IntoFieldMut, IntoVariantFieldMut};

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
            $trait_name:ident
            [$($super_trait:ident)*]
            [$($field:ident)*]
        ))*
    ) => (
        $(
            /// A structural alias for an array of this size
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
        )*
    )
}

declare_array_traits! {
    (Array_0_8 [] [I0 I1 I2 I3 I4 I5 I6 I7 ] )
    (Array_8_16 [Array_0_8 ] [I8 I9 I10 I11 I12 I13 I14 I15 ] )
    (Array_16_24 [Array_8_16 ] [I16 I17 I18 I19 I20 I21 I22 I23 ] )
    (Array0 [] [] )
    (Array1 [] [I0 ] )
    (Array2 [] [I0 I1 ] )
    (Array3 [] [I0 I1 I2 ] )
    (Array4 [] [I0 I1 I2 I3 ] )
    (Array5 [] [I0 I1 I2 I3 I4 ] )
    (Array6 [] [I0 I1 I2 I3 I4 I5 ] )
    (Array7 [] [I0 I1 I2 I3 I4 I5 I6 ] )
    (Array8 [Array_0_8 ] [] )
    (Array9 [Array_0_8 ] [I8 ] )
    (Array10 [Array_0_8 ] [I8 I9 ] )
    (Array11 [Array_0_8 ] [I8 I9 I10 ] )
    (Array12 [Array_0_8 ] [I8 I9 I10 I11 ] )
    (Array13 [Array_0_8 ] [I8 I9 I10 I11 I12 ] )
    (Array14 [Array_0_8 ] [I8 I9 I10 I11 I12 I13 ] )
    (Array15 [Array_0_8 ] [I8 I9 I10 I11 I12 I13 I14 ] )
    (Array16 [Array_8_16 ] [] )
    (Array17 [Array_8_16 ] [I16 ] )
    (Array18 [Array_8_16 ] [I16 I17 ] )
    (Array19 [Array_8_16 ] [I16 I17 I18 ] )
    (Array20 [Array_8_16 ] [I16 I17 I18 I19 ] )
    (Array21 [Array_8_16 ] [I16 I17 I18 I19 I20 ] )
    (Array22 [Array_8_16 ] [I16 I17 I18 I19 I20 I21 ] )
    (Array23 [Array_8_16 ] [I16 I17 I18 I19 I20 I21 I22 ] )
    (Array24 [Array_16_24 ] [] )
    (Array25 [Array_16_24 ] [I24 ] )
    (Array26 [Array_16_24 ] [I24 I25 ] )
    (Array27 [Array_16_24 ] [I24 I25 I26 ] )
    (Array28 [Array_16_24 ] [I24 I25 I26 I27 ] )
    (Array29 [Array_16_24 ] [I24 I25 I26 I27 I28 ] )
    (Array30 [Array_16_24 ] [I24 I25 I26 I27 I28 I29 ] )
    (Array31 [Array_16_24 ] [I24 I25 I26 I27 I28 I29 I30 ] )
    (Array32 [Array_16_24 ] [I24 I25 I26 I27 I28 I29 I30 I31 ] )

}

macro_rules! declare_array_traits {
    (
        $((
            $variant_trait_name:ident
            [$($super_trait:ident)*]
            [$($field:ident)*]
        ))*
    ) => (
        $(
            /// A structural alias for an array newtype variant
            ///
            /// The `V` generic parameter is the name of the variant.
            /// Example of the `V` parameter for a variant named `Foo`:
            /// - (since Rust 1.40): `TS!(Foo)`  <br>
            /// - (before Rust 1.40):`TS!(F o o)`<br>
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
    (ArrayVariant_0_8 [] [I0 I1 I2 I3 I4 I5 I6 I7 ] )
    (ArrayVariant_8_16 [ArrayVariant_0_8 ] [I8 I9 I10 I11 I12 I13 I14 I15 ] )
    (ArrayVariant_16_24 [ArrayVariant_8_16 ] [I16 I17 I18 I19 I20 I21 I22 I23 ] )
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
mod tests {
    use crate::field_traits::for_arrays::*;
    use crate::{GetFieldExt, Structural, TS};

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
        A: Array32Variant<u32, TS!(F o o)> + Clone,
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
