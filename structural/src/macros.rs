macro_rules! impl_getter{
    ( impl[$($typarams:tt)*]
        GetField <$field_name:tt : $field_ty:ty,$name_param:tt,{$index:expr}> 
        for $self_:ty 
    )=>{
        impl<$($typarams)*> GetField<$name_param> for $self_ {
            type Ty=$field_ty;

            fn get_field_(&self)->&Self::Ty{
                &self.$field_name
            }
        }
    };
    ( unsafe impl[$($typarams:tt)*]
        GetFieldMut <$field_name:tt : $field_ty:ty,$name_param:tt,{$index:expr}> 
        for $self_:ty 
    )=>{
        impl_getter!{
            impl[$($typarams)*] GetField<$field_name:$field_ty,$name_param,{$index}> for $self_
        }
    
        unsafe impl<$($typarams)*> GetFieldMut<$name_param> for $self_ {
            fn get_field_mut_(&mut self)->&mut Self::Ty{
                &mut self.$field_name
            }

            unsafe fn raw_get_mut_field(this:MutRef<'_,Self>)->&mut Self::Ty{
                &mut (*this.ptr).$field_name
            }
        }
    };
    ( unsafe impl[$($typarams:tt)*]
        IntoField <$field_name:tt : $field_ty:ty,$name_param:tt,{$index:expr}> 
        for $self_:ty 
    )=>{
        impl_getter!{
            unsafe impl[$($typarams)*] 
                GetFieldMut<$field_name:$field_ty,$name_param,{$index}> 
            for $self_
        }
    
        impl<$($typarams)*> IntoField<$name_param> for $self_ {
            fn into_field_(self)->Self::Ty{
                self.$field_name
            }
        }
    };
} 

/// Gets a type-level string value
///
#[macro_export]
macro_rules! tstr {
    ( $($strings:literal),* $(,)* ) => {{
        mod dummy{
            structural_derive::tstr_impl!{$($strings),*}
        }
        dummy::VALUE
    }};
}

/// Gets a type-level string for use as a generic parameter.
///
/// # Future Compatibility
///
/// This macro will continue supporting space separated characters 
/// even after string literals are usable as trait parameters.
///
#[macro_export]
macro_rules! TStr {
    ($($char:tt)*) => {
        $crate::type_level::TString<($($crate::TChar!($char),)*)>
    };
}


/*

Code to generate the non-default branches

fn main() {
    for b in 0..=255u8 {
        let c=b as char;
        if c.is_alphanumeric() && b<128 || c=='_' {
            println!("({})=>( $crate::chars::_{} );",c,c)
        }
    }
}

*/

