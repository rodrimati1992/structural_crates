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

// Testing that a generic type with only these bounds can be converted to EnumPubs
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
    assert_eq!(struct_pubs_from((5, 8, 13)), StructPubs(5, 8));

    assert_eq!(enum_pubs_from(Enum4::Foo(5, 8)), EnumPubs::Foo(5));
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
    #[struc(public, init_with_lit = "default value")] pub &'static str,
    #[struc(init_with_default)] &'static [u8],
    #[struc(init_with_val = "Ordering::Less")] Ordering,
    #[struc(init_with_fn = "|| (false, true, true) ")] (bool, bool, bool),
);

#[derive(Debug, Structural, PartialEq)]
#[struc(from_structural)]
struct StructInitPrivMoreLits(
    pub u32,
    pub u64,
    #[struc(init_with_lit = b"hello")] &'static [u8],
    #[struc(init_with_lit = r#"world"#)] &'static str,
    #[struc(init_with_lit = true)] bool,
    #[struc(init_with_lit = 20)] u32,
    #[struc(init_with_lit = 10.0)] f32,
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
        #[struc(init_with_fn = r#"|| (true, false) "#)]
        what: (bool, bool),
    },
}

fn struct_init_priv_from<T>(from: T) -> StructInitPriv
where
    T: IntoField<TS!(0), Ty = u32> + IntoField<TS!(1), Ty = u64>,
{
    from.into_struc()
}

fn struct_init_priv_more_lits_from<T>(from: T) -> StructInitPrivMoreLits
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

#[test]
fn from_structural_with_priv_fields() {
    {
        let val = struct_init_priv_from((3, 5));
        let expected = StructInitPriv(
            3,
            5,
            "default value",
            b"",
            Ordering::Less,
            (false, true, true),
        );
        assert_eq!(val, expected);
    }
    {
        let val = struct_init_priv_more_lits_from((3, 5));
        let expected = StructInitPrivMoreLits(3, 5, b"hello", "world", true, 20, 10.0);
        assert_eq!(val, expected);
    }
    {
        assert_eq!(
            enum_init_priv_from(EnumPubs::Foo(13)),
            EnumInitPriv::Foo(13)
        );
        assert_eq!(enum_init_priv_from(EnumPubs::Bar), EnumInitPriv::Bar);
        assert_eq!(
            enum_init_priv_from(EnumPubs::Qux {
                uh: [3, 5, 8, 13],
                what: (false, true)
            }),
            EnumInitPriv::Qux {
                uh: [0; 4],
                what: (true, false)
            }
        );
    }
}
