/*!
Contains type-level lists,and related items
*/

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use crate::type_level::collection_traits::{
    Append, AppendOut, Flatten, PushBack, PushBackOut, ToTList, ToTListOut,
};

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

/// A type-level non-empty list.
pub struct TList<Curr, Rem>(PhantomData<fn() -> (Curr, Rem)>);

/// A type-level empty list.
#[derive(Debug, Copy, Clone)]
pub struct TNil;

////////////////////////////////////////////////////////////////////////////////

impl<Curr, Rem> core_extensions::ConstDefault for TList<Curr, Rem> {
    const DEFAULT: Self = TList(PhantomData);
}

impl<Curr, Rem> Debug for TList<Curr, Rem> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TList").finish()
    }
}

impl<Curr, Rem> Copy for TList<Curr, Rem> {}

impl<Curr, Rem> Clone for TList<Curr, Rem> {
    fn clone(&self) -> Self {
        TList::NEW
    }
}

impl<Curr, Rem> TList<Curr, Rem> {
    /// Constructs this list.
    pub const NEW: Self = TList(PhantomData);
}

/////////////////////////////////////////////////////////////////////////////////

impl<T, Rem> ToTList for TList<T, Rem> {
    type Output = Self;
}

impl ToTList for TNil {
    type Output = Self;
}

/////////////////////////////////////////////////////////////////////////////////

impl core_extensions::ConstDefault for TNil {
    const DEFAULT: Self = TNil;
}

impl TNil {
    /// Constructs this empty list.
    pub const NEW: Self = TNil;
}

////////////////////////////////////////////////////////////////////////////////

impl<Current, Rem, Elem> PushBack<Elem> for TList<Current, Rem>
where
    Rem: PushBack<Elem>,
{
    type Output = TList<Current, PushBackOut<Rem, Elem>>;
}

impl<Elem> PushBack<Elem> for TNil {
    type Output = TList<Elem, TNil>;
}

////////////////////////////////////////////////////////////////////////////////

impl<T, Rem, T2, Rem2> Append<TList<T2, Rem2>> for TList<T, Rem>
where
    Rem: Append<TList<T2, Rem2>>,
{
    type Output = TList<T, AppendOut<Rem, TList<T2, Rem2>>>;
}

impl<T, Rem> Append<TNil> for TList<T, Rem> {
    type Output = TList<T, Rem>;
}

impl<T, Rem> Append<TList<T, Rem>> for TNil {
    type Output = TList<T, Rem>;
}

impl Append<TNil> for TNil {
    type Output = TNil;
}

////////////////////////////////////////////////////////////////////////////////

impl Flatten for TNil {
    type Output = TNil;
}
impl<Curr, Rem, Out> Flatten for TList<Curr, Rem>
where
    (): FlattenOutImpl<Rem, Curr, Output = Out>,
{
    type Output = Out;
}

#[doc(hidden)]
pub trait FlattenOutImpl<Outer, Inner> {
    type Output;
}

impl<List> FlattenOutImpl<TNil, List> for ()
where
    List: ToTList,
{
    type Output = ToTListOut<List>;
}

impl<Curr, Rem, Out> FlattenOutImpl<TList<Curr, Rem>, TNil> for ()
where
    Curr: ToTList,
    (): FlattenOutImpl<Rem, ToTListOut<Curr>, Output = Out>,
{
    type Output = Out;
}

impl<CurrI, RemI, CurrO, RemO, Out> FlattenOutImpl<TList<CurrO, RemO>, TList<CurrI, RemI>> for ()
where
    RemI: ToTList,
    (): FlattenOutImpl<TList<CurrO, RemO>, ToTListOut<RemI>, Output = Out>,
{
    type Output = TList<CurrI, Out>;
}
