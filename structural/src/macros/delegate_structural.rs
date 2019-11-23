/**
This macro allows delegating the implementation of the Structural and field accessor traits.

This macro delegates the implementation of those traits for all fields,
it doesn't provide a way to do so for only a list of fields.

# Safety

The unsafety of implementing GetFieldMut with this macro comes from the methods
used to do multiple mutable borrows.

You must ensure that the variable that you delegate GetField to is the same as the one
you delegate GetFieldMut to,
as well as ensuring that there are no other impls of the GetFieldMut trait 
borrowing from the same variable mutably.


# Example with all syntax

```rust
use structural::z_delegate_structural_with;
use structural::mut_ref::MutRef;

# trait Trait{}
# impl<T> Trait for T{}

struct Foo<T>{
    value:T
}

z_delegate_structural_with!{
    impl[T] Foo<T>
    // This where clause is required syntax
    where[
        T:Trait,
    ]
    
    // This is the identifier used for `self` in the blocks bellow.
    self_ident=this;
    
    // This is the type of the variable we delegate to,
    // this is required because Rust doesn't have a `typeof`/`decltype` construct.
    delegating_to_type=T;
    field_name_param=( fname_var : fname_ty );

    // This block of code is used to get the reference to the delegating variable 
    // in GetField and other traits.
    GetField {
        &this.value 
    }
    
    // This block of code is used to get a mutable reference to the delegating variable
    // in GetFieldMut
    //
    // This is `unsafe` because this block must always evaluate to a mutable reference
    // for the same variable,
    // and it must not be the same variable as other implementations of the GetFieldMut trait
    unsafe GetFieldMut 
    where [ 
        // This is an optional where clause
        // The last where predicate must have a trailing comma.
        T:Trait, 
    ]{
        &mut this.value
    }
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

use structural::{GetFieldExt,GetFieldMut,z_delegate_structural_with,make_struct,fp};
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


z_delegate_structural_with!{
    impl[T] Bar<T>
    where[
        // This requires T to implement Clone
        // for `Bar<T>` to implement the accessor traits
        T:Clone 
    ]
    self_ident=this;
    delegating_to_type=T;
    field_name_param=( fname_var : fname_ty );

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
macro_rules! z_delegate_structural_with {
    (
        impl $impl_params:tt $self:ty
        where $where_clause:tt
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

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
        
        $crate::z_delegate_structural_with!{
            inner-structural;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            delegating_to_type=$delegating_to_type;
            field_name_param=( $fname_var : $fname_ty );
            GetField $get_field_closure
        }

        $crate::z_delegate_structural_with!{
            inner;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            delegating_to_type=$delegating_to_type;
            field_name_param=( $fname_var : $fname_ty );
            GetField $get_field_closure
        }

        $(
            $crate::z_delegate_structural_with!{
                inner;
                impl $impl_params $self
                where $where_clause
                where[ $( $($mut_where_clause)* )? ]
                self_ident=$this;
                delegating_to_type=$delegating_to_type;
                field_name_param=( $fname_var : $fname_ty );
                GetField $get_field_closure
                unsafe GetFieldMut $unsafe_get_field_mut_closure
                as_delegating_raw $as_field_mutref_closure
            }
        )?

        $(
            $crate::z_delegate_structural_with!{
                inner;
                impl $impl_params $self
                where $where_clause
                where [ $( $($into_where_clause)* )? ]
                self_ident=$this;
                delegating_to_type=$delegating_to_type;
                field_name_param=( $fname_var : $fname_ty );
                IntoField $into_field_closure
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

        GetField $get_field_closure:block
    )=>{
        impl<$($impl_params)*> $crate::Structural for $self 
        where
            $delegating_to_type:$crate::Structural,
            $($where_clause)*
        {
            const FIELDS:&'static[$crate::structural_trait::FieldInfo]=
                <$delegating_to_type as $crate::Structural>::FIELDS;
        }

        impl<$($impl_params)*> $crate::StructuralDyn for $self
        where
            $delegating_to_type:$crate::StructuralDyn,
            $($where_clause)*
        {
            fn fields_info(&self)->&'static[$crate::structural_trait::FieldInfo]{
                let $this=self;
                let field:&$delegating_to_type=$get_field_closure;
                $crate::StructuralDyn::fields_info(field)
            }
        }
    };
    (inner;
        impl [$($($impl_params:tt)+)?] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        GetField $get_field_closure:block
    )=>{
        impl<$($($impl_params)* , )?__FieldName> 
            $crate::GetField<__FieldName>
            for $self
        where
            $delegating_to_type:$crate::GetField<__FieldName>,
            $($where_clause)*
        {
            type Ty=$crate::GetFieldType<$delegating_to_type,__FieldName>;

            fn get_field_(&self)->&Self::Ty{
                let $this=self;
                let field:&$delegating_to_type=$get_field_closure;
                $crate::GetField::<__FieldName>::get_field_(field)
            }
        }
    };
    (inner;
        impl [$($($impl_params:tt)+)?] $self:ty
        where [$($where_clause:tt)*]
        where [$($mut_where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        GetField $get_field_closure:block

        unsafe GetFieldMut $unsafe_get_field_mut_closure:block
        as_delegating_raw $as_field_mutref_closure:block
    )=>{
        unsafe impl<$( $($impl_params)* , )?__FieldName> 
            $crate::GetFieldMut<__FieldName>
            for $self
        where
            $delegating_to_type:$crate::GetFieldMut<__FieldName>,
            $($mut_where_clause)*
            $($where_clause)*
        {
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                let $this=self;
                let field:&mut $delegating_to_type=$unsafe_get_field_mut_closure;
                <$delegating_to_type as $crate::GetFieldMut<__FieldName>>::get_field_mut_(field)
            }
            unsafe fn get_field_raw_mut(
                $this:*mut (),
                $fname_var:$crate::pmr::PhantomData<__FieldName>
            )->*mut Self::Ty
            where 
                Self:Sized
            {
                let $this=$this as *mut Self;
                let $this:*mut $delegating_to_type=
                    $as_field_mutref_closure;
                
                <$delegating_to_type>::get_field_raw_mut( $this as *mut (),$fname_var )
            }

            fn get_field_raw_mut_func(
                &self
            )->$crate::field_traits::GetFieldMutRefFn<__FieldName,Self::Ty>{
                <Self as $crate::GetFieldMut<__FieldName>>::get_field_raw_mut
            }
        }
    };
        (inner;
        impl [$($($impl_params:tt)+)?] $self:ty
        where [$($where_clause:tt)*]
        where [$($into_where_clause:tt)*]
        self_ident=$this:ident;
        delegating_to_type=$delegating_to_type:ty;
        field_name_param=( $fname_var:ident : $fname_ty:ident );

        IntoField $into_field_closure:block
    )=>{
        impl<$( $($impl_params)* , )?__FieldName> 
            $crate::IntoField<__FieldName>
            for $self
        where
            $delegating_to_type:$crate::IntoField<__FieldName>,
            $($into_where_clause)*
            $($where_clause)*
        {
            fn into_field_(self)->Self::Ty{
                let $this=self;
                let field:$delegating_to_type=$into_field_closure;
                $crate::IntoField::<__FieldName>::into_field_(field)
            }

            $crate::z_impl_box_into_field_method!{__FieldName}
        }        
    };
}



//////////////////////////////////////////////////////////////////////////////
