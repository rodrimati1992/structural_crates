use structural::{
    enums::IsVariant, for_examples::Enum4, IntoField, IntoVariantField, Structural, StructuralExt,
    TS,
};

use std::cmp::Ordering;

#[derive(Debug, Structural, PartialEq)]
#[struc(from_structural)]
struct StructPubs(pub u32, pub u64);

#[derive(Debug, Structural, PartialEq)]
#[struc(from_structural)]
//#[struc(debug_print)]
enum EnumPubs {
    Foo(u8),
    Bar,
    Qux { uh: [u8; 4], what: (bool, bool) },
}

fn struct_pubs_from<T>(from: T) -> StructPubs
where
    T: IntoField<TS!(0), Ty = u32> + IntoField<TS!(1), Ty = u64>,
{
    from.into_struc()
}

fn enum_pubs_from<T>(from: T) -> EnumPubs
where
    T: IntoVariantField<TS!(Foo), TS!(0), Ty = u8>
        + IsVariant<TS!(Bar)>
        + IntoVariantField<TS!(Qux), TS!(uh), Ty = [u8; 4]>
        + IntoVariantField<TS!(Qux), TS!(what), Ty = (bool, bool)>,
{
    from.into_struc()
}

#[test]
fn all_pub_fields() {
    assert_eq!((5, 8, 13).into_struc::<StructPubs>(), StructPubs(5, 8));

    assert_eq!(Enum4::Foo(5, 8).into_struc::<EnumPubs>(), EnumPubs::Foo(5));
    assert_eq!(
        Enum4::Bar(Ordering::Less, None).into_struc::<EnumPubs>(),
        EnumPubs::Bar,
    );
    assert_eq!(
        Enum4::Qux {
            uh: [3, 5, 8, 13],
            what: (true, false)
        }
        .into_struc::<EnumPubs>(),
        EnumPubs::Qux {
            uh: [3, 5, 8, 13],
            what: (true, false)
        },
    );
}

#[derive(Debug, Structural, PartialEq)]
#[struc(from_structural)]
struct StructInitPriv(
    pub u32,
    pub u64,
    #[struc(init_with_lit = "default value")] pub &'static str,
    #[struc(public, init_with_default)] &'static [u8],
    #[struc(init_with_val = "Ordering::Less")] Ordering,
    #[struc(init_with_fn = "|| (false, true, true) ")] (bool, bool, bool),
);

#[derive(Debug, Structural, PartialEq)]
#[struc(from_structural)]
//#[struc(debug_print)]
enum EnumInitPriv {
    Foo(u8),
    Bar,
    Qux {
        #[struc(init_with_default)]
        uh: [u8; 4],
        #[struc(init_with_fn = "|| (true, false) ")]
        what: (bool, bool),
    },
}

fn struct_init_priv_from<T>(from: T) -> StructInitPriv
where
    T: IntoField<TS!(0), Ty = u32> + IntoField<TS!(1), Ty = u64>,
{
    from.into_struc()
}

fn enum_init_priv_from<T>(from: T) -> EnumInitPriv
where
    T: IntoVariantField<TS!(Foo), TS!(0), Ty = u8> + IsVariant<TS!(Bar)> + IsVariant<TS!(Qux)>,
{
    from.into_struc()
}
