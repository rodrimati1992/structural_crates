use super::*;

use crate::type_level::ident::{TString,_0,_1,_2,_3,_4,_5,_6,_7,_8,_9};

macro_rules! impl_tuple {
    (inner; ($field:tt,$field_ty:ident,$field_param:ty) ($($tuple_param:ident),*) )=>{
        impl_getter!{
            unsafe impl[$($tuple_param),*] 
                IntoField< $field:$field_ty,$field_param,{$field} > 
            for ($($tuple_param,)*)
        }
    };
    (
        [
            $( ($field:tt,$field_ty:ident,$field_param:ty) ),*
        ]
        $tuple_ty:tt        
    ) => {
        $(
            impl_tuple!{
                inner;
                ($field,$field_ty,$field_param) $tuple_ty
            }
        )*
    }
}


/*
Code used to generate the macro invocations


use itertools::Itertools; 

fn main(){
    for x in 0..12 {
        let range=0..x;
        println!(
            "impl_tuple!{{\n\
                {I4}[\n\
                {I8}{}\n\
                {I4}]\n\
                {I4}({})\n\
            }}",
            range.clone().map(|x|format!("({0},C{0},U{0})",x)).join(",\n        "),
            range.clone().map(|x|format!("C{0}",x)).join(","),
            I4="    ",
            I8="        ",
        );
    }
}

*/


impl_tuple!{
    [
        (0,C0,TString<(_0,)>)
    ]
    (C0)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>)
    ]
    (C0,C1)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>)
    ]
    (C0,C1,C2)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>)
    ]
    (C0,C1,C2,C3)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>)
    ]
    (C0,C1,C2,C3,C4)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>)
    ]
    (C0,C1,C2,C3,C4,C5)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>),
        (6,C6,TString<(_6,)>)
    ]
    (C0,C1,C2,C3,C4,C5,C6)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>),
        (6,C6,TString<(_6,)>),
        (7,C7,TString<(_7,)>)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>),
        (6,C6,TString<(_6,)>),
        (7,C7,TString<(_7,)>),
        (8,C8,TString<(_8,)>)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>),
        (6,C6,TString<(_6,)>),
        (7,C7,TString<(_7,)>),
        (8,C8,TString<(_8,)>),
        (9,C9,TString<(_9,)>)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>),
        (6,C6,TString<(_6,)>),
        (7,C7,TString<(_7,)>),
        (8,C8,TString<(_8,)>),
        (9,C9,TString<(_9,)>),
        (10,C10,TString<(_1,_0)>)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10)
}
impl_tuple!{
    [
        (0,C0,TString<(_0,)>),
        (1,C1,TString<(_1,)>),
        (2,C2,TString<(_2,)>),
        (3,C3,TString<(_3,)>),
        (4,C4,TString<(_4,)>),
        (5,C5,TString<(_5,)>),
        (6,C6,TString<(_6,)>),
        (7,C7,TString<(_7,)>),
        (8,C8,TString<(_8,)>),
        (9,C9,TString<(_9,)>),
        (10,C10,TString<(_1,_0)>),
        (11,C11,TString<(_1,_1)>)
    ]
    (C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11)
}



#[cfg(test)]
mod tests{
    use super::*;

    fn get_field_1<T>(val:&T)->&u64
    where
        T:GetField<TStr!(1),Ty=u64>,
    {
        val.field_(tstr!("1"))
    }


    #[test]
    fn get_field_1_test(){
        assert_eq!(*get_field_1(&(2,8)), 8);
        assert_eq!(*get_field_1(&(2,5,8)), 5);
        assert_eq!(*get_field_1(&(2,3,5,8)), 3);
        assert_eq!(*get_field_1(&(1,2,3,5,8)), 2);
        assert_eq!(*get_field_1(&(1,1,2,3,5,8)), 1);
        assert_eq!(*get_field_1(&(11,13,17,19,23,29,31)), 13);
        assert_eq!(*get_field_1(&(7,11,13,17,19,23,29,31)), 11);
        assert_eq!(*get_field_1(&(5,7,11,13,17,19,23,29,31)), 7);
        assert_eq!(*get_field_1(&(3,5,7,11,13,17,19,23,29,31)), 5);
    }


    #[test]
    fn get_mut_many(){
        {
            let mut tup=(0,1,2,3,4,5);
            let (e0,e1)=tup.fields_mut(tstr!("0","1"));
            *e0=101;
            *e1=102;

            assert_eq!(tup.0,101);
            assert_eq!(tup.1,102);
        }
        {
            let mut tup=(0,1,2,3,4,5);
            let (e0,e1,e2)=tup.fields_mut(tstr!("0","1","3"));
            *e0=101;
            *e1=102;
            *e2=103;

            assert_eq!(tup.0,101);
            assert_eq!(tup.1,102);
            assert_eq!(tup.2,2);
            assert_eq!(tup.3,103);
            assert_eq!(tup.4,4);
        }
        {
            let mut tup=(0,1,2,3,4,5,6,7,8);
            let (e0,e1,e2,e3)=tup.fields_mut(tstr!("0","1","2","8"));
            *e0=101;
            *e1=102;
            *e2=103;
            *e3=200;

            assert_eq!(tup.0,101);
            assert_eq!(tup.1,102);
            assert_eq!(tup.2,103);
            assert_eq!(tup.3,3);
            assert_eq!(tup.4,4);
            assert_eq!(tup.7,7);
            assert_eq!(tup.8,200);
        }
    }


    structural_alias!{
        trait Tuple4{
            move 0:u32,
            move 1:u32,
            move 2:u32,
            move 3:u32,
        }
    }



    fn takes_tuple4<This>(mut this:This)
    where
        This:Tuple4,
    {
        assert_eq!(this.fields(tstr!("0","1")),(&6,&5));
        assert_eq!(this.fields(tstr!("0","1","2")),(&6,&5,&4));
        assert_eq!(this.fields(tstr!("0","1","2","3")),(&6,&5,&4,&3));


        assert_eq!(this.fields_mut(tstr!("0","1")),(&mut 6,&mut 5));
        assert_eq!(this.fields_mut(tstr!("0","1","2")),(&mut 6,&mut 5,&mut 4));
        assert_eq!(this.fields_mut(tstr!("0","1","2","3")),(&mut 6,&mut 5,&mut 4,&mut 3));
    }


    #[test]
    fn tuple4_test(){
        takes_tuple4((6,5,4,3,2,1));
        takes_tuple4((6,5,4,3,2));
        takes_tuple4((6,5,4,3));
    }

}