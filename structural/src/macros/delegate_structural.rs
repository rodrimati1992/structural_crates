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
use structural::delegate_structural_with;

struct Foo<T>{
    value:T
}

delegate_structural_with!{
    impl[T] Foo<T>
    // This where clause is required syntax
    where[ /*You put the where predicates in here.*/ ]
    
    // This is the identifier used for `self` in the blocks bellow.
    self_ident=this;
    
    // This is the type of the variable we delegate to,
    // this is required because Rust doesn't have a `typeof`/`decltype` construct.
    field_ty=T;

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
    unsafe GetFieldMut {
        &mut this.value
    }
    
    // This block of code is used to get the delegating variable by value in IntoField.
    IntoField {
        this.value 
    }
}
```

# Example

This example is of a type wrapping a `ManuallyDrop<T>`,delegating to the `T` inside it.

```rust
use std::mem::ManuallyDrop;

use structural::{GetFieldExt,delegate_structural_with,make_struct,ti};

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


delegate_structural_with!{
    impl[T] Bar<T>
    where[
        // This requires T to implement Clone
        // for `Bar<T>` to implement the accessor traits
        T:Clone 
    ]
    self_ident=this;
    field_ty=T;

    GetField {
#       // This ensures that the `T:Clone` bound is put on the impl block.
#       T::clone;
        &*this.value 
    }
    unsafe GetFieldMut {
#       // This ensures that the `T:Clone` bound is put on the impl block.
#       T::clone;
        &mut *this.value 
    }
    IntoField {
#       // This ensures that the `T:Clone` bound is put on the impl block.
#       T::clone;
        ManuallyDrop::into_inner(this.value)
    }
}

{
    let mut bar=Bar::new((2,3,5,8,13));
    assert_eq!(
        bar.fields(ti!(4,3,2,1,0)),
        ( &13, &8, &5, &3, &2 )
    );

    assert_eq!(
        bar.fields_mut(ti!(1,2,3,4)),
        ( &mut 3, &mut 5, &mut 8, &mut 13 )
    );

    assert_eq!(bar.into_field(ti!(1)),3);
}

{
    let mut bar=Bar::new(make_struct!{
        #![derive(Clone)] //This derives Clone for the anonymous struct

        a:"hello",
        b:"world",
        c:"tour",
    });
    assert_eq!(
        bar.fields(ti!(a,b,c)),
        ( &"hello", &"world", &"tour" )
    );

    assert_eq!(
        bar.fields_mut(ti!(c,b,a)),
        ( &mut"tour", &mut"world", &mut"hello" )
    );

    assert_eq!( bar.into_field(ti!(c)), "tour" );
}


