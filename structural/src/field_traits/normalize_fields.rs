use super::{EnumField, StructField};

/// Converts the `Result<T,E>` representation of accessed fields to either `T` or `Option<T>`
///
/// If Self is a:
///
/// - `Result<T,StructField>`: it's converted to `T`
///
/// - `Result<T,EnumField>`: it's converted to `Option<T>`
///
/// - A tuple of `Result`s:
/// it's converted to a tuple of what those `Result`s `.normalize_fields()` into.
///
///
pub trait NormalizeFields {
    /// The thing this is converted into.
    type Output;

    /// Performs the conversion..
    fn normalize_fields(self) -> Self::Output;
}

/// The type `This` is converted into when calling `.normalize_fields()`.
pub type NormalizeFieldsOut<This> = <This as NormalizeFields>::Output;

impl<T> NormalizeFields for Result<T, StructField> {
    type Output = T;

    #[inline(always)]
    fn normalize_fields(self) -> Self::Output {
        match self {
            Ok(x) => x,
            Err(e) => match e {},
        }
    }
}

impl<T> NormalizeFields for Result<T, EnumField> {
    type Output = Option<T>;

    #[inline(always)]
    fn normalize_fields(self) -> Self::Output {
        self.ok()
    }
}

macro_rules! normalize_tuple {
    (
        $(( $(($elem:ident,$index:tt),)* ))*
    ) => {
        $(
            impl<$($elem,)*> NormalizeFields for ($($elem,)*)
            where
                $($elem:NormalizeFields,)*
            {
                type Output=(
                    $(NormalizeFieldsOut<$elem>,)*
                );

                #[inline(always)]
                fn normalize_fields(self)->Self::Output{
                    (
                        $(self.$index.normalize_fields(),)*
                    )
                }
            }
        )*
    };
}

/*
fn main(){
    let large=8;
    for x in 0..=13 {
        let is_large= x > large;
        print!("( ");
        for y in 0..x {
            if is_large && y%8==0 {
                print!("\n    ")
            }
            print!("(C{0},{0}), ",y);
        }
        if is_large {
            println!();
        }
        println!(")");
    }
}
*/

normalize_tuple! {
    ( )
    ( (C0,0), )
    ( (C0,0), (C1,1), )
    ( (C0,0), (C1,1), (C2,2), )
    ( (C0,0), (C1,1), (C2,2), (C3,3), )
    ( (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), )
    ( (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), )
    ( (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), )
    ( (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), (C7,7), )
    (
        (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), (C7,7),
        (C8,8),
    )
    (
        (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), (C7,7),
        (C8,8), (C9,9),
    )
    (
        (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), (C7,7),
        (C8,8), (C9,9), (C10,10),
    )
    (
        (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), (C7,7),
        (C8,8), (C9,9), (C10,10), (C11,11),
    )
    (
        (C0,0), (C1,1), (C2,2), (C3,3), (C4,4), (C5,5), (C6,6), (C7,7),
        (C8,8), (C9,9), (C10,10), (C11,11), (C12,12),
    )
}
