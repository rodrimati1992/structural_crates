/**

Type macro for a type-list.

This macro creates an ordered sequence of types,

# Example

This example is equivalent to `TList<String,TList<&'a str,TList<Cow<'a,str>,Nil>>>`.

```
use structural::TList;

use std::borrow::Cow;

type StringsTypes<'a>=TList![ String,&'a str,Cow<'a,str> ];

# fn main(){}

```

*/
#[macro_export]
#[doc(hidden)]
macro_rules! TList {
    () => { $crate::type_level::TNil };
    (..$rest:ty) => { $rest };
    ($current:ty) => { $crate::TList![$current,] };
    ($elem_0:ty,$elem_1:ty,$elem_2:ty,$elem_3:ty,$elem_4:ty, $($rest:tt)*) => {
        $crate::type_level::TList<
            $elem_0,
            $crate::type_level::TList<
                $elem_1,
                $crate::type_level::TList<
                    $elem_2,
                    $crate::type_level::TList<
                        $elem_3,
                        $crate::type_level::TList<
                            $elem_4,
                            $crate::TList![$($rest)*]
                        >
                    >
                >
            >
        >
    };
    ($elem_0:ty,$elem_1:ty,$elem_2:ty,$elem_3:ty, $($rest:tt)*) => {
        $crate::type_level::TList<
            $elem_0,
            $crate::type_level::TList<
                $elem_1,
                $crate::type_level::TList<
                    $elem_2,
                    $crate::type_level::TList<
                        $elem_3,
                        $crate::TList![$($rest)*]
                    >
                >
            >
        >
    };
    ($elem_0:ty,$elem_1:ty,$elem_2:ty, $($rest:tt)*) => {
        $crate::type_level::TList<
            $elem_0,
            $crate::type_level::TList<
                $elem_1,
                $crate::type_level::TList<
                    $elem_2,
                    $crate::TList![$($rest)*]
                >
            >
        >
    };
    ($elem_0:ty,$elem_1:ty, $($rest:tt)*) => {
        $crate::type_level::TList<
            $elem_0,
            $crate::type_level::TList<
                $elem_1,
                $crate::TList![$($rest)*]
            >
        >
    };
    ($current:ty, $($rest:tt)*) => {
        $crate::type_level::TList<$current,$crate::TList![$($rest)*]>
    };
}

/**

Instantiates a type-list,
which is a zero-sized-type which does not contain instances of the types it lists.

# Example

```
use structural::{tlist,TList};

use std::borrow::Cow;

fn main(){

    const STRINGS:
        TList![ String,&str,Cow<str> ]=
        tlist![ String,&str,Cow<str> ];

}

```


*/
#[macro_export]
#[doc(hidden)]
macro_rules! tlist {
    ($($all:tt)*) => {
        <TList!($($all)*)>::NEW
    };
}
