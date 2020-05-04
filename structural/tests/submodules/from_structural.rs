use structural::{
    for_examples::{MaxFields, Tuple16},
    structural_aliases as sa,
    structural_aliases::{ArrayMove32, TupleMove12},
    IntoField, StrucWrapper, StructuralExt, FP,
};

fn from_array32_tests(this: impl ArrayMove32<u8> + Copy) {
    this.into_struc::<[u8; 0]>();
    assert_eq!(this.into_struc::<[_; 1]>(), [11]);
    assert_eq!(this.into_struc::<[_; 2]>(), [11, 12]);
    assert_eq!(this.into_struc::<[_; 3]>(), [11, 12, 13]);
    assert_eq!(
        this.into_struc::<[_; 8]>(),
        [11, 12, 13, 14, 15, 16, 17, 18]
    );
    assert_eq!(
        this.into_struc::<[_; 9]>(),
        [11, 12, 13, 14, 15, 16, 17, 18, 21]
    );
    assert_eq!(
        this.into_struc::<[_; 15]>(),
        [11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27]
    );
    assert_eq!(
        this.into_struc::<[_; 16]>(),
        [11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28]
    );
    assert_eq!(
        this.into_struc::<[_; 17]>(),
        [11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31],
    );
    assert_eq!(
        this.into_struc::<[_; 30]>(),
        [
            11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36,
            37, 38, 41, 42, 43, 44, 45, 46
        ],
    );
    assert_eq!(
        this.into_struc::<[_; 31]>(),
        [
            11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36,
            37, 38, 41, 42, 43, 44, 45, 46, 47
        ],
    );
    assert_eq!(
        this.into_struc::<[_; 32]>(),
        [
            11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36,
            37, 38, 41, 42, 43, 44, 45, 46, 47, 48
        ],
    );
}

#[test]
fn array_from_structural() {
    from_array32_tests(MaxFields::<u8>(
        11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37,
        38, 41, 42, 43, 44, 45, 46, 47, 48, 51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66,
        67, 68, 71, 72, 73, 74, 75, 76, 77, 78, 81, 82, 83, 84, 85, 86, 87, 88,
    ));

    from_array32_tests([
        11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37,
        38, 41, 42, 43, 44, 45, 46, 47, 48,
    ]);
}

fn from_tuple_tests(
    this: impl TupleMove12<
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            Option<u8>,
            Option<u16>,
            Option<u32>,
            Option<u64>,
        > + Copy,
) {
    assert_eq!(this.into_struc::<()>(), ());
    assert_eq!(this.into_struc::<(_,)>(), (3u8,));
    assert_eq!(this.into_struc::<(_, _)>(), (3u8, 5u16,));
    assert_eq!(this.into_struc::<(_, _, _)>(), (3u8, 5u16, 8u32,));
    assert_eq!(this.into_struc::<(_, _, _, _)>(), (3u8, 5u16, 8u32, 13u64,));
    assert_eq!(
        this.into_struc::<(_, _, _, _, _)>(),
        (3u8, 5u16, 8u32, 13u64, 21i8,)
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _)>(),
        (3u8, 5u16, 8u32, 13u64, 21i8, 34i16,)
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _, _)>(),
        (3u8, 5u16, 8u32, 13u64, 21i8, 34i16, 55i32,)
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _, _, _)>(),
        (3u8, 5u16, 8u32, 13u64, 21i8, 34i16, 55i32, 89i64,)
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _, _, _, _)>(),
        (3u8, 5u16, 8u32, 13u64, 21i8, 34i16, 55i32, 89i64, Some(3u8),)
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _, _, _, _, _)>(),
        (
            3u8,
            5u16,
            8u32,
            13u64,
            21i8,
            34i16,
            55i32,
            89i64,
            Some(3u8),
            Some(5u16),
        )
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _, _, _, _, _, _)>(),
        (
            3u8,
            5u16,
            8u32,
            13u64,
            21i8,
            34i16,
            55i32,
            89i64,
            Some(3u8),
            Some(5u16),
            Some(8u32),
        )
    );
    assert_eq!(
        this.into_struc::<(_, _, _, _, _, _, _, _, _, _, _, _)>(),
        (
            3u8,
            5u16,
            8u32,
            13u64,
            21i8,
            34i16,
            55i32,
            89i64,
            Some(3u8),
            Some(5u16),
            Some(8u32),
            Some(13u64),
        )
    );
}

#[test]
fn tuples_from_structural() {
    from_tuple_tests(Tuple16(
        3u8,
        5u16,
        8u32,
        13u64,
        21i8,
        34i16,
        55i32,
        89i64,
        Some(3u8),
        Some(5u16),
        Some(8u32),
        Some(13u64),
        Some(21i8),
        Some(34i16),
        Some(55i32),
        Some(89i64),
    ));

    from_tuple_tests((
        3u8,
        5u16,
        8u32,
        13u64,
        21i8,
        34i16,
        55i32,
        89i64,
        Some(3u8),
        Some(5u16),
        Some(8u32),
        Some(13u64),
    ));
}