```


*/
#[macro_export]
macro_rules! delegate_structural_with {
    (
        impl $impl_params:tt $self:ty
        where $where_clause:tt
        self_ident=$this:ident;
        field_ty=$field_ty:ty;

        GetField $get_field_closure:block
        $(
            unsafe GetFieldMut 
            $( where[ $($mut_where_clause:tt)* ] )?
            $unsafe_get_field_mut_closure:block
        )?
        $(
            IntoField 
            $( where[ $($into_where_clause:tt)* ] )?
            $into_field_closure:block
        )?
    ) => (
        
        $crate::delegate_structural_with!{
            inner-structural;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            field_ty=$field_ty;
            GetField $get_field_closure
        }

        $crate::delegate_structural_with!{
            inner;
            impl $impl_params $self
            where $where_clause
            self_ident=$this;
            field_ty=$field_ty;
            GetField $get_field_closure
        }

        $(
            $crate::delegate_structural_with!{
                inner;
                impl $impl_params $self
                where $where_clause
                where[ $( $($mut_where_clause)* )? ]
                self_ident=$this;
                field_ty=$field_ty;
                GetField $get_field_closure
                unsafe GetFieldMut $unsafe_get_field_mut_closure
            }
        )?

        $(
            $crate::delegate_structural_with!{
                inner;
                impl $impl_params $self
                where $where_clause
                where [ $( $($into_where_clause)* )? ]
                self_ident=$this;
                field_ty=$field_ty;
                IntoField $into_field_closure
            }
        )?
    );
    (
        inner-structural;
        impl[$($impl_params:tt)*] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        field_ty=$field_ty:ty;

        GetField $get_field_closure:block
    )=>{
        impl<$($impl_params)*> $crate::Structural for $self 
        where
            $field_ty:$crate::Structural,
            $($where_clause)*
        {
            const FIELDS:&'static[$crate::structural_trait::FieldInfo]=
                <$field_ty as $crate::Structural>::FIELDS;

            type Fields=
                <$field_ty as $crate::Structural>::Fields;
        }

        impl<$($impl_params)*> $crate::StructuralDyn for $self
        where
            $field_ty:$crate::StructuralDyn,
            $($where_clause)*
        {
            fn fields_info(&self)->&'static[$crate::structural_trait::FieldInfo]{
                let $this=self;
                let field:&$field_ty=$get_field_closure;
                $crate::StructuralDyn::fields_info(field)
            }
        }
    };
    (inner;
        impl [$($($impl_params:tt)+)?] $self:ty
        where [$($where_clause:tt)*]
        self_ident=$this:ident;
        field_ty=$field_ty:ty;

        GetField $get_field_closure:block
    )=>{
        impl<$($($impl_params)* , )?__FieldName> 
            $crate::GetField<__FieldName>
            for $self
        where
            $field_ty:$crate::GetField<__FieldName>,
            $($where_clause)*
        {
            type Ty=$crate::GetFieldType<$field_ty,__FieldName>;

            fn get_field_(&self)->&Self::Ty{
                let $this=self;
                let field:&$field_ty=$get_field_closure;
                $crate::GetField::<__FieldName>::get_field_(field)
            }
        }
    };
    (inner;
        impl [$($($impl_params:tt)+)?] $self:ty
        where [$($where_clause:tt)*]
        where [$($mut_where_clause:tt)*]
        self_ident=$this:ident;
        field_ty=$field_ty:ty;

        GetField $get_field_closure:block
        unsafe GetFieldMut $unsafe_get_field_mut_closure:block
    )=>{
        unsafe impl<$( $($impl_params)* , )?__FieldName> 
            $crate::GetFieldMut<__FieldName>
            for $self
        where
            $field_ty:$crate::GetFieldMut<__FieldName>,
            $($mut_where_clause)*
            $($where_clause)*
        {
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                let $this=self;
                let field:&mut $field_ty=$unsafe_get_field_mut_closure;
                <$field_ty as $crate::GetFieldMut<__FieldName>>::get_field_mut_(field)
            }
            unsafe fn get_field_mutref(
                ptr:$crate::mut_ref::MutRef<'_,()>,
                getter:$crate::field_traits::GetFieldMutRefFn<__FieldName,Self::Ty>
            )->&mut Self::Ty
            where 
                Self:Sized
            {
                <$field_ty as 
                    $crate::GetFieldMut<__FieldName>
                >::get_field_mutref(ptr,getter)
            }

            fn as_mutref(&mut self)->$crate::mut_ref::MutRef<'_,()>{
                let $this=self;
                let field:&mut $field_ty=$unsafe_get_field_mut_closure;
                $crate::GetFieldMut::<__FieldName>::as_mutref(field)
            }

            fn get_field_mutref_func(
                &self
            )->$crate::field_traits::GetFieldMutRefFn<__FieldName,Self::Ty>{
                let $this=self;
                let field:&$field_ty=$get_field_closure;
                $crate::GetFieldMut::<__FieldName>::get_field_mutref_func(field)
            }
        }
    };
        (inner;
        impl [$($($impl_params:tt)+)?] $self:ty
        where [$($where_clause:tt)*]
        where [$($into_where_clause:tt)*]
        self_ident=$this:ident;
        field_ty=$field_ty:ty;

        IntoField $into_field_closure:block
    )=>{
        impl<$( $($impl_params)* , )?__FieldName> 
            $crate::IntoField<__FieldName>
            for $self
        where
            $field_ty:$crate::IntoField<__FieldName>,
            $($into_where_clause)*
            $($where_clause)*
        {
            fn into_field_(self)->Self::Ty{
                let $this=self;
                let field:$field_ty=$into_field_closure;
                $crate::IntoField::<__FieldName>::into_field_(field)
            }

            $crate::impl_box_into_field_method!{__FieldName}
        }        
    };
}


