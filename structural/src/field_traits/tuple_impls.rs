use crate::{
    field_traits::{
        for_arrays::{names, strings},
        IntoFieldMut, IntoVariantFieldMut,
    },
    structural_trait::{FieldInfo, FieldInfos, Structural},
};

macro_rules! impl_tuple {
    (inner;
        ($field:tt,$field_ty:ident,$field_param:ty)
        ($($tuple_param:ident),* $(,)* )
    )=>{
        impl_getter!{
            unsafe impl[$($tuple_param),*]
                IntoFieldMut< $field:$field_ty,nonopt,$field_param >
            for ($($tuple_param,)*)
        }
    };
    (
        $the_trait:ident,
        $variant_trait:ident,
        [
            $( ($field:tt,$field_ty:ident,$field_param:ident) ),*
        ]
        $tuple_ty:tt
    ) => {
        impl<$($field_ty),*> crate::IsStructural for $tuple_ty { }

        impl<$($field_ty),*> Structural for $tuple_ty {
            const FIELDS: &'static $crate::structural_trait::FieldInfos={
                &FieldInfos::Struct(&[
                    $( FieldInfo::not_renamed(stringify!( $field )) ,)*
                ])
            };
        }

        /// A structural alias for a tuple of the size.
        pub trait $the_trait<$($field_ty),*>:
            $(
                IntoFieldMut<names::$field_param,Ty=$field_ty>+
            )*
        {}

        impl<$($field_ty,)* This> $the_trait<$($field_ty),*> for This
        where
            This:
                $(
                    IntoFieldMut<names::$field_param,Ty=$field_ty>+
                )*
        {}


        /// A structural alias for a tuple of the size.
        pub trait $variant_trait<V,$($field_ty),*>:
            $(
                IntoVariantFieldMut<V,strings::$field_param,Ty=$field_ty>+
            )*
        {}

        impl<$($field_ty,)* This,V> $variant_trait<V,$($field_ty),*> for This
        where
            This:
                $(
                    IntoVariantFieldMut<V,strings::$field_param,Ty=$field_ty>+
                )*
        {}

        $(
            impl_tuple!{
                inner;
                ($field,$field_ty,names::$field_param) $tuple_ty
            }
        )*
    }
}

/*
Code used to generate the macro invocations


use itertools::Itertools;

fn main(){
    for x in 1..=12 {
        let range=0..x;
        println!(
            "impl_tuple!{{\n\
                {I4}Tuple{},\n\
                {I4}TupleVariant{},\n\
                {I4}[\n\
                {I8}{}\n\
                {I4}]\n\
                {I4}({},)\n\
            }}",
            x,
            range.clone().map(|x|format!("({0},C{0},I{0})",x)).join(",\n        "),
            range.clone().map(|x|format!("C{0}",x)).join(","),
            I4="    ",
            I8="        ",
        );
    }
}

*/

