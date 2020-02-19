/*!

This library provides field accessor traits,and emulation of structural types.

# Features

These are the features this library provides:

- [Derivation of the 3 accessor traits for every public field](./docs/structural_macro/index.html)
(GetFieldImpl/GetFieldMutImpl/IntoFieldImpl),
and aliases for the optional and non-optional variants of those traits.

- [Declaration of trait aliases for accessor trait bounds,using field-in-trait syntax.
](./macro.structural_alias.html).

- [The `impl_struct` macro to declare structural parameter/return types
](./macro.impl_struct.html)(available from Rust 1.40 onwards),
as well as [`make_struct` to construct anonymous structs ](./macro.make_struct.html)


# Examples


### Structural Derive for structs

This demonstrates how you can use any type with a superset of the
fields of another one in a function.

For details on the [Structural derive macro look here](./docs/structural_macro/index.html).

```rust
use structural::{GetFieldExt,Structural,fp};


fn reads_point4<S>(point:&S)
where
    // The `Structural` derive generated the `Point3D_SI` trait for `Point3D`,
    // aliasing the accessor traits for it.
    S:Point3D_SI<u32>
{
    let (a,b,c)=point.fields(fp!( x, y, z ));

    assert_eq!(a,&0);
    assert_eq!(b,&11);
    assert_eq!(c,&33);
}

fn main(){
    reads_point4(&Point3D { x: 0, y: 11, z: 33 });

    reads_point4(&Point4D {
        x: 0,
        y: 11,
        z: 33,
        a: 0xDEAD,
    });

    reads_point4(&Point5D {
        x: 0,
        y: 11,
        z: 33,
        a: 0xDEAD,
        b: 0xBEEF,
    });
}


#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Point3D<T>{
    x:T,
    y:T,
    z:T,
}

#[derive(Structural)]
// By default only public fields get accessor trait impls,
// using `#[struc(public)]` you can have impls to access private fields.
#[struc(public)]
struct Point4D<T>{
    x:T,
    y:T,
    z:T,
    a:T,
}

#[derive(Structural)]
struct Point5D<T>{
    pub x:T,
    pub y:T,
    pub z:T,
    pub a:T,
    pub b:T,
}

```

### Structural Derive for enums

This demonstrates how you can use structural enums.

For details on [enums look here](./docs/structural_macro/enums/index.html).

```rust
use structural::{GetFieldExt,Structural,fp,switch};

fn main(){
    {
        // Command

        run_command(Command::SendEmail(SendEmail{
            to:"ferris@lib.rs".to_string(),
            content:"Hello".to_string(),
        }));
        run_command(Command::RemoveAddress("gopher".to_string()));
    }
    {
        // ExtraCommand
        //
        // ExtraCommand can't be passed to `run_command` because that function requires
        // an enum with exactly the `SendEmail` and `RemoveAddress` variants.

        // The `SendEmail` variant can have more fields than the one in `Command`,
        // they're just ignored.
        run_command_nonexhaustive(ExtraCommand::SendEmail{
            to:"squatter@crates.io".to_string(),
            content:"Can you stop squatting crate names?".to_string(),
            topic:"squatting".to_string(),
        }).unwrap();

        let ra_cmd=ExtraCommand::RemoveAddress("smart_person".to_string());
        run_command_nonexhaustive(ra_cmd).unwrap();

        let ca_cmd=ExtraCommand::CreateAddress("honest_person".to_string());
        let res=run_command_nonexhaustive(ca_cmd.clone());
        assert_eq!( res, Err(UnsupportedCommand(ca_cmd)) );
    }
}

// Runs the passed in command.
//
// The `Command_ESI` trait allows only enums with the same variants as
// `Command` to be passed in(they can have a superset of the fields in `Command`).
fn run_command<S>(cmd:S)
where
    S:Command_ESI
{
    run_command_nonexhaustive(cmd)
        .ok()
        .expect("`run_command_nonexhaustive` must match all `Command` variants")
}

// Runs the passed in command.
//
// The `Command_ESI` trait allows enums with a superset of the variants in `Command`
// to be passed in,
// requiring the a `_=>` branch when it's matched on with the `switch` macro.
fn run_command_nonexhaustive<S>(cmd:S)->Result<(),UnsupportedCommand<S>>
where
    S:Command_SI
{
    switch!{cmd;
        // This matches the SendEmail variant and destructures it into the
        // `to` and `content` fields (by reference,because of the `ref`).
        ref SendEmail{to,content}=>{
            println!("Sending message to the '{}' email address.",to);
            println!("Content:{:?}",content);
            Ok(())
        }
        // `cmd` is moved into the branch here,
        // wrapped into a `VariantProxy<S,FP!(RemoveAddress)>`,
        // which allows non-optional access to the fields in the variant.
        //
        // This does not destructure the variant because
        // it's not possible to unwrap a structural type into multiple fields yet
        // (special casing the single field case doesn't seem like a good idea).
        RemoveAddress=>{
            let address=cmd.into_field(fp!(0));
            println!("removing the '{}' email address",address);
            Ok(())
        }
        _=>Err(UnsupportedCommand(cmd))
    }
}

#[derive(Structural)]
enum Command{
    // The `newtype(bounds="...")` attribute marks the variant as being a newtype variant,
    // delegating field accessors for the variant to `SendEmail`(its one field),
    // as well as replacing the bounds for the variant in the generated
    // `Command_SI` and `Command_ESI` traits with `SendEmail_VSI<TStr!(SendEmail)>`.
    //
    // `SendEmail_VSI` was generated by the `Structural` derive on `SendEmail`,
    // with accessor trait bounds for accessing the struct's fields
    // in a variant (it takes the name of the variant as a generic parameter).
    #[struc(newtype(bounds="SendEmail_VSI<@variant>"))]
    SendEmail(SendEmail),
    RemoveAddress(String),
}

#[derive(Structural)]
pub struct SendEmail{
    pub to: String,
    pub content: String,
}

#[derive(Debug,Structural,Clone,PartialEq)]
// This attribute stops the generation of the
// `ExtraCommands_SI` and `ExtraCommands_ESI` traits
#[struc(no_trait)]
pub enum ExtraCommand{
    SendEmail{
        to: String,
        content: String,
        topic: String,
    },
    RemoveAddress(String),
    CreateAddress(String),
}

#[derive(Debug,PartialEq)]
pub struct UnsupportedCommand<T>(pub T);

```

### Structural alias for struct

This demonstrates how you can define a trait aliasing field accessors,
using a fields-in-traits syntax.

For more details you can look at the docs for the
[`structural_alias`](./macro.structural_alias.html) macro.

```rust

use structural::{GetFieldExt,Structural,structural_alias,fp};

use std::borrow::Borrow;

structural_alias!{
    trait Person<H:House>{
        name:String,
        house:H,
    }

    trait House{
        dim:Dimension3D,
    }
}


fn print_name<T,H>(this:&T)
where
    T:?Sized+Person<H>,
    H:House,
{
    let (name,house_dim)=this.fields(fp!( name, house.dim ));
    println!("Hello, {}!", name);

    let (w,h,d)=house_dim.fields(fp!( width, height, depth ));

    if w*h*d >= 1_000_000 {
        println!("Your house is enormous.");
    }else{
        println!("Your house is normal sized.");
    }
}

// most structural aliases are object safe
fn print_name_dyn<H>(this:&dyn Person<H>)
where
    H:House,
{
    print_name(this)
}



#[derive(Structural)]
#[struc(public)]
struct Dimension3D{
    width:u32,
    height:u32,
    depth:u32,
}

//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate


fn main(){
    let worker=Worker{
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
        house:Mansion{
            dim:Dimension3D{
                width:300,
                height:300,
                depth:300,
            },
            money_vault_location:"In the basement".into(),
        }
    };

    let student=Student{
        name:"Jake English".into(),
        birth_year:1995,
        house:SmallHouse{
            dim:Dimension3D{
                width:30,
                height:30,
                depth:30,
            },
            residents:10,
        }
    };

    print_name(&worker);
    print_name(&student);

    print_name_dyn(&worker);
    print_name_dyn(&student);
}


#[derive(Structural)]
// Using the `#[struc(public)]` attribute tells the derive macro to
// generate the accessor trait impls for non-`pub` fields.
#[struc(public)]
struct Worker{
    name:String,
    salary:Cents,
    house:Mansion,
}

#[derive(Structural)]
#[struc(public)]
struct Student{
    name:String,
    birth_year:u32,
    house:SmallHouse,
}

# #[derive(Debug,Copy,Clone,PartialEq,Eq)]
# struct Cents(u64);

#[derive(Structural)]
#[struc(public)]
struct Mansion{
    dim:Dimension3D,
    money_vault_location:String,
}

#[derive(Structural)]
#[struc(public)]
struct SmallHouse{
    dim:Dimension3D,
    residents:u32,
}

```

### Structural alias for enums

This demonstrates how you can use structural aliases for enums.

This shows both exhaustive and nonexhaustive enum structural aliases,
by using the `#[struc(and_exhaustive_enum(suffix="_Ex"))]` attribute when declaring the
trait inside of `structural_alias`.<br>
You can also use the `#[struc(exhaustive_enum)]` attribute to make the annotated trait
itself exhaustive instead of having two traits.

```rust
use structural::{GetFieldExt,Structural,structural_alias,switch,fp};
use std::fmt::Debug;

# fn main(){
pet_animal_ex(&SomeMammals::Dog{years:1,volume_cm3:1});
pet_animal_ex(&SomeMammals::Horse);

// `MoreAnimals` cannot be passed to `pet_animal_ex`
// since that function requires an enum with only `Dog` and `Horse` variants.
assert_eq!( pet_animal(&MoreAnimals::Dog{years:10,volume_cm3:100}), Ok(()) );
assert_eq!( pet_animal(&MoreAnimals::Horse), Ok(()) );
assert_eq!( pet_animal(&MoreAnimals::Cat{lives:9}), Err(CouldNotPet) );
assert_eq!( pet_animal(&MoreAnimals::Seal), Err(CouldNotPet) );
# }

fn pet_animal(animal: &dyn Animal)-> Result<(),CouldNotPet> {
    // `::Dog` accesses the `Dog` variant
    // (without the `::` it'd be interpreted as a field access),
    // The `=>` allows getting multiple fields from inside a nested field
    // (this includes enum variants).
    // `years,volume_cm3` are the field accessed from inside `::Dog`
    let dog_fields = fp!(::Dog=>years,volume_cm3);

    if animal.is_variant(fp!(Horse)) {
        println!("You are petting the horse");
    }else if let Some((years,volume_cm3))= animal.fields(dog_fields) {
        println!("You are petting the {} year old,{} cm³ dog",years,volume_cm3);
    }else{
        return Err(CouldNotPet);
    }
    Ok(())
}

// This can't take a `&dyn Animal_Ex` because traits objects don't
// automatically support upcasting into other trait objects
// (except for auto traits like Send and Sync ).
fn pet_animal_ex(animal: &impl Animal_Ex) {
    pet_animal(animal)
        .expect("`pet_animal` must match on all variants from the `Animal` trait");
}

// The same as `pet_animal` ,except that this uses a `switch`
fn pet_animal_switch(animal: &dyn Animal)-> Result<(),CouldNotPet> {
    switch!{animal;
        ref Horse=>{
            println!("You are petting the horse");
        }
        ref Dog{years,volume_cm3}=>{
            println!("You are petting the {} year old,{} cm³ dog",years,volume_cm3);
        }
        _=>return Err(CouldNotPet)
    }
    Ok(())
}


#[derive(Debug,PartialEq)]
struct CouldNotPet;

structural_alias!{
    // The `#[struc(and_exhaustive_enum(suffix="_Ex"))]` attribute
    // generates the `Animal_Ex` trait with this trait as a supertrait,
    // and with the additional requirement that the enum
    // only has the `horse` and `Dog` variants
    // (They variants can still have more fields than required).
    //
    // structural aliases can have supertraits,here it's `Debug`
    #[struc(and_exhaustive_enum(suffix="_Ex"))]
    trait Animal: Debug{
        Horse,
        Dog{years:u16,volume_cm3:u64},
    }
}


#[derive(Debug,Structural)]
# #[struc(no_trait)]
enum SomeMammals{
    Horse,
    Dog{years:u16,volume_cm3:u64},
}

#[derive(Debug,Structural)]
# #[struc(no_trait)]
enum MoreAnimals{
    Cat{lives:u8},
    Dog{years:u16,volume_cm3:u64},
    Horse,
    Seal,
}



```

### Anonymous structs (`make_struct` macro)

This demonstrates how you can construct an anonymous struct.

For more details you can look at the docs for the
[`make_struct`](./macro.make_struct.html) macro.

For a macro to declare a structural type directly as a parameter or return type
(usable from Rust 1.40) you can look at the
[`impl_struct`](./macro.impl_struct.html) macro.

```rust

use structural::{GetFieldExt,make_struct,structural_alias,fp};

structural_alias!{
    trait Person<T>{
        // We only have shared access (`&String`) to the field.
        ref name:String,

        // We have shared,mutable,and by value access to the field.
        // Not specifying any of `mut`/`ref`/`move` is equivalent to `mut move value:T,`
        value:T,
    }
}

// From Rust 1.40 you can use
// `impl_struct!{ ref name:String, value:() }` as the return type,
// which is equivalent to `Person<()>`.
fn make_person(name:String)->impl Person<()> {
    make_struct!{
        name,
        value: (),
    }
}


fn print_name<T>(mut this:T)
where
    T:Person<Vec<String>>,
{
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
    assert_eq!( this.into_field(fp!(value)), list );
}


// most structural aliases are object safe
fn print_name_dyn(this:&mut dyn Person<Vec<String>>){
    println!("Hello, {}!",this.field_(fp!(name)) );

    let list=vec!["what".into()];
    *this.field_mut(fp!(value))=list.clone();
    assert_eq!( this.field_(fp!(value)), &list );
}

//////////////////////////////////////////////////////////////////////////
////          The stuff here could be defined in a separate crate

fn main(){
    let worker=make_struct!{
        // This derives clone for the anonymous struct
        #![derive(Clone)]
        name:"John Doe".into(),
        salary:Cents(1_000_000_000_000_000),
        value:vec![],
    };

    let student=make_struct!{
        // This derives clone for the anonymous struct
        #![derive(Clone)]
        name:"Jake English".into(),
        birth_year:1995,
        value:vec![],
    };

    print_name(worker.clone());
    print_name(student.clone());

    print_name_dyn(&mut worker.clone());
    print_name_dyn(&mut student.clone());

    let person=make_person("Louis".into());

    assert_eq!( person.field_(fp!(name)), "Louis" );
    assert_eq!( person.field_(fp!(value)), &() );
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Cents(u64);

```



*/
#![cfg_attr(feature = "nightly_impl_fields", feature(associated_type_bounds))]
#![cfg_attr(feature = "nightly_specialization", feature(specialization))]
#![cfg_attr(feature = "nightly_better_macros", feature(proc_macro_hygiene))]
#![cfg_attr(feature = "use_const_str", feature(const_if_match))]
#![cfg_attr(feature = "use_const_str", feature(const_generics))]
#![deny(rust_2018_idioms)]
#![no_std]

