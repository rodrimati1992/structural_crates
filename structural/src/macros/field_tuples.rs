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
/// This macro is most useful when destructuring a tuple of over 8 fields,
/// since accessing over 8 fields returns a tuple of tuples (8 fields each).
///
/// # StructuralExt Example
///
/// <span id="the-structuralext-example"></span>
///
/// These examples demonstrate both
/// the return value of `StructuralExt` methods that return over 8 fields,
/// as well as destructuring the return value using the `field_pat` macro.
///
/// Calling **`StructuralExt::fields`**:
/// ```rust
/// use structural::{field_pat, fp, StructuralExt};
///
/// let arr = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27];
///
/// assert_eq!(
///     arr.fields(fp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ,15, 16)),
///     (
///         (&10, &11, &12, &13, &14, &15, &16, &17),
///         (&18, &19, &20, &21, &22, &23, &24, &25),
///         (&26,)
///     )
/// );
///
/// let field_pat!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6)=
///     arr.fields(fp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ,15, 16));
///
/// assert_eq!(
///     [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6],
///     [&10, &11, &12, &13, &14, &15, &16, &17, &18, &19, &20, &21, &22, &23, &24, &25, &26],
/// );
/// ```
///
/// Calling **`StructuralExt::fields_mut`**:
/// ```rust
/// use structural::{field_pat, fp, StructuralExt};
///
/// let mut arr = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27];
///
/// assert_eq!(
///     arr.fields_mut(fp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ,15, 16)),
///     (
///         (&mut 10, &mut 11, &mut 12, &mut 13, &mut 14, &mut 15, &mut 16, &mut 17),
///         (&mut 18, &mut 19, &mut 20, &mut 21, &mut 22, &mut 23, &mut 24, &mut 25),
///         (&mut 26,)
///     )
/// );
///
/// let field_pat!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6)=
///     arr.fields_mut(fp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ,15, 16));
///
/// assert_eq!(
///     [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6],
///     [
///         &mut 10, &mut 11, &mut 12, &mut 13, &mut 14, &mut 15, &mut 16, &mut 17,
///         &mut 18, &mut 19, &mut 20, &mut 21, &mut 22, &mut 23, &mut 24, &mut 25,
///         &mut 26
///     ],
/// );
///
/// ```
///
/// Calling **`StructuralExt::into_fields`**:
/// ```rust
/// use structural::{field_pat, fp, StructuralExt};
///
/// let arr = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27];
///
/// assert_eq!(
///     arr.into_fields(fp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ,15, 16)),
///     (
///         (10, 11, 12, 13, 14, 15, 16, 17),
///         (18, 19, 20, 21, 22, 23, 24, 25),
///         (26,)
///     )
/// );
///
/// let field_pat!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6)=
///     arr.into_fields(fp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 ,15, 16));
///
/// assert_eq!(
///     [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6],
///     [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26],
/// );
///
/// ```
///
/// # Example
///
/// This demonstrates what values this pattern macro destructures.
///
/// ```rust
/// use structural::field_pat;
///
/// let field_pat!() = ();
///
/// let field_pat!(f0) = (0,);
///
/// let field_pat!(f0, f1) = (0, 1);
///
/// let field_pat!(f0, f1, f2, f3, f4, f5, f6, f7) = (0, 1, 2, 3, 4, 5, 6, 7);
///
/// let field_pat!(f0, f1, f2, f3, f4, f5, f6, f7, f8) = (
///     (0, 1, 2, 3, 4, 5, 6, 7),
///     (8,),
/// );
///
/// let field_pat!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5) = (
///     (0, 1, 2, 3, 4, 5, 6, 7),
///     (8, 9, 10, 11, 12, 13, 14 ,15),
/// );
///
/// let field_pat!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, b0, b1, b2, b3, b4, b5, b6) = (
///     (0, 1, 2, 3, 4, 5, 6, 7),
///     (8, 9, 10, 11, 12, 13, 14 ,15),
///     (16,),
/// );
///
/// ```
///
#[macro_export]
macro_rules! field_pat{
    ()=>{ () };
    ($f0:pat,$f1:pat,$f2:pat,$f3:pat,$f4:pat,$f5:pat,$f6:pat,$f7:pat $(,)? )=>{
        ($f0,$f1,$f2,$f3,$f4,$f5,$f6,$f7)
    };
    ($f0:pat,$f1:pat,$f2:pat,$f3:pat,$f4:pat,$f5:pat,$f6:pat,$f7:pat, $($rem:pat),* $(,)? )=>{
        $crate::__inner_field_pat!(
            output()
            rem($f0,$f1,$f2,$f3,$f4,$f5,$f6,$f7, $($rem,)*)
        )
    };
    ( $($f:pat),* $(,)? )=>{
        ($($f,)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __inner_path_tuple{
    (
        output($($prev:tt)*)
        rem($f0:expr,$f1:expr,$f2:expr,$f3:expr,$f4:expr,$f5:expr,$f6:expr,$f7:expr, $($rem:tt)*)
    )=>{
        $crate::__inner_path_tuple!(
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

/// For manually constructing a `FieldPathSet` to access up to 64 fields.
///
/// # Example
///
/// This demonstrates how to construct a `FieldPathSet` to access over 8 fields.
///
/// ```rust
/// use structural::{ FieldPathSet, StructuralExt, path_tuple, ts };
///
/// let this = ('a', 'b', 3, 5, "foo", "bar", false, true, Some(8), Some(13));
///
/// // If you access up to 8 fields with `FieldPathSet::large(path_tuple!(....))`,
/// // then accessor methods return non-nested tuples.
/// {
///     let path8 = FieldPathSet::large(path_tuple!(
///         ts!(0), ts!(1), ts!(2), ts!(3), ts!(4), ts!(5), ts!(6), ts!(7)
///     ));
///     assert_eq!(
///         this.fields(path8),
///         (&'a', &'b', &3, &5, &"foo", &"bar", &false, &true)
///     );
/// }
///
/// // If you access more than 8 fields with `FieldPathSet::large(path_tuple!(....))`,
/// // then accessor methods return nested tuples. 8 elements each.
/// {
///     let path10 = FieldPathSet::large(path_tuple!(
///         ts!(0), ts!(1), ts!(2), ts!(3), ts!(4), ts!(5), ts!(6), ts!(7),
///         ts!(8), ts!(9),
///     ));
///     assert_eq!(
///         this.fields(path10),
///         (
///             (&'a', &'b', &3, &5, &"foo", &"bar", &false, &true),
///             (&Some(8), &Some(13))
///         ),
///     );
///     assert_eq!(
///         this.cloned_fields(path10),
///         (
///             ('a', 'b', 3, 5, "foo", "bar", false, true),
///             (Some(8), Some(13))
///         ),
///     );
/// }
///
/// ```
///
/// # Example
///
/// This demnstrates what the macro expands into:
///
/// ```rust
/// use structural::path_tuple;
///
/// assert_eq!( path_tuple!(), () );
///
/// assert_eq!( path_tuple!(1), ((1,),) );
///
/// assert_eq!( path_tuple!(1, 2), ((1, 2),) );
///
/// assert_eq!(
///     path_tuple!(0, 1, 2, 3, 4, 5, 6, 7),
///     ((0, 1, 2, 3, 4, 5, 6, 7),),
/// );
///
/// assert_eq!(
///     path_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8),
///     ((0, 1, 2, 3, 4, 5, 6, 7), (8,)),
/// );
///
/// assert_eq!(
///     path_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15),
///     (
///         (0, 1, 2, 3, 4, 5, 6, 7),
///         (8, 9, 10, 11, 12, 13, 14, 15),
///     ),
/// );
///
/// assert_eq!(
///     path_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16),
///     (
///         (0, 1, 2, 3, 4, 5, 6, 7),
///         (8, 9, 10, 11, 12, 13, 14, 15),
///         (16,),
///     )
/// );
///
/// ```
///
#[macro_export]
macro_rules! path_tuple{
    ()=>{ () };
    ($($expr:expr),* $(,)?)=>{
        $crate::__inner_path_tuple!(output() rem($($expr,)*))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn field_pat_tests() {
        let field_pat!() = ();

        {
            let field_pat!(a) = (1,);
            assert_eq!(a, 1);
        }
        {
            let field_pat!(a, b, c, d, e, f, g, h) = (1, 2, 3, 4, 5, 6, 7, 8);
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
    fn path_tuple_tests() {
        let field_pat!() = ();

        {
            let ((a,),) = path_tuple!(1);
            assert_eq!(a, 1);
        }
        {
            let (tup0,) = path_tuple!(1, 2, 3, 4, 5, 6, 7, 8);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
        }
        {
            let (tup0, tup1) = path_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 10);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(tup1, (10,));
        }
        {
            let (tup0, tup1) = path_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18,);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(tup1, (11, 12, 13, 14, 15, 16, 17, 18,));
        }
        {
            let (tup0, tup1, tup2) =
                path_tuple!(1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18, 20,);
            assert_eq!(tup0, (1, 2, 3, 4, 5, 6, 7, 8));
            assert_eq!(tup1, (11, 12, 13, 14, 15, 16, 17, 18,));
            assert_eq!(tup2, (20,));
        }
    }
}
