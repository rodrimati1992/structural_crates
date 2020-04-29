use structural::{
    field::{Array5, Array5Variant},
    fp, Structural, StructuralExt, TS,
};

use structural::test_utils::OrOnDrop;

use std::cell::Cell;

mod drop_order;

#[cfg(feature = "alloc")]
#[test]
fn tuple_into_fields() {
    let number = Cell::new(0u64);
    fn make_tuple<'a, T>(
        mutref: T,
        number: &'a Cell<u64>,
    ) -> (
        OrOnDrop<'a, Box<String>>,
        OrOnDrop<'a, Vec<u32>>,
        OrOnDrop<'a, T>,
        OrOnDrop<'a, Box<[char]>>,
        OrOnDrop<'a, ()>,
    ) {
        (
            OrOnDrop::new(Box::new("Hello".to_string()), number, 1),
            OrOnDrop::new(vec![0, 1], number, 1 << 1),
            OrOnDrop::new(mutref, number, 1 << 2),
            OrOnDrop::new(vec!['a', 'b'].into_boxed_slice(), number, 1 << 3),
            OrOnDrop::new((), number, 1 << 4),
        )
    }

    {
        number.set(0);
        let mut list = vec![3, 5];
        let tuple = make_tuple(&mut list, &number);
        assert_eq!(number.get(), 0);
        let (f0, f2) = tuple.into_fields(fp!(0, 2));
        assert_eq!(f0.bits_to_set(), 1);
        assert_eq!(f2.bits_to_set(), 1 << 2);

        // ensuring that the non-moved out fields were dropped
        assert_eq!(number.get(), 0b11010);

        let mut f0 = f0.into_inner();
        let f2 = f2.into_inner();

        assert_eq!(*f0, "Hello");
        f0.push_str(", world!");
        assert_eq!(*f0, "Hello, world!");

        f2.push(8);
        assert_eq!(*f2, &[3, 5, 8][..]);

        assert_eq!(number.get(), 0b11111);
    }

    {
        number.set(0);
        let mut list = vec![3, 5];
        let tuple = make_tuple(&mut list, &number);
        assert_eq!(number.get(), 0);
        let (f3, f4) = tuple.into_fields(fp!(3, 4));
        assert_eq!(f3.bits_to_set(), 1 << 3);
        assert_eq!(f4.bits_to_set(), 1 << 4);

        // ensuring that the non-moved out fields were dropped
        assert_eq!(number.get(), 0b00111);

        let mut f3 = f3.into_inner();
        drop(f4);

        f3[0] = 'c';
        f3[1] = 'd';
        assert_eq!(&*f3, &['c', 'd'][..]);

        assert_eq!(number.get(), 0b11111);
    }
}

fn make_array_a(number: &Cell<u64>) -> [OrOnDrop<'_, u32>; 5] {
    [
        OrOnDrop::new(3, number, 1),
        OrOnDrop::new(5, number, 1 << 1),
        OrOnDrop::new(8, number, 1 << 2),
        OrOnDrop::new(13, number, 1 << 3),
        OrOnDrop::new(21, number, 1 << 4),
    ]
}

#[test]
fn array_into_fields() {
    let number = Cell::new(0u64);
    {
        number.set(0);
        let arr = make_array_a(&number);
        assert_eq!(number.get(), 0);
        let (f0, f2) = arr.into_fields(fp!(0, 2));

        // ensuring that the non-moved out fields were dropped
        assert_eq!(number.get(), 0b11010);

        assert_eq!(f0.into_inner_and_bits(), (3, 1));
        assert_eq!(f2.into_inner_and_bits(), (8, 1 << 2));

        assert_eq!(number.get(), 0b11111);
    }
    {
        number.set(0);
        let arr = make_array_a(&number);
        assert_eq!(number.get(), 0);
        let (f1, f3, f4) = arr.into_fields(fp!(1, 3, 4));

        // ensuring that the non-moved out fields were dropped
        assert_eq!(number.get(), 0b00101);

        assert_eq!(f1.into_inner_and_bits(), (5, 1 << 1));
        assert_eq!(number.get(), 0b00111);

        assert_eq!(f3.into_inner_and_bits(), (13, 1 << 3));
        assert_eq!(number.get(), 0b01111);

        assert_eq!(f4.into_inner_and_bits(), (21, 1 << 4));
        assert_eq!(number.get(), 0b11111);
    }
}