#[cfg(any(all(feature = "alloc", not(feature = "rust_1_36")), feature = "std",))]
pub extern crate std;

#[doc(hidden)]
pub extern crate core as std_;

#[doc(hidden)]
#[cfg(all(feature = "alloc", feature = "rust_1_36"))]
pub extern crate alloc as alloc_;

#[doc(hidden)]
#[cfg(all(feature = "alloc", feature = "rust_1_36"))]
pub use alloc_ as alloc;

#[doc(hidden)]
#[cfg(all(feature = "alloc", not(feature = "rust_1_36")))]
pub use std as alloc;

extern crate self as structural;

pub use structural_derive::Structural;

#[doc(hidden)]
pub use structural_derive::{
    _FP_impl_, _TStr_from_concatenated_chars, _TStr_impl_, _field_path_aliases_impl,
    _impl_struct_impl, _switch_tstring_aliases, _tstr_impl_, _tstring_aliases_impl, low_fp_impl_,
    structural_alias_impl,
};

#[macro_use]
mod macros;

#[cfg(feature = "use_const_str")]
pub mod const_generic_utils;
pub mod docs;
pub mod enums;
pub mod field_path;
pub mod field_traits;
pub mod mut_ref;
pub mod structural_trait;
pub mod utils;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
pub mod tests {
    mod delegation;
    mod enum_derive;
    #[cfg(feature = "rust_1_40")]
    mod impl_struct;
    //TODO: Adapt these tests to also work with const generics
    #[cfg(not(feature = "use_const_str"))]
    mod macro_tests;
    mod multi_nested_fields;
    mod optional_fields;
    mod structural_alias;
    mod structural_derive;
    mod switch;
}

