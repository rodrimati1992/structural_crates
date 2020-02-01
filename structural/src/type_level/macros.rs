macro_rules! impl_cmp_traits {
    (
        impl[ $($impl_params:tt)* ] $self:ty
        where[ $($where_clause:tt)* ]
    ) => (
        impl<$($impl_params)*> std_::cmp::PartialEq for $self
        where
            $($where_clause)*
        {
            #[inline(always)]
            fn eq(&self,_other:&Self)->bool{
                true
            }
        }

        impl<$($impl_params)*> std_::cmp::Eq for $self
        where
            $($where_clause)*
        {}

        impl<$($impl_params)*> std_::cmp::PartialOrd for $self
        where
            $($where_clause)*
        {
            #[inline(always)]
            fn partial_cmp(&self,_other:&Self)->Option<std_::cmp::Ordering>{
                Some(std_::cmp::Ordering::Equal)
            }
        }

        impl<$($impl_params)*> std_::cmp::Ord for $self
        where
            $($where_clause)*
        {
            #[inline(always)]
            fn cmp(&self,_other:&Self)->std_::cmp::Ordering{
                std_::cmp::Ordering::Equal
            }
        }
    )
}
