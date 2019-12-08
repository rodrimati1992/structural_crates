use super::{Append_, Flatten, Flatten_, ToTList, ToTList_, ToTuple, ToTuple_};

macro_rules! tuple_impls {
    (with-idents;
        $( ($len:ty,$len_expr:expr)=[ $($tparams:ident,)* => $($runtparams:ident,)* ])*
    ) => {
        $(

            impl<$($tparams),*> ToTuple_ for TList![$($tparams),*] {
                type Output=($($tparams,)*);
            }

            impl<$($tparams),*> ToTuple_ for ($($tparams,)*) {
                type Output=($($tparams,)*);
            }

            impl<$($tparams),*> ToTList_ for ($($tparams,)*) {
                type Output=TList![$($tparams),*];
            }

            impl<$($tparams,)*> Flatten_ for ($($tparams,)*)
            where
                Self:ToTList_,
                ToTList<Self>:Flatten_,
                Flatten<ToTList<Self>>:ToTuple_
            {
                type Output=ToTuple<Flatten<ToTList<Self>>>;
            }

            impl<$($tparams,)* Other,Appended> Append_<Other> for ($($tparams,)*)
            where
                Self:ToTList_,
                Other:ToTList_,
                ToTList<Self>:Append_<ToTList<Other>,Output=Appended>,
                Appended:ToTuple_,
            {
                type Output=ToTuple<Appended>;
            }

        )*
    }
}

tuple_impls! {with-idents;
    (U0,0)=[
        =>

    ]
    (U1,1)=[
        C0,=>
        R0,
    ]
    (U2,2)=[
        C0,C1,=>
        R0,R1,
    ]
    (U3,3)=[
        C0,C1,C2,=>
        R0,R1,R2,
    ]
    (U4,4)=[
        C0,C1,C2,C3,=>
        R0,R1,R2,R3,
    ]
    (U5,5)=[
        C0,C1,C2,C3,C4,=>
        R0,R1,R2,R3,R4,
    ]
    (U6,6)=[
        C0,C1,C2,C3,C4,C5,=>
        R0,R1,R2,R3,R4,R5,
    ]
    (U7,7)=[
        C0,C1,C2,C3,C4,C5,C6,=>
        R0,R1,R2,R3,R4,R5,R6,
    ]
    (U8,8)=[
        C0,C1,C2,C3,C4,C5,C6,C7,=>
       R0,R1,R2,R3,R4,R5,R6,R7,
    ]
    (U9,9)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,
    ]
    (U10,10)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,
    ]
    (U11,11)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,
    ]
    (U12,12)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,
    ]
    (U13,13)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,
    ]
    (U14,14)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,
    ]
    (U15,15)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,R14,
    ]
    (U16,16)=[
        C0,C1,C2,C3,C4,C5,C6,C7,C8,C9,C10,C11,C12,C13,C14,C15,=>
        R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,R10,R11,R12,R13,R14,R15,
    ]
}
