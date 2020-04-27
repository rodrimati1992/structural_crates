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

You must ensure that the variable that you delegate `Get*Field` to is the same as the one
you delegate `Get*FieldMut`/`Into*Field` to,
as well as ensuring that there are no other impls of the `Get*FieldMut`/`Into*Field` traits
accessing the same delegated-to variable.

# Drop behavior

When converting a type into multiple fields by value
(using [`StructuralExt::into_fields`] or [`StrucWrapper::vals`]),
the regular destructor (`Drop::drop`) won't run,
instead the steps [in the section below](#drop-order-section) will happen.

If your type has code that must run when it's dropped,
you can pass `DropFields= { ..... pre_move = foo_function; }` to this macro
to run that code before the type is converted into multiple fields by value
(using [`StructuralExt::into_fields`] or [`StrucWrapper::vals`]),

<span id="drop-order-section"></span>
### Drop order

The order of operations when invoking [`StructuralExt::into_fields`] or [`StrucWrapper::vals`]
is this by default:

- Call [`DropFields::pre_move`].

- Move out the fields.

- Call [`PrePostDropFields::pre_drop`].

- Drop the fields passed to `dropped_fields[]`, in the order that they were passed.

- Drop the delegated-to variable with [`DropFields::drop_fields`].

- Call [`PrePostDropFields::post_drop`].

- Return the moved out fields

[`DropFields`]: ./field/ownership/trait.DropFields.html

[`DropFields::drop_fields`]: ./field/ownership/trait.DropFields.html#tymethod.drop_fields

[`DropFields::pre_move`]: ./field/ownership/trait.DropFields.html#tymethod.pre_move

[`PrePostDropFields`]: ./field/ownership/trait.PrePostDropFields.html

[`PrePostDropFields::pre_drop`]: ./field/ownership/trait.PrePostDropFields.html#method.pre_drop

[`PrePostDropFields::post_drop`]: ./field/ownership/trait.PrePostDropFields.html#method.post_drop

# Example with all syntax

```rust
use structural::unsafe_delegate_structural_with;

# trait Trait{}
# impl<T> Trait for T{}

struct Foo<T>{
    a: (),
    b: (),
    c: (),
    d: Bar,
    value:T
}

struct Bar{
    e: (),
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
    // This is the default value for the parameter:
    // `specialization_params(Sized);`

    // This means that the type is `?Sized` and not specialization is used,
    // this may be slower in debug builds because this always uses a
    // function pointer call in raw-pointer-taking methods.
    //
    // `specialization_params(?Sized);`

    // This means that the type is `?Sized` by default.
    // The `cfg(anything)` argument enables specialization conditionally,
    //
    // When specialization is disabled theres only a default impl for `Self:?Sized`
    // which may be slower in debug builds,
    // because this uses a function pointer call in raw-pointer methods.
    //
    // When specialization is enabled,the impl is specializes on `Self:Sized`
    // to remove the overhead of raw-pointer methods.
    //
    // `specialization_params(cfg(anything));`


    // This is the type of the variable we delegate to,
    // this is required because Rust doesn't have a `typeof`/`decltype` construct.
    delegating_to_type=T;

    // This block of code is used to get the reference to the delegated-to variable
    // in GetField.
    GetField {
        &this.value
    }

    // This block of code is used to get a mutable reference to the delegated-to variable
    // in GetFieldMut
    //
    // This block must always evaluate to a mutable reference for the same variable,
    // and it must not be the same variable as other implementations of the GetFieldMut trait.
    //
    GetFieldMut
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

    // This block of code is used to get the delegated-to variable in IntoField::into_field.
    IntoField
    where [
        // This is an optional where clause
        // The last where predicate must have a trailing comma.
        T:Trait,
    ]{
        this.value
    }
    // This block of code is used to get the delegated-to variable in
    // IntoField::move_out_field_.
    move_out_field{
        &mut this.value
    }

    // `DropFields = .... ` determines what happens in the DropFields implementation
    // for the type, which isn't as trivial as delegating to the delegated_to type.
    //
    DropFields = {
        // Which fields are dropped in `DropFields::drop_fields`,
        // in the order that they're dropped.
        //
        // If you're dropping a nested field,it must be wrapped in parentheses(ie:`(a.b.c)`).
        dropped_fields[ a,b,c,(d.e) ]

        // An optional argument for code to run right before fields are moved.
        //
        // `some_function` is called in the implementation of
        // `DropFields::pre_move` generated for this type.
        //
        // The passed in function must have this signature: `fn(&mut Self)`
        // in this example it would be `fn<T: Trait>(&mut Foo<T>)`
        //
        // `pre_move = some_function;`

        // An optional argument which determines whether there's pre/post field-drop logic
        // in `DropFields::drop_fields`.
        //
        // With `pre_post_drop_fields=true;` this inserts
        // a `PrePostDropFields::pre_drop(self)` call before the fields are dropped,
        // and a `PrePostDropFields::post_drop(self)` call after the fields are dropped,
        //
        // To use this the type must implement the `PrePostDropFields` trait
        // to do something before and after the listed fields (and the delegated_to variable)
        // are dropped.
        //
        // This is the default value:
        // `pre_post_drop_fields=false;`

        // Whether to drop the fields in the delegated_to variable,with `DropFields::drop_fields`.
        //
        // With `drop_delegated_drop_variable=true;`,
        // this calls `DropFields::drop_fields` on the delegated to variable.
        //
        // With `drop_delegated_drop_variable=false;`,
        // it does not drop the delegated to variable in an way.
        //
        drop_delegated_to_variable=true;
    }

    // Requires that the DropFields trait is implemented manually.
    // DropFields = manual;

}
```

# Example

This example is of a type wrapping a `ManuallyDrop<T>`,delegating to the `T` inside it.

```rust
use std::{
    fmt::Debug,
    mem::ManuallyDrop,
};

use structural::{StructuralExt,GetFieldMut,unsafe_delegate_structural_with,make_struct,fp};

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

    GetFieldMut
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
    move_out_field{
        &mut *this.value
    }

    DropFields = {
        dropped_fields[]
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
            GetFieldMut
            $( where[ $($mut_where_clause:tt)* ] )?
            $unsafe_get_field_mut_closure:block
            as_delegating_raw $as_field_mutref_closure:block
        )?
        $(
            IntoField
            $( where[ $($into_where_clause:tt)* ] )?
            $into_field_closure:block
            move_out_field $move_out_field_closure:block

            DropFields = $drop_fields_param:tt $(;)?
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

                GetFieldMut $unsafe_get_field_mut_closure
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
                move_out_field $move_out_field_closure
            }

            $crate::unsafe_delegate_structural_with_inner!{
                @drop_fields_impls
                impl $impl_params $self
                where $where_clause
                where [ $( $($into_where_clause)* )? ]
                self_ident=$this;
                delegating_to_type=$delegating_to_type;
                IntoField $into_field_closure
                move_out_field $move_out_field_closure

                DropFields = $drop_fields_param
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

        GetFieldMut $unsafe_get_field_mut_closure:block
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

                GetFieldMut $unsafe_get_field_mut_closure
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

            GetFieldMut $unsafe_get_field_mut_closure:block
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

            GetFieldMut $unsafe_get_field_mut_closure:block
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

            GetFieldMut $unsafe_get_field_mut_closure:block
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

            GetFieldMut $unsafe_get_field_mut_closure:block
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
            )->$crate::field::GetFieldRawMutFn<
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
        move_out_field $move_out_field_closure:block
    )=>{

        unsafe impl<$($impl_params)* __V,__F,__Ty>
            $crate::pmr::IntoVariantField<$crate::TStr<__V>,__F>
        for $self
        where
            $delegating_to_type:
                $crate::pmr::IntoVariantField<$crate::TStr<__V>,__F,Ty=__Ty>,
            $($into_where_clause)*
            $($where_clause)*
        {
            #[inline(always)]
            fn into_vfield_(self,vname:$crate::TStr<__V>,fname: __F)->Option<__Ty>{
                let $this=self;
                let field:$delegating_to_type=$into_field_closure;
                $crate::IntoVariantField::<$crate::TStr<__V>, __F>::into_vfield_(
                    field,
                    vname,
                    fname,
                )
            }

            #[inline(always)]
            unsafe fn move_out_vfield_(
                &mut self,
                vname:$crate::TStr<__V>,
                fname:__F,
                moved: &mut $crate::pmr::MovedOutFields,
            )->Option<__Ty>{
                let $this=self;
                let field:&mut $delegating_to_type=$move_out_field_closure;
                $crate::IntoVariantField::<$crate::TStr<__V>,__F>::move_out_vfield_(
                    field,
                    vname,
                    fname,
                    moved,
                )
            }
        }

        unsafe impl<$($impl_params)* __F,__Ty>
            $crate::IntoField< __F>
            for $self
        where
            $delegating_to_type:
                $crate::IntoField<__F,Ty=__Ty>,
            $($into_where_clause)*
            $($where_clause)*
        {
            #[inline(always)]
            fn into_field_(self,fname: __F)->__Ty{
                let $this=self;
                let field:$delegating_to_type=$into_field_closure;
                $crate::IntoField::<__F>::into_field_(field, fname)
            }

            #[inline(always)]
            unsafe fn move_out_field_(
                &mut self,
                fname:__F,
                moved: &mut $crate::pmr::MovedOutFields,
            )->__Ty{
                let $this=self;
                let field:&mut $delegating_to_type=$move_out_field_closure;
                $crate::IntoField::<__F>::move_out_field_(
                    field,
                    fname,
                    moved,
                )
            }
        }
    };
    (@drop_fields_impls
        impl [$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        where [$($into_where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;

        IntoField $into_field_closure:block

        DropFields = manual
    )=>{

    };
    (@drop_fields_impls
        impl [$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        where [$($into_where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;

        IntoField $into_field_closure:block
        move_out_field $move_out_field_closure:block

        DropFields = {
            dropped_fields[$($field:tt)*]

            $(pre_move = $pre_move:expr;)?

            $(pre_post_drop_fields=$pp_drop_fields:ident;)?
            $(drop_delegated_to_variable=$drop_delegated:ident;)?
        }
    )=>{
        unsafe impl<$($impl_params)*> $crate::pmr::DropFields for $self
        where
            $delegating_to_type: $crate::pmr::DropFields,
            $($into_where_clause)*
            $($where_clause)*
        {
            #[inline(always)]
            fn pre_move(&mut self){
                #[allow(unused_variables,unused_mut)]
                let mut guard=$crate::pmr::RunOnDrop::new(
                    self,
                    #[inline(always)]
                    |$this|{
                        let field:&mut $delegating_to_type=$move_out_field_closure;
                        $crate::pmr::DropFields::pre_move(field);
                    }
                );

                $(
                    let func=$pre_move;
                    func(guard.reborrow_mut());
                )?
            }

            #[allow(unused_variables)]
            unsafe fn drop_fields(&mut self, moved: $crate::pmr::MovedOutFields) {
                let $this=self;

                $crate::delegate_to_into_helper!{
                    post_drop=$($pp_drop_fields)?,
                    self_ident=$this,
                }

                // Remember that:
                //
                // - the fields are dropped in the destructor for RunDrop
                //
                // - the delegated-to variable is dropped in the destructor for
                // RunDropFields.
                //
                // Destructors run in the opposite order in which variables are declared,
                // and after all statements.
                let mut $this=$crate::pmr::RunOnDrop::new(
                    $this,
                    #[inline(always)]
                    |$this|{
                        $crate::delegate_to_into_helper!{
                            drop_delegated_to=$($drop_delegated)?,
                            self_ident=$this,
                            delegating_to_type=$delegating_to_type,
                            delegating_to=$move_out_field_closure,
                            moved_fields_variable=moved,
                        }

                        $crate::reverse_code!{
                            $((
                                $crate::delegate_to_into_helper!{
                                    drop_field($field),
                                    self_ident=$this,
                                }
                            ))*
                        }
                    }
                );

                let $this=$this.reborrow_mut();
                $crate::delegate_to_into_helper!{
                    pre_drop=$($pp_drop_fields)?,
                    self_ident=$this,
                }
            }
        }
    };
    ( $($stuff:tt)* )=>{
        compile_error!{concat!(
            "Unrecognized `unsafe_delegate_structural_with_inner` arguments:",
            $(stringify!($stuff),)*
        )}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! delegate_to_into_helper {
    (
        drop_field(($($nested:tt)*)),
        self_ident=$this:ident,
    )=>{
        let _a=$crate::pmr::RunDrop::new(&mut $this.$($nested)*);
    };
    (
        // Ignoring the optional commas separating the fields
        drop_field(,),
        self_ident=$this:ident,
    )=>{};
    (
        drop_field($field:tt),
        self_ident=$this:ident,
    )=>{
        let _a=$crate::pmr::RunDrop::new(&mut $this.$field);
    };
    //////////////////////////////////////////////////////////////////////
    (
        $(pre_drop)? $(post_drop)?=$(false)?,
        self_ident=$this:ident,
    ) => ();
    (
        pre_drop=true,
        self_ident=$this:ident,
    ) => (
        $crate::pmr::PrePostDropFields::pre_drop($this);
    );
    (
        post_drop=true,
        self_ident=$this:ident,
    ) => (
        let mut $this = $crate::pmr::RunPostDrop::new($this);
        let $this=$this.get_mut();
    );
    //////////////////////////////////////////////////////////////////////
    (
        drop_delegated_to=false,
        self_ident=$this:ident,
        delegating_to_type=$delegating_to_type:ty,
        delegating_to=$into_field_closure:block,
        moved_fields_variable=$moved_fields_var:ident,
    ) => ();
    (
        drop_delegated_to=$(true)?,
        self_ident=$this:ident,
        delegating_to_type=$delegating_to_type:ty,
        delegating_to=$into_field_closure:block,
        moved_fields_variable=$moved_fields_var:ident,
    ) => (
        let delegated_to:&mut $delegating_to_type = $into_field_closure;
        let _a=$crate::pmr::RunDropFields::new(delegated_to,$moved_fields_var);
    );
}

//////////////////////////////////////////////////////////////////////////////
