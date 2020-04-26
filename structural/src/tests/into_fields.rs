use crate::{test_utils::OrOnDrop, StructuralExt};

use std_::cell::Cell;

mod drop_order;

#[cfg(feature = "alloc")]
#[test]
fn tuple_into_fields() {
    use crate::alloc::{
        boxed::Box,
        string::{String, ToString},
        vec,
        vec::Vec,
    };

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