pub mod type_level;

#[doc(hidden)]
pub mod p;

#[doc(inline)]
pub use crate::field_traits::GetFieldExt;

pub use crate::{
    field_traits::{
        FieldType, GetField, GetFieldImpl, GetFieldMut, GetFieldMutImpl, GetFieldType,
        GetFieldType2, GetFieldType3, GetFieldType4, IntoField, IntoFieldImpl, IntoFieldMut,
        OptGetField, OptGetFieldMut, OptIntoField, OptIntoFieldMut,
    },
    structural_trait::{IsStructural, Structural},
};

/// Reexports from the `core_extensions` crate.
pub mod reexports {
    pub use core_extensions::{
        collection_traits::{Cloned, IntoArray},
        type_asserts::AssertEq,
        MarkerType, SelfOps, TIdentity, TypeIdentity,
    };
}

// pmr(proc macro reexports):
// Reexports for the proc macros in structural_derive.
//
// Importing stuff from this module anywhere other than `structural_derive` is
// explicitly disallowed,and is likely to break.
#[doc(hidden)]
pub mod pmr {
    pub use crate::enums::variant_count::*;
    pub use crate::enums::*;
    pub use crate::field_path::*;
    pub use crate::field_traits::variant_field::*;
    pub use crate::field_traits::*;
    pub use crate::structural_trait::IsStructural;
    pub use crate::type_level::collection_traits::*;
    pub use crate::type_level::*;
    pub use crate::utils::{as_phantomdata, OptionParam, _Structural_BorrowSelf};
    pub use core_extensions::type_level_bool::{Boolean, False, True};
    pub use core_extensions::{MarkerType, TIdentity, TypeIdentity};

    pub use crate::std_::{
        hint::unreachable_unchecked,
        marker::PhantomData,
        mem::drop,
        option::Option::{self, None, Some},
    };

    #[cfg(feature = "alloc")]
    pub use crate::alloc::boxed::Box;
}

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

//////////////////////////////

use std_::marker::PhantomData;
use std_::mem::ManuallyDrop;

include! {"field_path/declare_field_path_types.rs"}

//////////////////////////////
