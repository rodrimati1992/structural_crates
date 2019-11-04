/*!
Contains type-level lists,and related items
*/

use std_::{
    fmt::{self,Debug},
    marker::PhantomData,
};

/// A type-level non-empty list.
pub struct TList<Curr,Rem>(PhantomData<fn()->(Curr,Rem)>);

/// A type-level empty list.
#[derive(Debug,Copy,Clone)]
pub struct TNil;

////////////////////////////////////////////////////////////////////////////////

unsafe impl<Curr,Rem> core_extensions::MarkerType for TList<Curr,Rem> {}

impl<Curr,Rem> Debug for TList<Curr,Rem> {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        f.debug_struct("TList").finish()
    }
}

impl<Curr,Rem> Copy for TList<Curr,Rem> {}

impl<Curr,Rem> Clone for TList<Curr,Rem> {
    fn clone(&self)->Self{
        TList::NEW
    }
}

impl<Curr,Rem> TList<Curr,Rem> {
    /// Constructs this list.
    pub const NEW:Self=TList(PhantomData);
}


//////////////////////6//////////////////////////////////////////////////////////

unsafe impl core_extensions::MarkerType for TNil{}

impl TNil{
    /// Constructs this empty list.
    pub const NEW:Self=TNil;
}

////////////////////////////////////////////////////////////////////////////////
