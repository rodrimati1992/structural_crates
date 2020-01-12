/*!
Traits related to arrays.

# `Array*` traits

The `Array*` traits alias the accessor traits for arrays,
with shared,mutable,and by value access to every element of the array.

These traits can be used with any array at least as large as the size indicated
by the trait.<br>
You can,for example,use `Array3` with any array type from `[T;3]` to `[T;32]` inclusive.


### Homogeneous tuples

You can pass homogeneous tuples to functions expecting `Array*` implementing types.


```
use structural::field_traits::for_arrays::Array4;
use structural::{GetFieldExt,fp};

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
use structural::field_traits::for_arrays::Array3;
use structural::{GetFieldExt,fp};

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



*/

use crate::{
    field_traits::{
        FieldType, GetFieldImpl, GetFieldMutImpl, GetFieldRawMutFn, IntoFieldImpl, NonOptField,
    },
    structural_trait::{FieldInfo, FieldInfos, Structural},
    type_level::{
        cmp::{Compare_, TGreater},
        integer::*,
        IsFieldPath,
    },
};

use std_::{marker::PhantomData, mem::ManuallyDrop, ptr};

////////////////////////////////////////////////////////////////////////////////

pub mod array_traits;

pub use self::array_traits::{
    Array0, Array1, Array10, Array11, Array12, Array13, Array14, Array15, Array16, Array17,
    Array18, Array19, Array2, Array20, Array21, Array22, Array23, Array24, Array25, Array26,
    Array27, Array28, Array29, Array3, Array30, Array31, Array32, Array4, Array5, Array6, Array7,
    Array8, Array9,
};

////////////////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/// A FieldPath that is usable for indexing (some) arrays.
pub trait ArrayPath: IsFieldPath + Sealed {
    const INDEX: usize;

    type Index: IsUnsigned;
}

/// Used to check whether the index associated with this `ArrayPath` is valid for `Array`.
pub trait IsPathForArray<Array>: ArrayPath {
    #[doc(hidden)]
    const _SEALED_IPFA: SealedIPFA<Self, Array>;
}

#[doc(hidden)]
pub struct SealedIPFA<Path, Array> {
    _marker: PhantomData<(Path, Array)>,
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! declare_array_paths {
    (
        $(
            (
                $index_name:ident = $index:expr,$tnum:ident,$fi_ind:ident,
                [$($fi_in_array:ident,)*]
            )
        )*
    ) => (
        field_path_aliases!{
            pub(crate) mod names{
                $( $index_name = $index ,)*
            }
        }
        use self::names::*;

        $(
            #[allow(dead_code)]
            const $fi_ind:FieldInfo=FieldInfo::not_renamed(stringify!($index));

            impl<T> crate::IsStructural for [T;$index] {}

            impl<T> Structural for [T;$index]{
                const FIELDS:&'static FieldInfos=&FieldInfos::Struct(&[
                    $( $fi_in_array, )*
                ]);
            }


            impl Sealed for $index_name{}

            impl ArrayPath for $index_name{
                const INDEX:usize=$index;
                type Index=$tnum;
            }

            impl<T,P> IsPathForArray<[T;$index]> for P
            where
                P:ArrayPath,
                $tnum:Compare_<P::Index,Output=TGreater>,
            {
                #[doc(hidden)]
                const _SEALED_IPFA:SealedIPFA<P,[T;$index]>=SealedIPFA{_marker:PhantomData};
            }

            impl<T,P> FieldType<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                type Ty=T;
            }

            impl<T,P> GetFieldImpl<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                type Err=NonOptField;

                #[inline(always)]
                fn get_field_(&self,_:P,_:())->Result<&Self::Ty,NonOptField>{
                    Ok(&self[P::INDEX])
                }
            }

            unsafe impl<T,P> GetFieldMutImpl<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                #[inline(always)]
                fn get_field_mut_(&mut self,_:P,_:())->Result<&mut Self::Ty,NonOptField>{
                    Ok(&mut self[P::INDEX])
                }

                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    ptr:*mut (),
                    _:P,
                    _:(),
                )->Result<*mut Self::Ty,NonOptField>{
                    Ok((ptr as *mut T).add(P::INDEX))
                }

                #[inline(always)]
                fn get_field_raw_mut_func(&self)->GetFieldRawMutFn<P,(),Self::Ty,NonOptField>{
                    <Self as GetFieldMutImpl<P>>::get_field_raw_mut
                }
            }

            impl<T,P> IntoFieldImpl<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                #[inline(always)]
                fn into_field_(self,_:P,_:())->Result<Self::Ty,NonOptField>{
                    unsafe{
                        let mut this=ManuallyDrop::new(self);
                        ptr::drop_in_place(&mut this[..P::INDEX]);
                        ptr::drop_in_place(&mut this[P::INDEX+1..]);
                        Ok(this.as_mut_ptr().add(P::INDEX).read())
                    }
                }

                z_impl_box_into_field_method!{ P }
            }
        )*
    )
}