#[doc(hidden)]
#[macro_export]
macro_rules! TChar{
    (0)=>( $crate::chars::_0 );
    (1)=>( $crate::chars::_1 );
    (2)=>( $crate::chars::_2 );
    (3)=>( $crate::chars::_3 );
    (4)=>( $crate::chars::_4 );
    (5)=>( $crate::chars::_5 );
    (6)=>( $crate::chars::_6 );
    (7)=>( $crate::chars::_7 );
    (8)=>( $crate::chars::_8 );
    (9)=>( $crate::chars::_9 );
    (A)=>( $crate::chars::_A );
    (B)=>( $crate::chars::_B );
    (C)=>( $crate::chars::_C );
    (D)=>( $crate::chars::_D );
    (E)=>( $crate::chars::_E );
    (F)=>( $crate::chars::_F );
    (G)=>( $crate::chars::_G );
    (H)=>( $crate::chars::_H );
    (I)=>( $crate::chars::_I );
    (J)=>( $crate::chars::_J );
    (K)=>( $crate::chars::_K );
    (L)=>( $crate::chars::_L );
    (M)=>( $crate::chars::_M );
    (N)=>( $crate::chars::_N );
    (O)=>( $crate::chars::_O );
    (P)=>( $crate::chars::_P );
    (Q)=>( $crate::chars::_Q );
    (R)=>( $crate::chars::_R );
    (S)=>( $crate::chars::_S );
    (T)=>( $crate::chars::_T );
    (U)=>( $crate::chars::_U );
    (V)=>( $crate::chars::_V );
    (W)=>( $crate::chars::_W );
    (X)=>( $crate::chars::_X );
    (Y)=>( $crate::chars::_Y );
    (Z)=>( $crate::chars::_Z );
    (_)=>( $crate::chars::__ );
    (a)=>( $crate::chars::_a );
    (b)=>( $crate::chars::_b );
    (c)=>( $crate::chars::_c );
    (d)=>( $crate::chars::_d );
    (e)=>( $crate::chars::_e );
    (f)=>( $crate::chars::_f );
    (g)=>( $crate::chars::_g );
    (h)=>( $crate::chars::_h );
    (i)=>( $crate::chars::_i );
    (j)=>( $crate::chars::_j );
    (k)=>( $crate::chars::_k );
    (l)=>( $crate::chars::_l );
    (m)=>( $crate::chars::_m );
    (n)=>( $crate::chars::_n );
    (o)=>( $crate::chars::_o );
    (p)=>( $crate::chars::_p );
    (q)=>( $crate::chars::_q );
    (r)=>( $crate::chars::_r );
    (s)=>( $crate::chars::_s );
    (t)=>( $crate::chars::_t );
    (u)=>( $crate::chars::_u );
    (v)=>( $crate::chars::_v );
    (w)=>( $crate::chars::_w );
    (x)=>( $crate::chars::_x );
    (y)=>( $crate::chars::_y );
    (z)=>( $crate::chars::_z );
    ($byte:ident)=>{
        $crate::chars::$byte
    }
} 


/**

The `structural_alias` defines a trait alias for multiple field accessors.

# The entire syntax

```
# use structural::structural_alias;
# pub trait SuperTrait{}

structural_alias!{
    pub trait Foo<'a,T>:SuperTrait
    where
        T:SuperTrait
    {
             a:u32,
        ref  b:T,
        mut  c:i64,
        move d:String,
    }
}

# fn main(){}
```

Outside of the `{...}` the trait syntax is the same as the 
regular one,with the same meaning.

Inside the `{...}` is a list of fields,
each of which get turned into supertraits on `Foo`:

`     a:u32`:
    Corresponds to the `GetField<TString<(_a,)>,Ty=u32>` shared reference 
    field accessor trait.

`ref  b:T`
    Corresponds to the `GetField<TString<(_b,)>,Ty=T>` shared reference 
    field accessor trait.

`mut  c:i64`:
    Corresponds to the `GetFieldMut<TString<(_c,)>,Ty=i64>` mutable reference 
    field accessor trait (which`itself implies `GetField`).

`move d:String`:
    Corresponds to the `IntoField<TString<(_d,)>,Ty=String>` by value
    field accessor trait (which`itself implies `GetField` and `GetFieldMut`).

# Examples

### Defining a Point trait alias

```rust
use structural::{
    structural_alias,
    tstr,
    GetFieldExt,
};

structural_alias!{
    trait Point<T>{
        x:T,
        y:T,
    }
}

fn print_point<T,U>(value:&T)
where
    T:Point<u32>
{
    // This gets references to the `x` and `y` fields.
    let (x,y)=value.fields(tstr!("x","y"));
    assert_ne!(x,y);
}

// TODO:add 3 structs deriving Structural,and pass them into the function.

# fn main(){}

```

### Defining a trait aliases with all accessibilities

```
use structural::{
    structural_alias,
    tstr,
    GetFieldExt,
};

structural_alias!{
    trait Person{
        // shared access (a & reference to the field)
        id:PersonId,
        
        // shared access (a & reference to the field)
        name:String,

        // mutable access (a &mut reference to the field),as well as shared access.
        mut friends:Vec<PersonId>,

        // by value access to the field (as well as shared and mutable)
        move candy:Candy,
    }
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Seconds(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct PersonId(u64);

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Candy;

# fn main(){}

```


*/
#[macro_export]
macro_rules! structural_alias{
    ( $($everything:tt)* )=>{
        structural_derive::structural_alias_impl!{ $($everything)* }
    }
}