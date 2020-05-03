use structural::{
    for_examples::{MaxFields, Tuple16},
    structural_aliases::{ArrayMove32, TupleMove12},
    StructuralExt,
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
