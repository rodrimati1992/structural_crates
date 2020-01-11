use super::*;
use crate::GetFieldExt;

macro_rules! slice_test {
    ( ref $this:expr ) => {{
        let mut this = $this;
        assert_eq!(this.field_(fp!(0)), Some(&3));
        assert_eq!(this.field_(fp!(1)), Some(&5));
        assert_eq!(this.field_(fp!(2)), Some(&8));
        assert_eq!(this.field_(fp!(3)), Some(&13));
        assert_eq!(this.field_(fp!(4)), Some(&21));
        assert_eq!(this.field_(fp!(5)), Some(&34));
        assert_eq!(this.field_(fp!(6)), Some(&55));
        assert_eq!(this.field_(fp!(7)), Some(&89));
        assert_eq!(this.field_(fp!(8)), Some(&144));
        assert_eq!(this.field_(fp!(9)), None);
    }};
    ( mut $this:expr) => {{
        let mut this = $this;

        assert_eq!(this.field_mut(fp!(0)), Some(&mut 3));
        assert_eq!(this.field_mut(fp!(1)), Some(&mut 5));
        assert_eq!(this.field_mut(fp!(2)), Some(&mut 8));
        assert_eq!(this.field_mut(fp!(3)), Some(&mut 13));
        assert_eq!(this.field_mut(fp!(4)), Some(&mut 21));
        assert_eq!(this.field_mut(fp!(5)), Some(&mut 34));
        assert_eq!(this.field_mut(fp!(6)), Some(&mut 55));
        assert_eq!(this.field_mut(fp!(7)), Some(&mut 89));
        assert_eq!(this.field_mut(fp!(8)), Some(&mut 144));
        assert_eq!(this.field_(fp!(9)), None);

        slice_test! {ref this}
    }};
}

#[test]
fn basic_core_tests() {
    slice_test! {ref &[3,5,8,13,21,34,55,89,144][..] }
    slice_test! {mut &mut [3,5,8,13,21,34,55,89,144][..] }
}

#[cfg(feature = "alloc")]
#[test]
fn basic_alloc_tests() {
    use crate::alloc::{boxed::Box, rc::Rc, sync::Arc};
    slice_test! {ref Rc::from([3,5,8,13,21,34,55,89,144]) as Rc<[_]> }
    slice_test! {ref Arc::from([3,5,8,13,21,34,55,89,144]) as Arc<[_]> }
    slice_test! {mut Box::from([3,5,8,13,21,34,55,89,144]) as Box<[_]> }
}

fn large_indices() {
    let mut array = [0u16; 1 << 10];
    for i in 0..array.len() {
        array[i] = (i * 4) as u16;
    }
    let array = &array[..];

    assert_eq!(array.field_(fp!(0)), Some(&0));
    assert_eq!(array.field_(fp!(1)), Some(&4));
    assert_eq!(array.field_(fp!(9)), Some(&8));
    assert_eq!(array.field_(fp!(10)), Some(&40));
    assert_eq!(array.field_(fp!(19)), Some(&36));
    assert_eq!(array.field_(fp!(99)), Some(&396));
    assert_eq!(array.field_(fp!(100)), Some(&400));
    assert_eq!(array.field_(fp!(199)), Some(&796));
    assert_eq!(array.field_(fp!(999)), Some(&3996));
    assert_eq!(array.field_(fp!(1000)), Some(&4000));
    assert_eq!(array.field_(fp!(1023)), Some(&4092));
    assert_eq!(array.field_(fp!(1024)), None);
}
