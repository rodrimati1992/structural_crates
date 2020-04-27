use crate::{
    field::{for_arrays::names, DropFields, IntoFieldMut, IntoVariantFieldMut, MovedOutFields},
    structural_trait::Structural,
};

macro_rules! impl_tuple {
    (inner;
        ($field:tt,$field_ty:ident,$field_index:expr,$field_param:ty)
        ($($tuple_param:ident),* $(,)* )
    )=>{
        _private_impl_getter!{
            unsafe impl[$($tuple_param),*]
                IntoFieldMut< $field:$field_ty,$field_index,$field_param >
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
        impl<$($field_ty),*> Structural for $tuple_ty {}

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

        unsafe impl<$($field_ty,)*> DropFields for $tuple_ty {
            #[inline(always)]
            fn pre_move(&mut self){}

            unsafe fn drop_fields(&mut self,moved: MovedOutFields){
                use $crate::pmr::FieldBit;
                $({
                    const BIT: FieldBit = FieldBit::new($field);
                    if !moved.is_moved_out(BIT) {
                        std::ptr::drop_in_place(&mut self.$field)
                    }
                })*
            }
        }


        /// A structural alias for a tuple variant of the size,
        /// in which all fields have mutable and by-value accessors.
        ///
        /// The last type parameter takes the name of the variant as a
        /// [TStr](../../struct.TStr.html)
        pub trait $variant_trait<$($field_ty,)* V>:
            $(
                IntoVariantFieldMut<V,names::$field_param,Ty=$field_ty>+
            )*
        {}

        impl<$($field_ty,)* This,V> $variant_trait<$($field_ty,)* V> for This
        where
            This:
                $(
                    IntoVariantFieldMut<V,names::$field_param,Ty=$field_ty>+
                )*
        {}

        $(
            impl_tuple!{
                inner;
                ($field, $field_ty, $field, names::$field_param) $tuple_ty
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
                {I4}Tuple{}Variant,\n\
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
    Tuple1Variant,
    [
        (0,C0,I0)
    ]
    (C0,)
}
impl_tuple! {
    Tuple2,
    Tuple2Variant,
    [
        (0,C0,I0),
        (1,C1,I1)
    ]
    (C0,C1,)
}
impl_tuple! {
    Tuple3,
    Tuple3Variant,
    [
        (0,C0,I0),
        (1,C1,I1),
        (2,C2,I2)
    ]
    (C0,C1,C2,)
}
impl_tuple! {
    Tuple4,
    Tuple4Variant,
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
    Tuple5Variant,
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
    Tuple6Variant,
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
    Tuple7Variant,
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
    Tuple8Variant,
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
    Tuple9Variant,
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
    Tuple10Variant,
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
    Tuple11Variant,
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
    Tuple12Variant,
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
#[allow(clippy::redundant_clone)]
mod tests {
    use crate::field::for_tuples::{Tuple4, Tuple4Variant};
    use crate::{fp, GetField, Structural, StructuralExt};

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

    fn takes_tuple4<This>(mut this: This)
    where
        This: Tuple4<u32, u32, u32, u32> + Clone,
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

        assert_eq!(this.clone().into_field(fp!(0)), 6);
        assert_eq!(this.clone().into_field(fp!(1)), 5);
        assert_eq!(this.clone().into_field(fp!(2)), 4);
        assert_eq!(this.clone().into_field(fp!(3)), 3);
    }

    #[test]
    fn tuple4_test() {
        takes_tuple4((6, 5, 4, 3, 2, 1));
        takes_tuple4((6, 5, 4, 3, 2));
        takes_tuple4((6, 5, 4, 3));
    }

    fn takes_tuple4_variant<This>(this: This)
    where
        This: Tuple4Variant<u32, u32, u32, u32, TS!(Foo)> + Clone,
    {
        takes_tuple4(this.into_field(fp!(::Foo)).unwrap())
    }

    #[test]
    fn tuple4_variant_test() {
        takes_tuple4_variant(Enum::Foo((6, 5, 4, 3, 2, 1)));
        takes_tuple4_variant(Enum::Foo((6, 5, 4, 3, 2)));
        takes_tuple4_variant(Enum::Foo((6, 5, 4, 3)));
    }

    #[derive(Structural, Clone)]
    enum Enum<T> {
        #[struc(newtype)]
        Foo(T),
    }
}
