/*!

The Structural derive macro implements the Structurl trit,as well as accessor traits(GetField/GetFieldMut/IntoField) for fields.

# Default Behavior

By default,this derive generates:

- Implementation of the structural trait for the deriving type.

- Implementations of the accessor traits (GetField/GetFieldMut/IntoField) for pub fields.

- A trait named `<deriving_type>_SI`,aliasing the accessor traits for the type,
with a blanket implementation for all types with the same fields.

- Only the GetField trait (by reference accessor) will be implemented for fields,
requiring use of the `#[struc(access="...")]` attribute to implement extra traits.

# Container Attributes

### `#[struc(debug_print)]`

Prints the output of the derive macro by panicking.


# Field Attributes

### `#[struc(rename="<new_name>")]`

Changes the name for the field in the accessor trait impls.


# Container/Field Attributes

Unless stated otherwise,
when these attributes are put on the container it will have the same effect as 
being put on the field,and are overriden by attributes directly on the field.

### `#[struc(public)]`

Marks the fields as public,generating the accessor traits for the field.

### `#[struc(not_public)]`

Marks the fields as private,not generating the accessor traits for the field.

### `#[struc(access="")]`

Changes the implemented accessor traits for the field(s).

`#[struc(access="ref")]`:
Generates impls of the `GetField` trait for the field(s).

`#[struc(access="mut")]`:
Generates impls of the `GetField`+`GetFieldMut` for the field(s).

`#[struc(access="move")]`:
Generates impls of the `GetField`+`GetFieldMut`+`IntoField` for the field(s).

# Examples

### Basic example

```rust
use structural::{Structural,GetFieldExt,structural_alias,tstr};


structural_alias!{
    trait Pair<T>{
        a:T,
        a:T,
    }
}


fn reads_pair<O>(pair:&O)
where
    O:Pair<u32>
{
    let (a,b)=pair.fields(tstr!("a","b"));
    assert_eq!(a,&11);
    assert_eq!(b,&33);
}


#[derive(Structural)]
struct Hello{
    a:u32,
    b:u32
}

#[derive(Structural)]
#[struc(access="move")]
struct World{
    run:String,
    a:u32,
    b:u32,
}

reads_pair(&Hello{ a:11, b:33 });

reads_pair(&World{ run:"nope".into(), a:11, b:33 });


```

### Mutating fields

```rust
use structural::{Structural,GetFieldExt,structural_alias,tstr};


structural_alias!{
    trait Tuple2<T>{
        move 0:T,
        move 1:T,
    }
}


fn mutates_pair<O>(pair:&mut O)
where
    O:Tuple2<u32>
{
    let (a,b)=pair.fields_mut(tstr!("0","1"));
    assert_eq!(a,&mut 14);
    assert_eq!(b,&mut 16);
    *a*=2;
    *b*=2;
}


#[derive(Structural)]
#[struc(access="move")]
struct Point(
    #[struc(public)]
    u32,
    // This attribute isn't redundant,it causes the field to get accessor trait impls.
    #[struc(access="move")]
    u32,
    #[struc(not_public)]
    pub u32,
);

let mut point=Point(14,16,11);
let mut tuple=(14,16);

mutates_pair(&mut point);
mutates_pair(&mut tuple);

assert_eq!(point,Point(28,32,11));
assert_eq!(tuple,(28,32));

```


*/