/*
Generated with:

fn main() {
    for i in 0..=32{
        print!("(I{0}={0},U{0},FI_{0},[",i);
        for j in 0..i {
            if j%10==0 {
                print!("\n    ");
            }
            print!("FI_{0},",j);
        }
        println!("\n])");
    }
}

*/

declare_array_paths! {
    (I0=0,U0,FI_0,[
    ])
    (I1=1,U1,FI_1,[
        FI_0,
    ])
    (I2=2,U2,FI_2,[
        FI_0,FI_1,
    ])
    (I3=3,U3,FI_3,[
        FI_0,FI_1,FI_2,
    ])
    (I4=4,U4,FI_4,[
        FI_0,FI_1,FI_2,FI_3,
    ])
    (I5=5,U5,FI_5,[
        FI_0,FI_1,FI_2,FI_3,FI_4,
    ])
    (I6=6,U6,FI_6,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,
    ])
    (I7=7,U7,FI_7,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,
    ])
    (I8=8,U8,FI_8,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,
    ])
    (I9=9,U9,FI_9,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,
    ])
    (I10=10,U10,FI_10,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
    ])
    (I11=11,U11,FI_11,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,
    ])
    (I12=12,U12,FI_12,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,
    ])
    (I13=13,U13,FI_13,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,
    ])
    (I14=14,U14,FI_14,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,
    ])
    (I15=15,U15,FI_15,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,
    ])
    (I16=16,U16,FI_16,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,
    ])
    (I17=17,U17,FI_17,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,
    ])
    (I18=18,U18,FI_18,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,
    ])
    (I19=19,U19,FI_19,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,
    ])
    (I20=20,U20,FI_20,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
    ])
    (I21=21,U21,FI_21,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,
    ])
    (I22=22,U22,FI_22,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,
    ])
    (I23=23,U23,FI_23,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,
    ])
    (I24=24,U24,FI_24,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,
    ])
    (I25=25,U25,FI_25,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,
    ])
    (I26=26,U26,FI_26,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,
    ])
    (I27=27,U27,FI_27,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,FI_26,
    ])
    (I28=28,U28,FI_28,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,FI_26,FI_27,
    ])
    (I29=29,U29,FI_29,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,FI_26,FI_27,FI_28,
    ])
    (I30=30,U30,FI_30,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,FI_26,FI_27,FI_28,FI_29,
    ])
    (I31=31,U31,FI_31,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,FI_26,FI_27,FI_28,FI_29,
        FI_30,
    ])
    (I32=32,U32,FI_32,[
        FI_0,FI_1,FI_2,FI_3,FI_4,FI_5,FI_6,FI_7,FI_8,FI_9,
        FI_10,FI_11,FI_12,FI_13,FI_14,FI_15,FI_16,FI_17,FI_18,FI_19,
        FI_20,FI_21,FI_22,FI_23,FI_24,FI_25,FI_26,FI_27,FI_28,FI_29,
        FI_30,FI_31,
    ])

}

