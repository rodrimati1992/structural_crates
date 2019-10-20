macro_rules! impl_getter{
    ( impl[$($typarams:tt)*]
        GetField <$field_name:tt : $field_ty:ty,$name_param:tt,{$index:expr}> 
        for $self_:ty 
    )=>{
        impl<$($typarams)*> GetField<$name_param> for $self_ {
            type Ty=$field_ty;

            const INDEX:usize=$index;

            const NAME:&'static str=stringify!($field_name);
            
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
