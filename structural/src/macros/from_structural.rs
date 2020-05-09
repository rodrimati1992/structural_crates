/// For implementing [`FromStructural`],
/// and delegating the implementation of [`TryFromStructural`] to it.
///
/// [`FromStructural`]: ./convert/trait.FromStructural.html
/// [`TryFromStructural`]: ./convert/trait.TryFromStructural.html
///
/// # Example
///
/// This example demonstrates how you can implement `FromStructural` in a
/// more general way than necessary,
///
/// ```rust
/// use structural::{FP, IntoField, Structural, StructuralExt, fp, make_struct};
///
/// use std::borrow::Cow;
///
/// {
///     let this = make_struct!{
///         encoding: "utf8",
///         contents: &[0,1,2,3,4][..],
///     };
///     assert_eq!(
///         this.into_struc::<Message>(),
///         Message{encoding: "utf8".to_string(), contents: vec![0,1,2,3,4]}
///     );
/// }
/// {
///     let this = HttpMessage{
///         encoding: Cow::from("utf16"),
///         contents: Cow::from(vec![5,7,8]),
///         valid_until: 0o40002,
///     };
///     assert_eq!(
///         this.into_struc::<Message>(),
///         Message{encoding: "utf16".to_string(), contents: vec![5,7,8]}
///     );
/// }
///
/// #[derive(Structural, Debug)]
/// #[struc(no_trait, public, access="move")]
/// pub struct HttpMessage<'a> {
///     encoding: Cow<'a,str>,
///     contents: Cow<'a,[u8]>,
///     valid_until: u32,
/// }
///
/// #[derive(Structural, Debug, PartialEq)]
/// #[struc(no_trait, public, access="move")]
/// pub struct Message {
///     encoding: String,
///     contents: Vec<u8>,
/// }
///
/// // This macro generates the TryFromStructural impl based on the passed
/// // FromStructural implementation.
/// structural::z_impl_from_structural! {
///     impl[F, E, C] FromStructural<F> for Message
///     where[
///         // The bounds here would usually just be `F: Message_SI`
///         // (Message_SI being a trait generated by the Structural derive,
///         //  aliasing the accessor traits implemented by Message),
///         // but I decided to make this example different.
///         F: IntoField<FP!(encoding), Ty = C>,
///         F: IntoField<FP!(contents), Ty = E>,
///         C: Into<String>,
///         E: Into<Vec<u8>>,
///     ]{
///         fn from_structural(this){
///             let (encoding, contents) = this.into_fields(fp!(encoding, contents));
///             Self {
///                 encoding: encoding.into(),
///                 contents: contents.into(),
///             }
///         }
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! z_impl_from_structural {
    (
        impl[ $($impl_params:tt)* ] FromStructural<$from:ident> for $self:ty
        where [ $($where_preds:tt)* ]
        {
            fn from_structural($from_var:ident){
                $($code:tt)*
            }
        }
    ) => {
        impl< $($impl_params)* > $crate::pmr::FromStructural<$from> for $self
        where
            $($where_preds)*
        {
            fn from_structural($from_var: $from) -> Self {
                $($code)*
            }
        }

        impl< $($impl_params)*> $crate::pmr::TryFromStructural<$from> for $self
        where
            $($where_preds)*
        {
            type Error = $crate::pmr::Infallible;

            #[inline(always)]
            fn try_from_structural(
                $from_var: $from,
            ) -> Result<Self, $crate::pmr::TryFromError<$from,$crate::pmr::Infallible>> {
                Ok(<Self as $crate::pmr::FromStructural<$from>>::from_structural($from_var))
            }
        }
    };
}

