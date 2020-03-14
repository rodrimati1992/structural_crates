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

You must ensure that the variable that you delegate Get*Field to is the same as the one
you delegate Get*FieldMut to,
as well as ensuring that there are no other impls of the GetFieldMut trait
borrowing from the same variable mutably.

### GetFieldMut

The unsafety of implementing `GetFieldMut` with this macro comes from the methods
used to do multiple mutable borrows.

### GetVariantFieldMut

The unsafety of implementing `GetVariantFieldMut` with this macro comes from the methods
used to do multiple mutable borrows,
as well as the requirement that the IsVariant and the GetVariantFieldMut impls must
agree on what the current variant is.

###  general



# Example with all syntax

```rust
use structural::unsafe_delegate_structural_with;

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
    //
    specialization_params(Sized);

    // This means that the type is `?Sized` and not specialization is used,
    // this may be slower in debug builds because this always uses a
    // function pointer call in raw-pointer-taking methods.
    //
    // specialization_params(?Sized);

    // This means that the type is `?Sized` by default.
    // The `cfg(anything)` argument enables specialization conditionally,
    //
    // When specialization is disables theres only a default impl for `Self:?Sized`
    // which may be slower in debug builds,
    // because this uses a function pointer call in raw-pointer methods.
    //
    // When specialization is enabled,the impl is specializes on `Self:Sized`
    // to remove the overhead of raw-pointer methods.
    //
    // specialization_params(cfg(anything));


    // This is the type of the variable we delegate to,
    // this is required because Rust doesn't have a `typeof`/`decltype` construct.
    delegating_to_type=T;

    // This block of code is used to get the reference to the delegating variable
    // in GetField.
    GetField {
        &this.value
    }

    // This block of code is used to get a mutable reference to the delegating variable
    // in GetFieldMut
    //
    // This is `unsafe` because this block must always evaluate to a mutable reference
    // for the same variable,
    // and it must not be the same variable as other implementations of the GetFieldMut trait.
    //
    unsafe GetFieldMut
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

    // This block of code is used to get the delegating variable by value in IntoField.
    IntoField
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

use structural::{GetFieldExt,GetFieldMut,unsafe_delegate_structural_with,make_struct,fp};

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

    GetField {
#       // This ensures that the `T:Clone` bound is put on the impl block.
#       T::clone;
        &*this.value
    }

    unsafe GetFieldMut
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


    IntoField
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
        $( #[doc=$doc:expr] )*
        impl $impl_params:tt $self:ty
        where $where_clause:tt
        self_ident=$this:ident;
        $( specialization_params($($raw_mut_impl:tt)*); )?
        delegating_to_type=$delegating_to_type:ty;

        $($rest:tt)*
    ) => (
        $crate::unsafe_delegate_structural_with_inner!{
            $( #[doc=$doc] )*
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            specialization_params( $( $($raw_mut_impl)* )? );
            delegating_to_type=$delegating_to_type;

            $($rest)*
        }
    )
}

#[macro_export]
#[doc(hidden)]
macro_rules! unsafe_delegate_structural_with_inner {
    (
        $( #[doc=$doc:expr] )*
        impl $impl_params:tt $self:ty
        where $where_clause:tt
        self_ident=$this:ident;
        specialization_params $raw_mut_impl:tt;
        delegating_to_type=$delegating_to_type:ty;

        GetField $get_field_closure:block
        $(
            unsafe GetFieldMut
            $( where[ $($mut_where_clause:tt)* ] )?
            $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )?
        $(
            IntoField
            $( where[ $($into_where_clause:tt)* ] )?
            $into_field_closure:block
        )?
    ) => (

        $crate::unsafe_delegate_structural_with_inner!{
            inner-structural;
            $( #[doc=$doc] )*
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            delegating_to_type=$delegating_to_type;
            GetField $get_field_closure
        }

        $crate::unsafe_delegate_structural_with_inner!{
            inner;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            delegating_to_type=$delegating_to_type;
            GetField $get_field_closure
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

                unsafe GetFieldMut $unsafe_get_field_mut_closure
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
                IntoField $into_field_closure
            }
        )?
    );
    (
        inner-structural;
        $( #[doc=$doc:expr] )*
        impl[$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;

        GetField $get_field_closure:block
    )=>{
        $( #[doc=$doc] )*
        impl<$($impl_params)*> $crate::Structural for $self
        where
            $delegating_to_type: $crate::Structural,
            $($where_clause)*
        {}
    };
    (inner;
        impl [$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;

        GetField $get_field_closure:block
    )=>{
        unsafe impl<$($impl_params)* __V>
            $crate::pmr::IsVariant<$crate::TStr<__V>>
        for $self
        where
            $delegating_to_type: $crate::pmr::IsVariant<$crate::TStr<__V>>,
            $($where_clause)*
        {
            fn is_variant_(&self,name:$crate::TStr<__V>)->bool{
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

        // This is defined separately from `unsafe_delegate_vfield!`
        // because additional bounds might be added to GetField.
        //
        unsafe impl<$($impl_params)* __V,__F,__Ty>
            $crate::GetVariantField<$crate::TStr<__V>,__F>
        for $self
        where
            $delegating_to_type:
                $crate::GetVariantField<$crate::TStr<__V>,__F,Ty=__Ty>,
            $($where_clause)*
        {
            #[inline(always)]
            fn get_vfield_(
                &self,
                vname: $crate::TStr<__V>,
                fname: __F,
            ) -> Option<&$crate::GetVariantFieldType<
                    $delegating_to_type,
                    $crate::TStr<__V>,
                    __F
                >>
            {
                let $this=self;
                let field:&$delegating_to_type=$get_field_closure;
                $crate::GetVariantField::get_vfield_(field,vname,fname)
            }
        }

        impl<$($impl_params)* NP,__Ty> $crate::FieldType<NP> for $self
        where
            $delegating_to_type: $crate::FieldType<NP,Ty=__Ty>,
            $($where_clause)*
        {
            type Ty=$crate::GetFieldType<$delegating_to_type, NP>;
        }

        impl<$($impl_params)* __F,__Ty>
            $crate::GetField< __F>
            for $self
        where
            $delegating_to_type: $crate::GetField<__F,Ty=__Ty>,
            $($where_clause)*
        {
            #[inline(always)]
            fn get_field_(
                &self,
                fname: __F,
            )->&__Ty{
                let $this=self;
                let field:&$delegating_to_type=$get_field_closure;
                $crate::GetField::get_field_(field,fname)
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

        unsafe GetFieldMut $unsafe_get_field_mut_closure:block
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

                unsafe GetFieldMut $unsafe_get_field_mut_closure
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

            unsafe GetFieldMut $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        $inner_mut_stuff:tt
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-2;

            $inner_mut_stuff

            struct_fn(
                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    $this:*mut  (),
                    fname: __F,
                )->*mut __Ty
                where
                    Self:Sized
                {
                    let $this=$this as *mut  Self;
                    let $this:*mut $delegating_to_type=
                        $as_field_mutref_closure;
                    <$delegating_to_type as
                        $crate::GetFieldMut<__F>
                    >::get_field_raw_mut( $this as *mut  (),fname )
                }
            )

            enum_fn(
                #[inline(always)]
                unsafe fn get_vfield_raw_mut_(
                    $this: *mut  (),
                    vname: $crate::TStr<__V>,
                    fname: __F,
                ) -> Option<$crate::pmr::NonNull<__Ty>>
                where
                    Self: Sized
                {
                    let $this=$this as *mut  Self;
                    let $this:*mut $delegating_to_type=
                        $as_field_mutref_closure;
                    <$delegating_to_type as
                        $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                    >::get_vfield_raw_mut_( $this as *mut  (),vname,fname)
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

            unsafe GetFieldMut $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        $inner_mut_stuff:tt
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-2;

            $inner_mut_stuff

            struct_fn(
                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    $this:*mut  (),
                    fname: __F,
                )->*mut __Ty
                where
                    Self:Sized
                {
                    let $this=$this as *mut  Self;
                    let $this:*mut $delegating_to_type=
                        $as_field_mutref_closure;
                    let func=<
                        $delegating_to_type as
                        $crate::GetFieldMut<__F>
                    >::get_field_raw_mut_fn(&*$this);
                    func( $this as *mut  (),fname )
                }
            )

            enum_fn(
                #[inline(always)]
                unsafe fn get_vfield_raw_mut_(
                    $this: *mut  (),
                    vname: $crate::TStr<__V>,
                    fname: __F,
                ) -> Option<$crate::pmr::NonNull<__Ty>>
                where
                    Self: Sized
                {
                    let $this=$this as *mut  Self;
                    let $this:*mut $delegating_to_type=
                        $as_field_mutref_closure;
                    let func=<
                        $delegating_to_type as
                        $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                    >::get_vfield_raw_mut_fn(&*$this);
                    func( $this as *mut  (),vname,fname )
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

            unsafe GetFieldMut $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        $inner_mut_stuff:tt
    )=>{
        $crate::unsafe_delegate_structural_with_inner!{
            inner-mut-2;

            $inner_mut_stuff


            struct_fn(
                #[inline(always)]
                unsafe fn get_field_raw_mut(
                    $this:*mut  (),
                    fname: __F,
                )->*mut __Ty
                where
                    Self:Sized
                {
                    <Self as
                        $crate::pmr::SpecGetFieldMut<__F>
                    >::get_field_raw_mut_inner(
                        $this,
                        fname,
                    )
                }
            )

            enum_fn(
                #[inline(always)]
                unsafe fn get_vfield_raw_mut_(
                    $this: *mut  (),
                    vname: $crate::TStr<__V>,
                    fname: __F,
                ) -> Option<$crate::pmr::NonNull<__Ty>>
                where
                    Self: Sized
                {
                    <Self as
                        $crate::pmr::SpecGetVariantFieldMut<$crate::TStr<__V>,__F>
                    >::get_vfield_raw_mut_inner(
                        $this,
                        vname,
                        fname,
                    )
                }
            )

            impl(
                unsafe impl<$($impl_params)* __F,__Ty>
                    $crate::pmr::SpecGetFieldMut< __F>
                    for $self
                where
                    $delegating_to_type: $crate::GetFieldMut<__F,Ty=__Ty>,
                    $($mut_where_clause)*
                    $($where_clause)*
                {
                    $crate::default_if!{
                        #[inline(always)]
                        cfg(all($($specialize_cfg)*))
                        unsafe fn get_field_raw_mut_inner(
                            $this:*mut  (),
                            fname: __F,

                        )->*mut __Ty
                        where
                            Self:Sized
                        {
                            let $this=$this as *mut  Self;
                            let $this:*mut $delegating_to_type=
                                $as_field_mutref_closure;
                            let func=<
                                $delegating_to_type as
                                $crate::GetFieldMut<__F>
                            >::get_field_raw_mut_fn(&*$this);
                            func( $this as *mut  (),fname )
                        }
                    }
                }

                #[cfg(all($($specialize_cfg)*))]
                unsafe impl<$($impl_params)* __F,__Ty>
                    $crate::pmr::SpecGetFieldMut< __F>
                    for $self
                where
                    $delegating_to_type:
                        Sized +
                        $crate::GetFieldMut<__F,Ty=__Ty>,
                    $($mut_where_clause)*
                    $($where_clause)*
                {
                    unsafe fn get_field_raw_mut_inner(
                        $this:*mut  (),
                        fname: __F,

                    )->*mut __Ty
                    where
                        Self:Sized
                    {
                        let $this=$this as *mut  Self;
                        let $this:*mut $delegating_to_type=
                            $as_field_mutref_closure;
                        <$delegating_to_type as
                            $crate::GetFieldMut<__F>
                        >::get_field_raw_mut( $this as *mut  (),fname )
                    }
                }

                unsafe impl<$($impl_params)* __V,__F,__Ty>
                    $crate::pmr::SpecGetVariantFieldMut< $crate::TStr<__V>,__F>
                    for $self
                where
                    $delegating_to_type:
                        $crate::GetVariantFieldMut<$crate::TStr<__V>,__F,Ty=__Ty>,
                    $($mut_where_clause)*
                    $($where_clause)*
                {
                    $crate::default_if!{
                        #[inline(always)]
                        cfg(all($($specialize_cfg)*))
                        unsafe fn get_vfield_raw_mut_inner(
                            $this:*mut  (),
                            vname: $crate::TStr<__V>,
                            fname: __F,
                        )->Option<$crate::pmr::NonNull<
                            __Ty
                        >>
                        where
                            Self:Sized
                        {
                            let $this=$this as *mut  Self;
                            let $this:*mut $delegating_to_type=
                                $as_field_mutref_closure;
                            let func=<
                                $delegating_to_type as
                                $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                            >::get_vfield_raw_mut_fn(&*$this);
                            func( $this as *mut  (),vname,fname )
                        }
                    }
                }

                #[cfg(all($($specialize_cfg)*))]
                unsafe impl<$($impl_params)* __V,__F,__Ty>
                    $crate::pmr::SpecGetVariantFieldMut< $crate::TStr<__V>,__F>
                    for $self
                where
                    $delegating_to_type:
                        Sized +
                        $crate::GetVariantFieldMut<$crate::TStr<__V>,__F,Ty=__Ty>,
                    $($mut_where_clause)*
                    $($where_clause)*
                {
                    unsafe fn get_vfield_raw_mut_inner(
                        $this:*mut  (),
                        vname: $crate::TStr<__V>,
                        fname: __F,
                    )->Option<$crate::pmr::NonNull<__Ty>>
                    where
                        Self:Sized
                    {
                        let $this=$this as *mut  Self;
                        let $this:*mut $delegating_to_type=
                            $as_field_mutref_closure;
                        <$delegating_to_type as
                            $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                        >::get_vfield_raw_mut_( $this as *mut  (),vname,fname )
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

            unsafe GetFieldMut $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )

        struct_fn( $($raw_ptr_fn:tt)* )
        enum_fn( $($raw_enum_ptr_fn:tt)* )
        impl( $($raw_ptr_impl:tt)* )
    )=>{
        unsafe impl<$($impl_params)* __F,__Ty>
            $crate::GetFieldMut<__F>
            for $self
        where
            $self: Sized,
            $delegating_to_type:
                $crate::GetFieldMut<__F,Ty=__Ty>,
            $($where_clause)*
            $($mut_where_clause)*
        {
            #[inline(always)]
            fn get_field_mut_(
                &mut self,
                fname: __F,
            )->&mut __Ty {
                let $this=self;
                let field:&mut $delegating_to_type=$unsafe_get_field_mut_closure;
                <$delegating_to_type as
                    $crate::GetFieldMut<_>
                >::get_field_mut_(field,fname)
            }

            $($raw_ptr_fn)*

            #[inline(always)]
            fn get_field_raw_mut_fn(
                &self
            )->$crate::field_traits::GetFieldRawMutFn<
                __F,
                __Ty,
            >{
                <Self as $crate::GetFieldMut<__F>>::get_field_raw_mut
            }
        }

        unsafe impl<$($impl_params)* __V,__F,__Ty>
            $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
            for $self
        where
            $self: Sized,
            $delegating_to_type: $crate::GetVariantFieldMut<$crate::TStr<__V>,__F,Ty=__Ty>,
            $($where_clause)*
            $($mut_where_clause)*
        {
            #[inline(always)]
            fn get_vfield_mut_(
                &mut self,
                vname: $crate::TStr<__V>,
                fname: __F,
            ) -> Option<&mut __Ty>{
                let $this=self;
                let field:&mut $delegating_to_type=$unsafe_get_field_mut_closure;
                <$delegating_to_type as
                    $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                >::get_vfield_mut_(field,vname,fname)
            }

            $($raw_enum_ptr_fn)*

            #[inline(always)]
            fn get_vfield_raw_mut_unchecked_fn(
                &self
            )->$crate::pmr::GetFieldRawMutFn<__F,__Ty>{
                <Self as
                    $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                >::get_vfield_raw_mut_unchecked
            }

            #[inline(always)]
            fn get_vfield_raw_mut_fn(
                &self
            )->$crate::pmr::GetVFieldRawMutFn<
                    $crate::TStr<__V>,
                    __F,
                    __Ty
                >
            {
                <Self as
                    $crate::GetVariantFieldMut<$crate::TStr<__V>,__F>
                >::get_vfield_raw_mut_
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

        IntoField $into_field_closure:block
    )=>{

        // This is defined separately from `unsafe_delegate_vfield!`
        // because additional bounds might be added to IntoField.
        unsafe impl<$($impl_params)* __V,__F,__Ty>
            $crate::pmr::IntoVariantField<$crate::TStr<__V>,__F>
        for $self
        where
            $delegating_to_type:
                Sized+
                $crate::pmr::IntoVariantField<$crate::TStr<__V>,__F,Ty=__Ty>,
            $($into_where_clause)*
            $($where_clause)*
        {
            #[inline(always)]
            fn into_vfield_(
                self,
                vname:$crate::TStr<__V>,
                fname:__F,
            )->Option<$crate::GetVariantFieldType<$delegating_to_type,$crate::TStr<__V>,__F>>{
                let $this=self;
                let field:$delegating_to_type=$into_field_closure;
                $crate::IntoVariantField::<$crate::TStr<__V>,__F>::into_vfield_(field,vname,fname)
            }

            $crate::z_impl_box_into_variant_field_method!{
                $crate::TStr<__V>,
                __F,
                $crate::GetVariantFieldType<$delegating_to_type,$crate::TStr<__V>,__F>,
            }
        }

        impl<$($impl_params)* __F,__Ty>
            $crate::IntoField< __F>
            for $self
        where
            $delegating_to_type:
                Sized+
                $crate::IntoField<__F,Ty=__Ty>,
            $($into_where_clause)*
            $($where_clause)*
        {
            #[inline(always)]
            fn into_field_(
                self,
                fname: __F,
            )->__Ty{
                let $this=self;
                let field:$delegating_to_type=$into_field_closure;
                $crate::IntoField::<__F>::into_field_(field,fname)
            }

            $crate::z_impl_box_into_field_method!{
                __F,
                __Ty,
            }
        }
    };
}

//////////////////////////////////////////////////////////////////////////////
