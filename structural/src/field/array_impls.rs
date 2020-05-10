#![allow(non_snake_case)]

use crate::{
    field::{
        DropFields, FieldBit, FieldType, GetField, GetFieldMut, GetFieldRawMutFn, IntoField,
        MovedOutFields,
    },
    path::{
        array_paths::{
            IsPathForArray, I0, I1, I10, I11, I12, I13, I14, I15, I16, I17, I18, I19, I2, I20, I21,
            I22, I23, I24, I25, I26, I27, I28, I29, I3, I30, I31, I4, I5, I6, I7, I8, I9,
        },
        FieldPathSet, LargePathSet,
    },
    structural_trait::Structural,
    StructuralExt,
};

use std_::{mem::ManuallyDrop, ptr};

////////////////////////////////////////////////////////////////////////////////

use crate::structural_aliases::{
    ArrayMove0, ArrayMove1, ArrayMove10, ArrayMove11, ArrayMove12, ArrayMove13, ArrayMove14,
    ArrayMove15, ArrayMove16, ArrayMove17, ArrayMove18, ArrayMove19, ArrayMove2, ArrayMove20,
    ArrayMove21, ArrayMove22, ArrayMove23, ArrayMove24, ArrayMove25, ArrayMove26, ArrayMove27,
    ArrayMove28, ArrayMove29, ArrayMove3, ArrayMove30, ArrayMove31, ArrayMove32, ArrayMove4,
    ArrayMove5, ArrayMove6, ArrayMove7, ArrayMove8, ArrayMove9,
};

////////////////////////////////////////////////////////////////////////////////

macro_rules! array_impls {
    (
        $(
            (
                $index_name:ident = $index:tt,$index_str:literal,$tnum:ident,$trait:ident
            )
        )*
    ) => (
        $(
            impl<T> Structural for [T;$index]{}

            impl<T,P> FieldType<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                type Ty=T;
            }

            impl<T,P> GetField<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                #[inline(always)]
                fn get_field_(&self,_:P)->&Self::Ty{
                    &self[P::INDEX]
                }
            }

            unsafe impl<T,P> GetFieldMut<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                #[inline(always)]
                fn get_field_mut_(&mut self,_:P)->&mut Self::Ty{
                    &mut self[P::INDEX]
                }

                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    ptr:*mut  (),
                    _:P,
                )->*mut Self::Ty{
                    let ptr=ptr as *mut T;
                    ptr.add(P::INDEX)
                }

                #[inline(always)]
                fn get_field_raw_mut_fn(&self)->GetFieldRawMutFn<P,Self::Ty>{
                    <Self as GetFieldMut<P>>::get_field_raw_mut
                }
            }

            unsafe impl<T,P> IntoField<P> for [T;$index]
            where
                P:IsPathForArray<Self>,
            {
                #[inline(always)]
                fn into_field_(self,_:P)->Self::Ty{
                    unsafe{
                        let mut this=ManuallyDrop::new(self);
                        ptr::drop_in_place(&mut this[..P::INDEX]);
                        ptr::drop_in_place(&mut this[P::INDEX+1..]);
                        this.as_mut_ptr().add(P::INDEX).read()
                    }
                }

                unsafe fn move_out_field_(
                    &mut self,
                    _field_name: P,
                    moved: &mut MovedOutFields,
                )->T{
                    moved.set_moved_out(P::DROP_BIT);
                    self.as_mut_ptr().add(P::INDEX).read()
                }
            }

            unsafe impl<T> DropFields for [T;$index]{
                #[inline(always)]
                fn pre_move(&mut self){}

                unsafe fn drop_fields(&mut self,moved: MovedOutFields){
                    for (i,mutref) in (0..$index).zip(self) {
                        if !moved.is_moved_out(FieldBit::new(i)) {
                            std::ptr::drop_in_place(mutref);
                        }
                    }
                }
            }
        )*

        array_impls!{
            @inner
            before()
            $(
                (
                    $index_name = $index, $tnum, $trait
                )
            )*
        }
    );
    (@inner
        before( $( ( $index_name:ident = $index:tt, $field_name:ident, $trait:ident ) )* )
        ( $next_index_name:ident = $next_index:tt, $next_field_name:ident, $next_trait:ident )
        $($after:tt)*
    ) => (
        array_impls!{
            @inner
            before(
                $( ( $index_name = $index, $field_name, $trait ) )*
                ( $next_index_name = $next_index, $next_field_name, $next_trait )
            )
            $($after)*
        }

        z_impl_from_structural!{
            impl[T,F] FromStructural<F> for [T;$next_index]
            where[ F: $next_trait<T> ]
            {
                fn from_structural(from){
                    // This unsafe is checked in the test for converting arrays to smaller srrays
                    let path_set=unsafe{
                        let x=LargePathSet(path_tuple!( $($index_name,)* ));
                        let x=FieldPathSet::many(x).upgrade_unchecked();
                        x
                    };
                    let field_pat!($($field_name,)*)=from.into_fields(path_set);
                    [$($field_name),*]
                }
            }
        }
    );
    (@inner before $before:tt ) => ();
}