/// For implementing [`TryFromStructural`],
/// and delegating the implementation of [`FromStructural`] to it.
///
/// The implementation of [`FromStructural`] inherits all the constraints of the
/// [`TryFromStructural`] impl.
///
/// In order to implement [`FromStructural`],
/// this macro assumes that the [`TryFromStructural`] implementation written by users:
///
/// - Matches on all the variants of the enum
///
/// - Returns `Ok` for all the variants of the enum that were matches by name.
///
/// [`FromStructural`]: ./convert/trait.FromStructural.html
/// [`TryFromStructural`]: ./convert/trait.TryFromStructural.html
///
/// # Example
///
/// ```rust
/// use structural::{
///     convert::{EmptyTryFromError, FromStructural, TryFromError, TryFromStructural},
///     Structural, StructuralExt, switch,
/// };
///
/// use std::cmp::Ordering;
///
///
/// assert_eq!(
///     EnumAAAA::Foo([9,8,7,6,5,4,3,2,1,0]).try_into_struc::<Variants>(),
///     Ok(Variants::Foo(9)),
/// );
///
/// assert_eq!(
///     EnumAAAA::Bar{heh: true}.try_into_struc::<Variants>(),
///     Ok(Variants::Bar),
/// );
///
/// assert_eq!(
///     EnumAAAA::Baz{foom: "hi", uh: Ordering::Less}.try_into_struc::<Variants>(),
///     Ok(Variants::Baz{foom: "hi"}),
/// );
///
/// assert_eq!(
///     EnumAAAA::Qux.try_into_struc::<Variants>(),
///     Err(TryFromError::with_empty_error(EnumAAAA::Qux)),
/// );
///
///
/// #[derive(Structural, Copy, Clone, Debug, PartialEq)]
/// #[struc(no_trait)]
/// enum EnumAAAA {
///     // This delegates the `*VariantField` accessor traits to the array,
///     // meaning that `.field_(fp!(::Foo.0))` would access the 0th element,
///     // `.field_(fp!(::Foo.4))` would access the 4th element of the array,
///     // etcetera.
///     //
///     // If this enum had a `EnumAAAA_SI` trait alias (generated by the `Structural` derive),
///     // this variant would only have the `IsVariant<TS!(Foo)>` bound in the trait alias.
///     #[struc(newtype)]
///     Foo([u8;10]),
///     Bar{ heh: bool },
///     Baz {
///         foom: &'static str,
///         uh: Ordering,
///     },
///     Qux,
/// }
///  
/// #[derive(Structural, Copy, Clone, Debug, PartialEq)]
/// enum Variants {
///     Foo(u8),
///     Bar,
///     Baz { foom: &'static str },
/// }
///  
///
/// structural::z_impl_try_from_structural_for_enum!{
///     impl[F] TryFromStructural<F> for Variants
///     where[
///         // `Variants_SI` was generated by the `Structural` derive for `Variants`
///         // aliasing its accessor trait impls,
///         // and allows `F` to have more variants than `Foo`,`Bar`,and `Baz`.
///         F: Variants_SI,
///     ]{
///         type Error = EmptyTryFromError;
///
///         fn try_from_structural(this) {
///             switch! {this;
///                 Foo(x) => Ok(Self::Foo(x)),
///                 Bar => Ok(Self::Bar),
///                 Baz{foom} => Ok(Self::Baz{foom}),
///                 _ => Err(TryFromError::with_empty_error(this)),
///             }
///         }
///     }
///
///     // `Variants_ESI` is like `Variants_SI` with the additional requirement that `F`
///     // only has the `Foo`,`Bar`,and `Baz` variants.
///     FromStructural
///     where[ F: Variants_ESI, ]
/// }
///
/// ```
#[macro_export]
macro_rules! z_impl_try_from_structural_for_enum {
    (
        impl[ $($impl_params:tt)* ] TryFromStructural<$from:ident> for $self:ty
        where [ $($where_preds:tt)* ]
        {
            type Error= $err:ty;
            fn try_from_structural($from_var:ident){
                $($code:tt)*
            }
        }

        FromStructural
        where [ $($from_where_preds:tt)* ]
    ) => {
        impl< $($impl_params)* > $crate::pmr::FromStructural<$from> for $self
        where
            $($where_preds)*
            $($from_where_preds)*
        {
            #[inline]
            fn from_structural(from_var: $from) -> Self {
                let res=<Self as $crate::pmr::TryFromStructural<$from>>::try_from_structural(
                    from_var
                );
                match res {
                    Ok(x) => x,
                    Err(e) => unreachable!(
                        "expected type to implement `TryFromStructural::try_from_structural`
                         such that it doesn't return an error in `FromStructural`.\n\
                        type:\n\t{}\n\
                        error:\n\t{}\n",
                        std::any::type_name::<Self>(),
                        e.error,
                    ),
                }
            }
        }

        impl< $($impl_params)*> $crate::pmr::TryFromStructural<$from> for $self
        where
            $($where_preds)*
        {
            type Error = $err;

            #[inline(always)]
            fn try_from_structural(
                $from_var: $from,
            ) -> Result<Self, $crate::pmr::TryFromError<$from,$err>> {
                $($code)*
            }
        }
    };
}
