/*!
Contains type-level lists,and related items
*/

use std_::{
    fmt::{self, Debug},
    marker::PhantomData,
};

use crate::type_level::collection_traits::{
    Append, Append_, Flatten_, PushBack, PushBack_, ToTList, ToTList_,
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

unsafe impl<Curr, Rem> core_extensions::MarkerType for TList<Curr, Rem> {}

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

impl<T, Rem> ToTList_ for TList<T, Rem> {
    type Output = Self;
}

impl ToTList_ for TNil {
    type Output = Self;
}

/////////////////////////////////////////////////////////////////////////////////

unsafe impl core_extensions::MarkerType for TNil {}

impl TNil {
    /// Constructs this empty list.
    pub const NEW: Self = TNil;
}

////////////////////////////////////////////////////////////////////////////////

impl<Current, Rem, Elem> PushBack_<Elem> for TList<Current, Rem>
where
    Rem: PushBack_<Elem>,
{
    type Output = TList<Current, PushBack<Rem, Elem>>;
}

impl<Elem> PushBack_<Elem> for TNil {
    type Output = TList<Elem, TNil>;
}

////////////////////////////////////////////////////////////////////////////////

impl<T, Rem, T2, Rem2> Append_<TList<T2, Rem2>> for TList<T, Rem>
where
    Rem: Append_<TList<T2, Rem2>>,
{
    type Output = TList<T, Append<Rem, TList<T2, Rem2>>>;
}

impl<T, Rem> Append_<TNil> for TList<T, Rem> {
    type Output = TList<T, Rem>;
}

impl<T, Rem> Append_<TList<T, Rem>> for TNil {
    type Output = TList<T, Rem>;
}

impl Append_<TNil> for TNil {
    type Output = TNil;
}

////////////////////////////////////////////////////////////////////////////////

impl Flatten_ for TNil {
    type Output = TNil;
}
impl<Curr, Rem, Out> Flatten_ for TList<Curr, Rem>
where
    (): FlattenImpl<Rem, Curr, Output = Out>,
{
    type Output = Out;
}

#[doc(hidden)]
pub trait FlattenImpl<Outer, Inner> {
    type Output;
}

impl<List> FlattenImpl<TNil, List> for ()
where
    List: ToTList_,
{
    type Output = ToTList<List>;
}

impl<Curr, Rem, Out> FlattenImpl<TList<Curr, Rem>, TNil> for ()
where
    Curr: ToTList_,
    (): FlattenImpl<Rem, ToTList<Curr>, Output = Out>,
{
    type Output = Out;
}

impl<CurrI, RemI, CurrO, RemO, Out> FlattenImpl<TList<CurrO, RemO>, TList<CurrI, RemI>> for ()
where
    RemI: ToTList_,
    (): FlattenImpl<TList<CurrO, RemO>, ToTList<RemI>, Output = Out>,
{
    type Output = TList<CurrI, Out>;
}