///
///
/// ```compile_fail
/// use structural::{GetFieldExt,fp};
///
/// let _=[0;31].field_(fp!(31));
///
/// ```
///
/// ```rust
/// use structural::{GetFieldExt,fp};
///
/// let _=[0;31].field_(fp!(30));
///
/// ```
///
#[cfg(feature = "testing")]
#[allow(dead_code)]
struct Foo0;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::GetFieldExt;

    use std_::convert::TryFrom;
    use std_::mem;

    #[test]
    fn get_field() {
        assert_eq!([5].field_(fp!(0)), &5);

        {
            let arr = [5, 8];
            assert_eq!(arr.field_(fp!(0)), &5);
            assert_eq!(arr.field_(fp!(1)), &8);
        }

        {
            let arr = [5, 8, 13];
            assert_eq!(arr.field_(fp!(0)), &5);
            assert_eq!(arr.field_(fp!(1)), &8);
            assert_eq!(arr.field_(fp!(2)), &13);
        }
    }

    #[test]
    fn get_field_mut() {
        assert_eq!([5].field_mut(fp!(0)), &mut 5);

        {
            let mut arr = [5, 8];
            assert_eq!(arr.field_mut(fp!(0)), &mut 5);
            assert_eq!(arr.field_mut(fp!(1)), &mut 8);
        }

        {
            let mut arr = [5, 8, 13];
            assert_eq!(arr.field_mut(fp!(0)), &mut 5);
            assert_eq!(arr.field_mut(fp!(1)), &mut 8);
            assert_eq!(arr.field_mut(fp!(2)), &mut 13);
        }

        assert_eq!([5].field_mut(fp!(0)), &mut 5);
        assert_eq!([5, 8].fields_mut(fp!(0, 1)), (&mut 5, &mut 8));
        assert_eq!(
            [5, 8, 13].fields_mut(fp!(0, 1, 2)),
            (&mut 5, &mut 8, &mut 13)
        );
        {
            let mut array = [5, 8, 13];
            let (e0, e1, e2) = array.fields_mut(fp!(0, 1, 2));
            mem::swap(e0, e1);
            mem::swap(e0, e2);
            mem::swap(e1, e2);
            assert_eq!(array, [13, 8, 5]);
        }
    }

    #[test]
    fn into_field() {
        assert_eq!([5].into_field(fp!(0)), 5);

        {
            let arr = [5, 8];
            assert_eq!(arr.clone().into_field(fp!(0)), 5);
            assert_eq!(arr.clone().into_field(fp!(1)), 8);
        }

        {
            let arr = [5, 8, 13];
            assert_eq!(arr.clone().into_field(fp!(0)), 5);
            assert_eq!(arr.clone().into_field(fp!(1)), 8);
            assert_eq!(arr.clone().into_field(fp!(2)), 13);
        }

        {
            use crate::test_utils::DecOnDrop;
            use std_::cell::Cell;
            let counter = Cell::new(0);
            let cnt = DecOnDrop::new(&counter);

            [cnt.clone()].into_field(fp!(0));

            assert_eq!(counter.get(), 1);

            {
                let arr = [cnt.clone(), cnt.clone()];
                assert_eq!(counter.get(), 3);
                arr.clone().into_field(fp!(0));
                arr.clone().into_field(fp!(1));
            }
            assert_eq!(counter.get(), 1);

            {
                let arr = [cnt.clone(), cnt.clone(), cnt.clone()];
                assert_eq!(counter.get(), 4);
                arr.clone().into_field(fp!(0));
                arr.clone().into_field(fp!(1));
                arr.clone().into_field(fp!(2));
            }
            assert_eq!(counter.get(), 1);

            drop(cnt);
            assert_eq!(counter.get(), 0);
        }
    }

    #[test]
    fn structural_aliases() {
        let mut array = [0i32; 32];
        (0..=31).for_each(|x| array[x as usize] = 100 + x);

        macro_rules! structural_alias_test {
            (
                $size:literal,$trait_:ident,[$(($field:ident,$index:literal)),* $(,)* ]
            ) => ({
                fn constraint<T>(_:&impl $trait_<T>){}

                let arr=<&[i32;$size]>::try_from(&array[0..$size]).unwrap().clone();
                constraint(&arr);
                $(
                    assert_eq!( arr.field_($field), &array[$index] );
                )*
            })
        }

        structural_alias_test!(1, Array1, [(I0, 0),]);

        structural_alias_test!(
            7,
            Array7,
            [
                (I0, 0),
                (I1, 1),
                (I2, 2),
                (I3, 3),
                (I4, 4),
                (I5, 5),
                (I6, 6),
            ]
        );

        structural_alias_test!(8, Array8, [(I0, 0), (I6, 6), (I7, 7),]);

        structural_alias_test!(9, Array9, [(I0, 0), (I7, 7), (I8, 8),]);

        structural_alias_test!(
            15,
            Array15,
            [(I0, 0), (I7, 7), (I8, 8), (I9, 9), (I14, 14),]
        );

        structural_alias_test!(
            16,
            Array16,
            [(I0, 0), (I7, 7), (I8, 8), (I14, 14), (I15, 15),]
        );

        structural_alias_test!(
            17,
            Array17,
            [(I0, 0), (I7, 7), (I8, 8), (I14, 14), (I15, 15), (I16, 16),]
        );

        structural_alias_test!(
            23,
            Array23,
            [
                (I0, 0),
                (I7, 7),
                (I8, 8),
                (I14, 14),
                (I15, 15),
                (I16, 16),
                (I22, 22),
            ]
        );

        structural_alias_test!(
            24,
            Array24,
            [
                (I0, 0),
                (I7, 7),
                (I8, 8),
                (I14, 14),
                (I15, 15),
                (I16, 16),
                (I22, 22),
                (I23, 23),
            ]
        );

        structural_alias_test!(
            30,
            Array30,
            [
                (I0, 0),
                (I7, 7),
                (I8, 8),
                (I14, 14),
                (I15, 15),
                (I16, 16),
                (I22, 22),
                (I23, 23),
                (I29, 29),
            ]
        );

        structural_alias_test!(
            31,
            Array31,
            [
                (I0, 0),
                (I7, 7),
                (I8, 8),
                (I14, 14),
                (I15, 15),
                (I16, 16),
                (I22, 22),
                (I23, 23),
                (I29, 29),
                (I30, 30),
            ]
        );

        structural_alias_test!(
            32,
            Array32,
            [
                (I0, 0),
                (I7, 7),
                (I8, 8),
                (I14, 14),
                (I15, 15),
                (I16, 16),
                (I22, 22),
                (I23, 23),
                (I29, 29),
                (I30, 30),
                (I31, 31),
            ]
        );
    }
}
