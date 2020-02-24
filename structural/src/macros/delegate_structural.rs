/**
This macro allows delegating the implementation of the accessor traits.

This macro delegates the implementation of those traits for all fields,
it doesn't provide a way to do so for only a list of fields.

# Safety

In order to use this macro soundly,
you must ensure that side effects (including mutation) do not happen while
getting the delegated-to variable,
and that the delegated-to variable has a consistent value when
methods from `structural` traits are called in a sequence
(with no method calls from non-`structural`-traits in between).

The unsafety of implementing GetFieldMutImpl with this macro comes from the methods
used to do multiple mutable borrows.

You must ensure that the variable that you delegate GetFieldImpl to is the same as the one
you delegate GetFieldMutImpl to,
as well as ensuring that there are no other impls of the GetFieldMutImpl trait
borrowing from the same variable mutably.


# Example with all syntax

```rust
use structural::unsafe_delegate_structural_with;
use structural::mut_ref::MutRef;

# trait Trait{}
# impl<T> Trait for T{}

struct Foo<T>{
    value:T
}

unsafe_delegate_structural_with!{
    impl[T,] Foo<T>
    // This where clause is required syntax
    where[
        T:Trait,
    ]

    // This is the identifier used for `self` in the blocks below.
    self_ident=this;

    // An optional macro argument that tells it how to specialize pointer methods.
    //
    // `specialization_params(Sized);` is the default when this the parameter is not passed,
    // it means that no specialization is used,always requiring `Self:Sized`.
    specialization_params(Sized);

    // This means that the type is `?Sized` and not specialization is used,
    // this may be slower in debug builds because this always uses a
    // function pointer call in raw-pointer-taking methods.
    // specialization_params(?Sized);

    // This means that the type is `?Sized` by default.
    // The `cfg(anything)` argument enables specialization conditionally,
    // with a default impl for `Self:?Sized` which may be slower in debug builds,
    // because this always uses a function pointer call in raw-pointer methods.
    // It specializes on `Self:Sized` to remove the overhead of raw-pointer methods.
    // specialization_params(cfg(anything));


    // This is the type of the variable we delegate to,
    // this is required because Rust doesn't have a `typeof`/`decltype` construct.
    delegating_to_type=T;

    // `field_name` is the name for a `PhantomData` parameter in
    // `GetFieldMutImpl::get_field_raw_mut`
    // (usable from `as_delegating_raw{}` in this macro),
    // with the name of the field being accessed
    //
    // `FieldName` is the name of the type parameter that represents the
    // name of the field being accessed.
    field_name_param=( field_name : FieldName );

    // This block of code is used to get the reference to the delegating variable
    // in GetFieldImpl.
    GetFieldImpl {
        &this.value
    }

    // This block of code is used to get a mutable reference to the delegating variable
    // in GetFieldMutImpl
    //
    // This is `unsafe` because this block must always evaluate to a mutable reference
    // for the same variable,
    // and it must not be the same variable as other implementations of the GetFieldMutImpl trait
    unsafe GetFieldMutImpl
    where [
        // This is an optional where clause
        // The last where predicate must have a trailing comma.
        T:Trait,
    ]{
        &mut this.value
    }

    // This gets a raw mutable pointer to the variable this delegates to.
    as_delegating_raw{
        &mut (*this).value as *mut T
    }

    // This block of code is used to get the delegating variable by value in IntoFieldImpl.
    IntoFieldImpl
    where [
        // This is an optional where clause
        // The last where predicate must have a trailing comma.
        T:Trait,
    ]{
        this.value
    }
}
```

# Example

This example is of a type wrapping a `ManuallyDrop<T>`,delegating to the `T` inside it.

```rust
use std::{
    fmt::Debug,
    mem::ManuallyDrop,
};

use structural::{GetFieldExt,GetFieldMutImpl,unsafe_delegate_structural_with,make_struct,fp};
use structural::mut_ref::MutRef;

struct Bar<T>{
    value:ManuallyDrop<T>
}

impl<T> Bar<T>{
    pub fn new(value:T)->Self{
        Self{
            value:ManuallyDrop::new(value)
        }
    }
}


unsafe_delegate_structural_with!{
    // You must write a trailing comma.
    impl[T,] Bar<T>
    where[
        // This requires T to implement Clone
        // for `Bar<T>` to implement the accessor traits
        T:Clone,
    ]
    self_ident=this;
    specialization_params(Sized);
    delegating_to_type=T;
    field_name_param=( field_name : FieldName );

    GetFieldImpl {
#       // This ensures that the `T:Clone` bound is put on the impl block.
#       T::clone;
        &*this.value
    }

    unsafe GetFieldMutImpl
    where [T:Debug,]
    {
#       // This ensures that the `T:Clone+Debug` bounds are put on the impl block.
#       T::clone;
#       <T as Debug>::fmt;
        &mut *this.value
    }
    as_delegating_raw{
        &mut (*this).value as *mut ManuallyDrop<T> as *mut T
    }


    IntoFieldImpl
    where [T:Debug,]
    {
#       // This ensures that the `T:Clone+Debug` bounds are put on the impl block.
#       T::clone;
#       <T as Debug>::fmt;
        ManuallyDrop::into_inner(this.value)
    }
}

{
    let mut bar=Bar::new((2,3,5,8,13));
    assert_eq!(
        bar.fields(fp!(4,3,2,1,0)),
        ( &13, &8, &5, &3, &2 )
    );

    assert_eq!(
        bar.fields_mut(fp!(1,2,3,4)),
        ( &mut 3, &mut 5, &mut 8, &mut 13 )
    );

    assert_eq!(bar.into_field(fp!(1)),3);
}

{
    let mut bar=Bar::new(make_struct!{
        #![derive(Clone,Debug)] //This derives Clone and Debug for the anonymous struct

        a:"hello",
        b:"world",
        c:"tour",
    });
    assert_eq!(
        bar.fields(fp!(a,b,c)),
        ( &"hello", &"world", &"tour" )
    );

    assert_eq!(
        bar.fields_mut(fp!(c,b,a)),
        ( &mut"tour", &mut"world", &mut"hello" )
    );

    assert_eq!( bar.into_field(fp!(c)), "tour" );
}


```


*/
#[macro_export]
macro_rules! unsafe_delegate_structural_with {
    (
        impl $impl_params:tt $self:ty
        where $where_clause:tt
        self_ident=$this:ident;
        $( specialization_params($($raw_mut_impl:tt)*); )?
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        $($rest:tt)*
    ) => (
        $crate::unsafe_delegate_structural_with_inner!{
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            specialization_params( $( $($raw_mut_impl)* )? );
            delegating_to_type=$delegating_to_type;
            field_name_param=( $fname_var : $fname_ty );

            $($rest)*
        }
    )
}