macro_rules! generic_tuple_struct_enum_test {
    (
        constructor=$constructor_ok:ident,
        type=$type:ty,
        absent=$absent:expr,
    ) => {{
        let number = Cell::new(0u64);

        let value: $type = $constructor_ok(make_array_a(&number));
        let (f0, f2) = value.into_fields(fp!(::$constructor_ok.0=>0, 2)).unwrap();

        // ensuring that the non-moved out fields were dropped
        assert_eq!(number.get(), 0b11010);

        assert_eq!(f0.into_inner_and_bits(), (3, 1));
        assert_eq!(number.get(), 0b11011);

        assert_eq!(f2.into_inner_and_bits(), (8, 1<<2));
        assert_eq!(number.get(), 0b11111);

        let other: $type =$absent;
        assert!(other.into_fields(fp!(::$constructor_ok.0=>0, 2)).is_none());
    }};
}

#[test]
fn result_into_fields() {
    generic_tuple_struct_enum_test! {
        constructor=Ok,
        type=Result<[OrOnDrop<'_,u32>;5],()>,
        absent=Err(()),
    }
}

#[test]
fn option_into_fields() {
    generic_tuple_struct_enum_test! {
        constructor=Some,
        type=Option<[OrOnDrop<'_,u32>;5]>,
        absent=None,
    }
}

#[test]
fn range_into_fields() {
    let number = Cell::new(0u64);
    let this = OrOnDrop::new(3, &number, 1)..OrOnDrop::new(5, &number, 1 << 1);

    let (start, end) = this.into_fields(fp!(start, end));

    assert_eq!(start.into_inner_and_bits(), (3, 1));
    assert_eq!(number.get(), 0b01);

    assert_eq!(end.into_inner_and_bits(), (5, 1 << 1));
    assert_eq!(number.get(), 0b11);
}

macro_rules! generic_struct_test {
    (
        constructor=$constructor:expr,
        enum_constructor=$enum_constructor:expr,
    ) => {
        {
            let number = Cell::new(0u64);

            let value = $constructor(make_array_a(&number));
            let field = value.into_field(fp!(0));
            assert_eq!(number.replace(0), 0b11110 );
            assert_eq!(field.into_inner_and_bits(), (3, 1));
            assert_eq!(number.get(), 0b00001);
        }
        {
            let number = Cell::new(0u64);

            let value = $enum_constructor(make_array_a(&number));
            let field = value.into_field(fp!(::Some.1)).unwrap();

            assert_eq!(number.replace(0), 0b11101 );
            assert_eq!(field.into_inner_and_bits(), (5, 1<<1));
            assert_eq!(number.get(), 0b00010);
        }
        {
            let number = Cell::new(0u64);

            let value = $constructor(make_array_a(&number));
            let (f0, f2) = value.into_fields(fp!(0, 2));

            // ensuring that the non-moved out fields were dropped
            assert_eq!(number.replace(0), 0b11010 );

            assert_eq!(f0.into_inner_and_bits(), (3, 1));
            assert_eq!(number.get(), 0b00001);

            assert_eq!(f2.into_inner_and_bits(), (8, 1<<2));
            assert_eq!(number.get(), 0b00101);
        }
        {
            let number = Cell::new(0u64);

            let value = $enum_constructor(make_array_a(&number));
            let (f1, f3, f4) = value.into_fields(fp!(::Some=>1, 3, 4)).unwrap();

            // ensuring that the non-moved out fields were dropped
            assert_eq!(number.replace(0), 0b00101);

            assert_eq!(f1.into_inner_and_bits(), (5, 1<<1));
            assert_eq!(number.get(), 0b00010);

            assert_eq!(f3.into_inner_and_bits(), (13, 1<<3));
            assert_eq!(number.get(), 0b01010);

            assert_eq!(f4.into_inner_and_bits(), (21, 1<<4));
            assert_eq!(number.get(), 0b11010);

        }

    };
}

#[test]
fn box_into_fields() {
    generic_struct_test! {
        constructor=Box::new,
        enum_constructor=(|x|Box::new(NewtypeEnum::Some(x))),
    }

    fn dyn_constructor<'a>(
        this: [OrOnDrop<'a, u32>; 5],
    ) -> Box<dyn Array5<OrOnDrop<'a, u32>> + 'a> {
        Box::new(this)
    }

    fn dyn_variant_constructor<'a>(
        this: [OrOnDrop<'a, u32>; 5],
    ) -> Box<dyn Array5Variant<OrOnDrop<'a, u32>, TS!(Some)> + 'a> {
        Box::new(NewtypeEnum::Some(this))
    }

    generic_struct_test! {
        constructor=dyn_constructor,
        enum_constructor=dyn_variant_constructor,
    }
}

#[derive(Structural)]
#[struc(no_trait)]
enum NewtypeEnum<T> {
    #[struc(newtype)]
    Some(T),
}