#[test]
fn ranges_from_structural() {
    use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

    type Start_STR = FP!(start);
    type End_STR = FP!(end);

    {
        fn into_range<T>(
            this: impl IntoField<Start_STR, Ty = T> + IntoField<End_STR, Ty = T>,
        ) -> Range<T> {
            this.into_struc()
        }

        assert_eq!(into_range(100..999), 100..999);
    }
    {
        fn into_rangeinclusive<T>(
            this: impl IntoField<Start_STR, Ty = T> + IntoField<End_STR, Ty = T>,
        ) -> RangeInclusive<T> {
            this.into_struc()
        }

        assert_eq!(into_rangeinclusive(100..999), 100..=999);
    }
    {
        fn into_rangefrom<T>(this: impl IntoField<Start_STR, Ty = T>) -> RangeFrom<T> {
            this.into_struc()
        }

        assert_eq!(into_rangefrom(100..999), 100..);
    }
    {
        fn into_rangeto<T>(this: impl IntoField<End_STR, Ty = T>) -> RangeTo<T> {
            this.into_struc()
        }

        assert_eq!(into_rangeto(100..999), ..999);
    }
    {
        fn into_rangetoi<T>(this: impl IntoField<End_STR, Ty = T>) -> RangeToInclusive<T> {
            this.into_struc()
        }

        assert_eq!(into_rangetoi(100..999), ..=999);
    }
}

#[test]
fn wrappers_from_structural() {
    use std::mem::ManuallyDrop;

    {
        fn into_manuallydrop<T>(
            this: impl IntoField<FP!(0), Ty = T> + IntoField<FP!(1), Ty = T>,
        ) -> ManuallyDrop<[T; 2]> {
            this.into_struc()
        }

        assert_eq!(into_manuallydrop((3, 5)), ManuallyDrop::new([3, 5]));
        assert_eq!(
            into_manuallydrop((8, 13, 21, 34)),
            ManuallyDrop::new([8, 13])
        );
    }
    {
        fn into_wrapper<T>(
            this: impl IntoField<FP!(0), Ty = T> + IntoField<FP!(1), Ty = T>,
        ) -> StrucWrapper<[T; 2]> {
            this.into_struc()
        }

        assert_eq!(into_wrapper((3, 5)), StrucWrapper([3, 5]));
        assert_eq!(into_wrapper((8, 13, 21, 34)), StrucWrapper([8, 13]));
    }
}

#[test]
#[cfg(feature = "alloc")]
fn alloc_from_structural() {
    use std::{pin::Pin, rc::Rc, sync::Arc};

    macro_rules! ptr_test {
        ($type:ident) => {{
            fn into_ptr<T>(
                this: impl IntoField<FP!(0), Ty = T> + IntoField<FP!(1), Ty = T>,
            ) -> $type<[T; 2]> {
                this.into_struc()
            }

            assert_eq!(into_ptr((3, 5)), $type::new([3, 5]));
            assert_eq!(into_ptr((8, 13, 21, 34)), $type::new([8, 13]));
        }};
    }

    ptr_test!(Box);
    ptr_test!(Rc);
    ptr_test!(Arc);

    {
        fn into_pin<T: Unpin>(
            this: impl IntoField<FP!(0), Ty = T> + IntoField<FP!(1), Ty = T>,
        ) -> Pin<Box<[T; 2]>> {
            this.into_struc()
        }

        assert_eq!(into_pin((3, 5)), Pin::new(Box::new([3, 5])));
        assert_eq!(into_pin((8, 13, 21, 34)), Pin::new(Box::new([8, 13])));
    }
}

#[test]
fn enum_from_structural() {
    use structural::for_examples::{OptionLike, ResultLike};

    {
        fn option_from<T>(this: impl sa::OptionMove_ESI<T>) -> Option<T> {
            this.into_struc()
        }

        assert_eq!(option_from(OptionLike::Some(3)), Some(3));
        assert_eq!(option_from(OptionLike::Some(5)), Some(5));
        assert_eq!(option_from(OptionLike::<()>::None), None);

        assert_eq!(option_from(Some(8)), Some(8));
        assert_eq!(option_from(Some(13)), Some(13));
        assert_eq!(option_from(None::<()>), None);
    }
    {
        fn result_from<T, E>(this: impl sa::ResultMove_ESI<T, E>) -> Result<T, E> {
            this.into_struc()
        }

        assert_eq!(result_from(ResultLike::<_, ()>::Ok(3)), Ok(3));
        assert_eq!(result_from(ResultLike::<_, ()>::Ok(5)), Ok(5));
        assert_eq!(result_from(ResultLike::<(), _>::Err(8)), Err(8));
        assert_eq!(result_from(ResultLike::<(), _>::Err(13)), Err(13));

        assert_eq!(result_from(Ok::<_, ()>(21)), Ok(21));
        assert_eq!(result_from(Ok::<_, ()>(34)), Ok(34));
        assert_eq!(result_from(Err::<(), _>(55)), Err(55));
        assert_eq!(result_from(Err::<(), _>(89)), Err(89));
    }
}