/*
Generated with:

fn main() {
    for i in 0..=32{
        println!("(I{0}={0},\"{0}\",U{0})",i);
    }
}

*/

array_impls! {
    (I0=0,"0",U0,ArrayMove0)
    (I1=1,"1",U1,ArrayMove1)
    (I2=2,"2",U2,ArrayMove2)
    (I3=3,"3",U3,ArrayMove3)
    (I4=4,"4",U4,ArrayMove4)
    (I5=5,"5",U5,ArrayMove5)
    (I6=6,"6",U6,ArrayMove6)
    (I7=7,"7",U7,ArrayMove7)
    (I8=8,"8",U8,ArrayMove8)
    (I9=9,"9",U9,ArrayMove9)
    (I10=10,"10",U10,ArrayMove10)
    (I11=11,"11",U11,ArrayMove11)
    (I12=12,"12",U12,ArrayMove12)
    (I13=13,"13",U13,ArrayMove13)
    (I14=14,"14",U14,ArrayMove14)
    (I15=15,"15",U15,ArrayMove15)
    (I16=16,"16",U16,ArrayMove16)
    (I17=17,"17",U17,ArrayMove17)
    (I18=18,"18",U18,ArrayMove18)
    (I19=19,"19",U19,ArrayMove19)
    (I20=20,"20",U20,ArrayMove20)
    (I21=21,"21",U21,ArrayMove21)
    (I22=22,"22",U22,ArrayMove22)
    (I23=23,"23",U23,ArrayMove23)
    (I24=24,"24",U24,ArrayMove24)
    (I25=25,"25",U25,ArrayMove25)
    (I26=26,"26",U26,ArrayMove26)
    (I27=27,"27",U27,ArrayMove27)
    (I28=28,"28",U28,ArrayMove28)
    (I29=29,"29",U29,ArrayMove29)
    (I30=30,"30",U30,ArrayMove30)
    (I31=31,"31",U31,ArrayMove31)
    (I32=32,"32",U32,ArrayMove32)
}

// fn foo() {
//     let foo: &dyn crate::structural_aliases::Array8<u32> = &[0, 1, 2];
// }

///
///
/// ```compile_fail
/// use structural::{StructuralExt,fp};
///
/// let _=[0;31].field_(fp!(31));
///
/// ```
///
/// ```rust
/// use structural::{StructuralExt,fp};
///
/// let _=[0;31].field_(fp!(30));
///
/// ```
///
#[cfg(feature = "testing")]
#[allow(dead_code)]
struct Foo0;

#[cfg(test)]
#[allow(clippy::redundant_clone)]
mod tests {
    use super::{I0, I1, I14, I15, I16, I2, I22, I23, I29, I3, I30, I31, I4, I5, I6, I7, I8, I9};

    use crate::{
        structural_aliases::{
            Array1, Array15, Array16, Array17, Array23, Array24, Array30, Array31, Array32, Array7,
            Array8, Array9,
        },
        StructuralExt,
    };

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
    // Every statement after the array initialization is unrelated to every other statement.
    #[allow(clippy::cognitive_complexity)]
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
                    assert_eq!( arr.field_(<$field>::NEW), &array[$index] );
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