#[macro_export]
#[doc(hidden)]
macro_rules! unsafe_delegate_structural_with_inner {
    (
        impl $impl_params:tt $self:ty
        where $where_clause:tt
        self_ident=$this:ident;
        specialization_params $raw_mut_impl:tt;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        GetFieldImpl $get_field_closure:block
        $(
            unsafe GetFieldMutImpl
            $( where[ $($mut_where_clause:tt)* ] )?
            $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )?
        $(
            IntoFieldImpl
            $( where[ $($into_where_clause:tt)* ] )?
            $into_field_closure:block
        )?
    ) => (

        $crate::unsafe_delegate_structural_with_inner!{
            inner-structural;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            delegating_to_type=$delegating_to_type;
            field_name_param=( $fname_var : $fname_ty );
            GetFieldImpl $get_field_closure
        }

        $crate::unsafe_delegate_structural_with_inner!{
            inner;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            delegating_to_type=$delegating_to_type;
            field_name_param=( $fname_var : $fname_ty );
            GetFieldImpl $get_field_closure
        }

        $(
            $crate::unsafe_delegate_structural_with_inner!{
                inner;
                impl $impl_params $self
                where $where_clause
                where[ $( $($mut_where_clause)* )? ]
                self_ident=$this;
                specialization_params $raw_mut_impl;
                delegating_to_type=$delegating_to_type;
                field_name_param=( $fname_var : $fname_ty );

                unsafe GetFieldMutImpl $unsafe_get_field_mut_closure
                as_delegating_raw $as_field_mutref_closure
            }
        )?

        $(
            $crate::unsafe_delegate_structural_with_inner!{
                inner;
                impl $impl_params $self
                where $where_clause
                where [ $( $($into_where_clause)* )? ]
                self_ident=$this;
                delegating_to_type=$delegating_to_type;
                field_name_param=( $fname_var : $fname_ty );
                IntoFieldImpl $into_field_closure
            }
        )?
    );
    (
        inner-structural;
        impl[$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        GetFieldImpl $get_field_closure:block
    )=>{
        impl<$($impl_params)*> $crate::Structural for $self
        where
            $delegating_to_type: $crate::Structural,
            $($where_clause)*
        {
            const FIELDS: &'static $crate::structural_trait::FieldInfos={
                <$delegating_to_type as $crate::Structural>::FIELDS
            };
        }
    };
    (inner;
        impl [$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        GetFieldImpl $get_field_closure:block
    )=>{
        impl<$($impl_params)*> $crate::IsStructural for $self
        where
            $delegating_to_type: $crate::IsStructural,
            $($where_clause)*
        {}

        unsafe impl<$($impl_params)* _V>
            $crate::pmr::IsVariant<$crate::pmr::TStr<_V>>
        for $self
        where
            $delegating_to_type: $crate::pmr::IsVariant<$crate::pmr::TStr<_V>>,
            $($where_clause)*
        {
            fn is_variant_(&self,name:$crate::pmr::TStr<_V>)->bool{
                let $this=self;
                let field:&$delegating_to_type=$get_field_closure;
                $crate::pmr::IsVariant::is_variant_(field,name)
            }
        }

        unsafe impl<$($impl_params)*> $crate::pmr::VariantCount for $self
        where
            $delegating_to_type: $crate::pmr::VariantCount,
            $($where_clause)*
        {
            type Count=$crate::pmr::VariantCountOut<$delegating_to_type>;
        }

        // This is defined separately from `unsafe_delegate_variant_field!`
        // because additional bounds might be added to GetFieldImpl.
        //
        unsafe impl<$($impl_params)* _V,_F>
            $crate::pmr::GetVariantFieldImpl<$crate::pmr::TStr<_V>,_F>
        for $self
        where
            $delegating_to_type:
                $crate::IsStructural+
                $crate::pmr::GetVariantFieldImpl<$crate::pmr::TStr<_V>,_F>,
            $($where_clause)*
        {}

        impl<$($impl_params)* $fname_ty> $crate::FieldType<$fname_ty> for $self
        where
            $delegating_to_type: $crate::IsStructural + $crate::FieldType<$fname_ty>,
            $($where_clause)*
        {
            type Ty=$crate::GetFieldType<$delegating_to_type, $fname_ty>;
        }

        impl<$($impl_params)* $fname_ty,__P>
            $crate::GetFieldImpl< $fname_ty, __P>
            for $self
        where
            $delegating_to_type: $crate::IsStructural + $crate::GetFieldImpl<$fname_ty,__P>,
            $($where_clause)*
        {
            type Err=$crate::field_traits::GetFieldErr<$delegating_to_type, $fname_ty, __P>;

            #[inline(always)]
            fn get_field_(
                &self,
                $fname_var: $fname_ty,
                __param:__P,
            )->Result<
                &$crate::GetFieldType<Self,$fname_ty>,
                $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
            >{
                let $this=self;
                let field:&$delegating_to_type=$get_field_closure;
                $crate::GetFieldImpl::get_field_(field,$fname_var,__param)
            }
        }
    };
    (inner;
        impl [$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        where [$($mut_where_clause:tt)*]
        self_ident=$this:ident;
        specialization_params($($raw_mut_impl:tt)*);
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        unsafe GetFieldMutImpl $unsafe_get_field_mut_closure:block
        as_delegating_raw $as_field_mutref_closure:block
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-0;

            (
                impl [$($impl_params)*] $self
                where [$($where_clause)*]
                where [$($mut_where_clause)*]
                self_ident=$this;
                specialization_params($($raw_mut_impl)*);
                delegating_to_type=$delegating_to_type;
                field_name_param=( $fname_var : $fname_ty );

                unsafe GetFieldMutImpl $unsafe_get_field_mut_closure
                as_delegating_raw $as_field_mutref_closure
            )
        }
    };
    (inner-mut-0; $inner_mut_stuff:tt )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-1;

            $inner_mut_stuff
            $inner_mut_stuff
        }
    };
    (inner-mut-1;
        (
            impl [$($impl_params:tt)*] $self:ty
            where [$($where_clause:tt)*]
            where [$($mut_where_clause:tt)*]
            self_ident=$this:ident;
            specialization_params( $(Sized)? );
            delegating_to_type=$delegating_to_type:ty;
            field_name_param=( $fname_var:ident : $fname_ty:ident );

            unsafe GetFieldMutImpl $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        $inner_mut_stuff:tt
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-2;

            $inner_mut_stuff

            fn(
                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    $this:*mut (),
                    $fname_var: $fname_ty,
                    __param:__P,
                )->Result<
                    *mut $crate::GetFieldType<Self,$fname_ty>,
                    $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
                >
                where
                    Self:Sized
                {
                    let $this=$this as *mut Self;
                    let $this:*mut $delegating_to_type=
                        $as_field_mutref_closure;

                    <$delegating_to_type as
                        $crate::GetFieldMutImpl<$fname_ty,__P>
                    >::get_field_raw_mut( $this as *mut (),$fname_var,__param )
                }
            )

            impl()
        }
    };
    (inner-mut-1;
        (
            impl [$($impl_params:tt)*] $self:ty
            where [$($where_clause:tt)*]
            where [$($mut_where_clause:tt)*]
            self_ident=$this:ident;
            specialization_params( ?Sized );
            delegating_to_type=$delegating_to_type:ty;
            field_name_param=( $fname_var:ident : $fname_ty:ident );

            unsafe GetFieldMutImpl $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        $inner_mut_stuff:tt
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-2;

            $inner_mut_stuff

            fn(
                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    $this:*mut (),
                    $fname_var: $fname_ty,
                    __param:__P,
                )->Result<
                    *mut $crate::GetFieldType<Self,$fname_ty>,
                    $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
                >
                where
                    Self:Sized
                {
                    let $this=$this as *mut Self;
                    let $this:*mut $delegating_to_type=
                        $as_field_mutref_closure;

                    let func=<
                        $delegating_to_type as
                        $crate::GetFieldMutImpl<$fname_ty,__P>
                    >::get_field_raw_mut_func(&*$this);
                    func( $this as *mut (),$fname_var,__param )
                }
            )

            impl()
        }
    };
    (inner-mut-1;
        (
            impl [$($impl_params:tt)*] $self:ty
            where [$($where_clause:tt)*]
            where [$($mut_where_clause:tt)*]
            self_ident=$this:ident;
            specialization_params( specialize_cfg( $($specialize_cfg:tt)* ) );
            delegating_to_type=$delegating_to_type:ty;
            field_name_param=( $fname_var:ident : $fname_ty:ident );

            unsafe GetFieldMutImpl $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        $inner_mut_stuff:tt
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-2;

            $inner_mut_stuff


            fn(
                $crate::default_if!{
                    #[inline(always)]
                    cfg(all($($specialize_cfg)*))
                    unsafe fn get_field_raw_mut(
                        $this:*mut (),
                        $fname_var: $fname_ty,
                        __param:__P,
                    )->Result<
                        *mut $crate::GetFieldType<Self,$fname_ty>,
                        $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
                    >
                    where
                        Self:Sized
                    {
                        let $this=$this as *mut Self;
                        let $this:*mut $delegating_to_type=
                            $as_field_mutref_closure;

                        let func=<
                            $delegating_to_type as
                            $crate::GetFieldMutImpl<$fname_ty,__P>
                        >::get_field_raw_mut_func(&*$this);
                        func( $this as *mut (),$fname_var,__param )
                    }
                }
            )

            impl(
                #[cfg(all($($specialize_cfg)*))]
                unsafe impl<$($impl_params)* $fname_ty,__P>
                    $crate::GetFieldMutImpl< $fname_ty,__P>
                    for $self
                where
                    $delegating_to_type:
                        Sized +
                        $crate::IsStructural+
                        $crate::GetFieldMutImpl<$fname_ty,__P>,
                    $($mut_where_clause)*
                    $($where_clause)*
                {
                    unsafe fn get_field_raw_mut(
                        $this:*mut (),
                        $fname_var: $fname_ty,
                        __param:__P,
                    )->Result<
                        *mut $crate::GetFieldType<Self,$fname_ty>,
                        $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
                    >
                    where
                        Self:Sized
                    {
                        let $this=$this as *mut Self;
                        let $this:*mut $delegating_to_type=
                            $as_field_mutref_closure;

                        <$delegating_to_type as
                            $crate::GetFieldMutImpl<$fname_ty,__P>
                        >::get_field_raw_mut( $this as *mut (),$fname_var,__param )
                    }
                }
            )
        }
    };
    (inner-mut-2;
        (
            impl [$($impl_params:tt)*] $self:ty
            where [$($where_clause:tt)*]
            where [$($mut_where_clause:tt)*]
            self_ident=$this:ident;
            specialization_params($($raw_mut_impl:tt)*);
            delegating_to_type=$delegating_to_type:ty;
            field_name_param=( $fname_var:ident : $fname_ty:ident );

            unsafe GetFieldMutImpl $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        fn( $($raw_ptr_fn:tt)* )
        impl( $($raw_ptr_impl:tt)* )
    )=>{

        // This is defined separately from `unsafe_delegate_variant_field!`
        // because additional bounds might be added to GetFieldMutImpl.
        unsafe impl<$($impl_params)* _V,_F>
            $crate::pmr::GetVariantFieldMutImpl<$crate::pmr::TStr<_V>,_F>
        for $self
        where
            $delegating_to_type:
                $crate::IsStructural +
                $crate::pmr::GetVariantFieldMutImpl<$crate::pmr::TStr<_V>,_F>,
            $($where_clause)*
            $($mut_where_clause)*
        {}

        unsafe impl<$($impl_params)* $fname_ty,__P>
            $crate::GetFieldMutImpl<$fname_ty,__P>
            for $self
        where
            $self: Sized,
            $delegating_to_type:
                $crate::IsStructural +
                $crate::GetFieldMutImpl<$fname_ty,__P>,
            $($where_clause)*
            $($mut_where_clause)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self,
                $fname_var: $fname_ty,
                __param:__P,
            )->Result<
                &mut $crate::GetFieldType<Self,$fname_ty>,
                $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
            >{
                let $this=self;
                let field:&mut $delegating_to_type=$unsafe_get_field_mut_closure;
                <$delegating_to_type as
                    $crate::GetFieldMutImpl<_,_>
                >::get_field_mut_(field,$fname_var,__param)
            }

            $($raw_ptr_fn)*

            #[inline(always)]
            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                $fname_ty,
                __P,
                $crate::GetFieldType<Self,$fname_ty>,
                $crate::pmr::GetFieldErr<Self,$fname_ty,__P>,
            >{
                <Self as $crate::GetFieldMutImpl<$fname_ty,__P>>::get_field_raw_mut
            }
        }

        $($raw_ptr_impl)*
    };
    (inner;
        impl [$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        where [$($into_where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        IntoFieldImpl $into_field_closure:block
    )=>{

        // This is defined separately from `unsafe_delegate_variant_field!`
        // because additional bounds might be added to IntoFieldImpl.
        unsafe impl<$($impl_params)* _V,_F>
            $crate::pmr::IntoVariantFieldImpl<$crate::pmr::TStr<_V>,_F>
        for $self
        where
            $delegating_to_type:
                Sized+
                $crate::IsStructural +
                $crate::pmr::IntoVariantFieldImpl<$crate::pmr::TStr<_V>,_F>,
            $($into_where_clause)*
            $($where_clause)*
        {}

        impl<$($impl_params)* $fname_ty,__P>
            $crate::IntoFieldImpl< $fname_ty,__P>
            for $self
        where
            $delegating_to_type:
                Sized+
                $crate::IsStructural+
                $crate::IntoFieldImpl<$fname_ty,__P>,
            $($into_where_clause)*
            $($where_clause)*
        {
            #[inline(always)]
            fn into_field_(
                self,
                $fname_var: $fname_ty,
                __param:__P,
            )->Result<$crate::GetFieldType<$delegating_to_type,$fname_ty>,Self::Err>{
                let $this=self;
                let field:$delegating_to_type=$into_field_closure;
                $crate::IntoFieldImpl::<$fname_ty,__P>::into_field_(field,$fname_var,__param)
            }

            $crate::z_impl_box_into_field_method!{
                $fname_ty,
                __P,
                $crate::GetFieldType<$delegating_to_type,$fname_ty>,
                $crate::pmr::GetFieldErr<$delegating_to_type,$fname_ty,__P>,
            }
        }
    };
}

//////////////////////////////////////////////////////////////////////////////