impl_tuple! {
    Tuple1,
    TupleVariant1,
    [
        (0,C0,I0)
    ]
    (C0,)
}
impl_tuple! {
    Tuple2,
    TupleVariant2,
    [
        (0,C0,I0),
        (1,C1,I1)
    ]
    (C0,C1,)
}
impl_tuple! {
    Tuple3,
    TupleVariant3,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2)
    ]
    (C0,C1,C2,)
}
impl_tuple! {
    Tuple4,
    TupleVariant4,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3)
    ]
    (C0,C1,C2,C3,)
}
impl_tuple! {
    Tuple5,
    TupleVariant5,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4)
    ]
    (C0,C1,C2,C3,C4,)
}
impl_tuple! {
    Tuple6,
    TupleVariant6,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5)
    ]
    (C0,C1,C2,C3,C4,C5,)
}
impl_tuple! {
    Tuple7,
    TupleVariant7,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5),
        (6,C6,I6)
    ]
    (C0,C1,C2,C3,C4,C5,C6,)
}
impl_tuple! {
    Tuple8,
    TupleVariant8,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5),
        (6,C6,I6),
        (7,C7,I7)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,)
}
impl_tuple! {
    Tuple9,
    TupleVariant9,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5),
        (6,C6,I6),
        (7,C7,I7),
        (8,C8,I8)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,)
}
impl_tuple! {
    Tuple10,
    TupleVariant10,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5),
        (6,C6,I6),
        (7,C7,I7),
        (8,C8,I8),
        (9,C9,I9)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,)
}
impl_tuple! {
    Tuple11,
    TupleVariant11,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5),
        (6,C6,I6),
        (7,C7,I7),
        (8,C8,I8),
        (9,C9,I9),
        (10,C10,I10)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,)
}
impl_tuple! {
    Tuple12,
    TupleVariant12,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2),
        (3,C3,I3),
        (4,C4,I4),
        (5,C5,I5),
        (6,C6,I6),
        (7,C7,I7),
        (8,C8,I8),
        (9,C9,I9),
        (10,C10,I10),
        (11,C11,I11)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,)
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_field_1<T>(val: &T) -> &u64
    where
        T: GetField<FP!(1), Ty = u64>,
    {
        val.field_(fp!(1))
    }

    #[test]
    fn get_field_1_test() {
        assert_eq!(*get_field_1(&(2, 8)), 8);
        assert_eq!(*get_field_1(&(2, 5, 8)), 5);
        assert_eq!(*get_field_1(&(2, 3, 5, 8)), 3);
        assert_eq!(*get_field_1(&(1, 2, 3, 5, 8)), 2);
        assert_eq!(*get_field_1(&(1, 1, 2, 3, 5, 8)), 1);
        assert_eq!(*get_field_1(&(11, 13, 17, 19, 23, 29, 31)), 13);
        assert_eq!(*get_field_1(&(7, 11, 13, 17, 19, 23, 29, 31)), 11);
        assert_eq!(*get_field_1(&(5, 7, 11, 13, 17, 19, 23, 29, 31)), 7);
        assert_eq!(*get_field_1(&(3, 5, 7, 11, 13, 17, 19, 23, 29, 31)), 5);
    }

    #[test]
    fn get_mut_many() {
        {
            let mut tup = (0, 1, 2, 3, 4, 5);
            let (e0, e1) = tup.fields_mut(fp!(0, 1));
            *e0 = 101;
            *e1 = 102;

            assert_eq!(tup.0, 101);
            assert_eq!(tup.1, 102);
        }
        {
            let mut tup = (0, 1, 2, 3, 4, 5);
            let (e0, e1, e2) = tup.fields_mut(fp!(0, 1, 3));
            *e0 = 101;
            *e1 = 102;
            *e2 = 103;

            assert_eq!(tup.0, 101);
            assert_eq!(tup.1, 102);
            assert_eq!(tup.2, 2);
            assert_eq!(tup.3, 103);
            assert_eq!(tup.4, 4);
        }
        {
            let mut tup = (0, 1, 2, 3, 4, 5, 6, 7, 8);
            let (e0, e1, e2, e3) = tup.fields_mut(fp!(0, 1, 2, 8));
            *e0 = 101;
            *e1 = 102;
            *e2 = 103;
            *e3 = 200;

            assert_eq!(tup.0, 101);
            assert_eq!(tup.1, 102);
            assert_eq!(tup.2, 103);
            assert_eq!(tup.3, 3);
            assert_eq!(tup.4, 4);
            assert_eq!(tup.7, 7);
            assert_eq!(tup.8, 200);
        }
    }

    structural_alias! {
        trait Tuple4{
            mut move 0:u32,
            mut move 1:u32,
            mut move 2:u32,
            mut move 3:u32,
        }
    }

    fn takes_tuple4<This>(mut this: This)
    where
        This: Tuple4,
    {
        assert_eq!(this.fields(fp!(0, 1)), (&6, &5));
        assert_eq!(this.fields(fp!(0, 1, 2)), (&6, &5, &4));
        assert_eq!(this.fields(fp!(0, 1, 2, 3)), (&6, &5, &4, &3));

        assert_eq!(this.fields_mut(fp!(0, 1)), (&mut 6, &mut 5));
        assert_eq!(this.fields_mut(fp!(0, 1, 2)), (&mut 6, &mut 5, &mut 4));
        assert_eq!(
            this.fields_mut(fp!(0, 1, 2, 3)),
            (&mut 6, &mut 5, &mut 4, &mut 3)
        );
    }

    #[test]
    fn tuple4_test() {
        takes_tuple4((6, 5, 4, 3, 2, 1));
        takes_tuple4((6, 5, 4, 3, 2));
        takes_tuple4((6, 5, 4, 3));
    }
}
