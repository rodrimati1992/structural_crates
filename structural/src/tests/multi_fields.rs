use crate::field::{
    multi_fields::{
        RevGetMultiField, RevGetMultiFieldImpl, RevGetMultiFieldImplOut, RevGetMultiFieldMut,
        RevGetMultiFieldMutImpl, RevGetMultiFieldMutImplOut, RevGetMultiFieldMutImplRaw,
        RevGetMultiFieldMutOut, RevGetMultiFieldMutRaw, RevGetMultiFieldOut,
    },
    FailedAccess, InfallibleAccess,
};
use crate::field_path_aliases;

use core_extensions::type_asserts::AssertEq3;

field_path_aliases! {
    Foo=(3,1,4?),
}

type TupleA = (u8, u16, u32, u64, Option<i8>, Option<i16>);

#[allow(non_upper_case_globals)]
const TupleA: TupleA = (3, 5, 8, 13, Some(21), None);

#[test]
fn rev_get_multi_field() {
    {
        type TupleA_Ref<'a> = (&'a u64, &'a u16, Option<&'a i8>);
        let _: AssertEq3<
            <Foo as RevGetMultiField<'_, TupleA>>::Fields,
            RevGetMultiFieldOut<'_, Foo, TupleA>,
            TupleA_Ref<'_>,
        >;
        let ret: TupleA_Ref<'_> = RevGetMultiField::rev_get_multi_field(Foo, &TupleA);
        assert_eq!(ret, (&13, &5, Some(&21)));
    }
}

#[test]
fn rev_get_multi_field_mut() {
    {
        type TupleA_Mut0<'a> = (&'a mut u64, &'a mut u16, Option<&'a mut i8>);
        let _: AssertEq3<
            <Foo as RevGetMultiFieldMut<'_, TupleA>>::FieldsMut,
            RevGetMultiFieldMutOut<'_, Foo, TupleA>,
            TupleA_Mut0<'_>,
        >;
        let mut tup = TupleA;
        let ret: TupleA_Mut0<'_> = RevGetMultiFieldMut::rev_get_multi_field_mut(Foo, &mut tup);
        assert_eq!(ret, (&mut 13, &mut 5, Some(&mut 21)));
    }
    {
        type TupleA_Raw0 = (*mut u64, *mut u16, Option<*mut i8>);
        let _: AssertEq3<
            <Foo as RevGetMultiFieldMut<'_, TupleA>>::FieldsRawMut,
            RevGetMultiFieldMutRaw<'_, Foo, TupleA>,
            TupleA_Raw0,
        >;
        let mut tup = TupleA;
        unsafe {
            let ret: TupleA_Raw0 =
                RevGetMultiFieldMut::rev_get_multi_field_raw_mut(Foo, &mut tup as *mut TupleA);
            assert_eq!(
                (&mut *ret.0, &mut *ret.1, ret.2.map(|x| &mut *x),),
                (&mut 13_u64, &mut 5_u16, Some(&mut 21_i8),),
            );
        };
    }
}

#[test]
fn rev_get_multi_field_impl() {
    {
        type TupleA_Ref<'a> = (
            Result<&'a u64, InfallibleAccess>,
            Result<&'a u16, InfallibleAccess>,
            Result<&'a i8, FailedAccess>,
        );
        let _: AssertEq3<
            <Foo as RevGetMultiFieldImpl<'_, TupleA>>::UnnormFields,
            RevGetMultiFieldImplOut<'_, Foo, TupleA>,
            TupleA_Ref<'_>,
        >;
        let ret: TupleA_Ref<'_> = RevGetMultiFieldImpl::rev_get_multi_field_impl(Foo, &TupleA);
        assert_eq!(
            ret,
            (
                Ok::<_, InfallibleAccess>(&13),
                Ok::<_, InfallibleAccess>(&5),
                Ok::<_, FailedAccess>(&21),
            ),
        );
    }
}

#[test]
fn rev_get_multi_field_mut_impl() {
    {
        type TupleA_Mut0<'a> = (
            Result<&'a mut u64, InfallibleAccess>,
            Result<&'a mut u16, InfallibleAccess>,
            Result<&'a mut i8, FailedAccess>,
        );
        let _: AssertEq3<
            <Foo as RevGetMultiFieldMutImpl<'_, TupleA>>::UnnormFieldsMut,
            RevGetMultiFieldMutImplOut<'_, Foo, TupleA>,
            TupleA_Mut0<'_>,
        >;
        let mut tup = TupleA;
        let ret: TupleA_Mut0<'_> =
            RevGetMultiFieldMutImpl::rev_get_multi_field_mut_impl(Foo, &mut tup);
        assert_eq!(
            ret,
            (
                Ok::<_, InfallibleAccess>(&mut 13),
                Ok::<_, InfallibleAccess>(&mut 5),
                Ok::<_, FailedAccess>(&mut 21),
            ),
        );
    }
    {
        type TupleA_Raw0 = (
            Result<*mut u64, InfallibleAccess>,
            Result<*mut u16, InfallibleAccess>,
            Result<*mut i8, FailedAccess>,
        );
        let _: AssertEq3<
            <Foo as RevGetMultiFieldMutImpl<'_, TupleA>>::UnnormFieldsRawMut,
            RevGetMultiFieldMutImplRaw<'_, Foo, TupleA>,
            TupleA_Raw0,
        >;
        let mut tup = TupleA;
        unsafe {
            let ret: TupleA_Raw0 = RevGetMultiFieldMutImpl::rev_get_multi_field_raw_mut_impl(
                Foo,
                &mut tup as *mut TupleA,
            );
            assert_eq!(
                (
                    ret.0.map(|x| &mut *x),
                    ret.1.map(|x| &mut *x),
                    ret.2.map(|x| &mut *x),
                ),
                (
                    Ok::<_, InfallibleAccess>(&mut 13_u64),
                    Ok::<_, InfallibleAccess>(&mut 5_u16),
                    Ok::<_, FailedAccess>(&mut 21_i8),
                ),
            );
        };
    }
}
