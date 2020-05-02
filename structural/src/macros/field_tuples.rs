#[doc(hidden)]
#[macro_export]
macro_rules! __inner_field_pat{
    (
        output($($prev:tt)*)
        rem($f0:pat,$f1:pat,$f2:pat,$f3:pat,$f4:pat,$f5:pat,$f6:pat,$f7:pat, $($rem:tt)*)
    )=>{
        $crate::__inner_field_pat!(
            output($($prev)* ($f0,$f1,$f2,$f3,$f4,$f5,$f6,$f7), )
            rem( $($rem)* )
        )
    };
    (
        output($($prev:tt)*)
        rem()
    )=>{
        ($($prev)*)
    };
    (
        output($($prev:tt)*)
        rem($($rem:tt)*)
    )=>{
        ($($prev)* ($($rem)*),)
    };
}

/// Macro to destructure the tuple returned by `StructuralExt` methods that
/// access multiple fields.
///
/// This macro is most useful when destructuring a tuple of over 8 fields
///
///
#[macro_export]
macro_rules! field_pat{
    ()=>{ () };
    ($($pattern:pat),* $(,)?)=>{
        $crate::__inner_field_pat!(output() rem($($pattern,)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_field_tuple{
    (
        output($($prev:tt)*)
        rem($f0:expr,$f1:expr,$f2:expr,$f3:expr,$f4:expr,$f5:expr,$f6:expr,$f7:expr, $($rem:tt)*)
    )=>{
        $crate::__inner_field_tuple!(
            output($($prev)* ($f0,$f1,$f2,$f3,$f4,$f5,$f6,$f7), )
            rem( $($rem)* )
        )
    };
    (
        output($($prev:tt)*)
        rem()
    )=>{
        ($($prev)*)
    };
    (
        output($($prev:tt)*)
        rem($($rem:tt)*)
    )=>{
        ($($prev)* ($($rem)*),)
    };
}

/// Constructs the tuple returned by `StructuralExt` methods that
/// access multiple fields.
///
/// This macro is most useful to cosntruct a tuple of over 8 fields
///
///
#[macro_export]
macro_rules! field_tuple{
    ()=>{ () };
    ($($expr:expr),* $(,)?)=>{
        $crate::__inner_field_tuple!(output() rem($($expr,)*))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn field_pat_tests() {
        let field_pat!() = ();

        {
            let field_pat!(a) = ((1,),);
            assert_eq!(a, 1);
        }
        {
            let field_pat!(a, b, c, d, e, f, g, h) = ((1, 2, 3, 4, 5, 6, 7, 8),);
            assert_eq!((a, b, c, d, e, f, g, h), (1, 2, 3, 4, 5, 6, 7, 8));
        }
        {
            let field_pat!(a, b, c, d, e, f, g, h, i,) = ((1, 2, 3, 4, 5, 6, 7, 8), (9,));
            assert_eq!((a, b, c, d, e, f, g, h), (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(i, 9);
        }
        {
            let field_pat!(
                f00, f01, f02, f03, f04, f05, f06, f07, f10, f11, f12, f13, f14, f15, f16, f17,
                f20,
            ) = (
                (1, 2, 3, 4, 5, 6, 7, 8),
                (11, 12, 13, 14, 15, 16, 17, 18),
                (20,),
            );
            assert_eq!(
                (f00, f01, f02, f03, f04, f05, f06, f07,),
                (1, 2, 3, 4, 5, 6, 7, 8)
            );
            assert_eq!(
                (f10, f11, f12, f13, f14, f15, f16, f17,),
                (11, 12, 13, 14, 15, 16, 17, 18)
            );
            assert_eq!(f20, 20);
        }
    }

    #[test]
    fn field_tuple_tests() {
        let field_pat!() = ();

        {
            let ((a,),) = field_tuple!(1);
            assert_eq!(a, 1);
        }
        {
            let (tup0,) = field_tuple!(1, 2, 3, 4, 5, 6, 7, 8);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
        }
        {
            let (tup0, tup1) = field_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 10);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(tup1, (10,));
        }
        {
            let (tup0, tup1) =
                field_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18,);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(tup1, (11, 12, 13, 14, 15, 16, 17, 18,));
        }
        {
            let (tup0, tup1, tup2) =
                field_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18, 20,);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(tup1, (11, 12, 13, 14, 15, 16, 17, 18,));
            assert_eq!(tup2, (20,));
        }
    }
}
