//! This tests that std types implement the structural aliases for them.

use structural::structural_aliases as sa;

use structural::assert_implements;

use std::ops;

#[test]
fn std_types() {
    assert_implements!(for[T] Option<T>, sa::Option_SI<T>);
    assert_implements!(for[T] Option<T>, sa::OptionMove_ESI<T>);
    assert_implements!(for[T,E] Result<T,E>, sa::Result_SI<T,E>);
    assert_implements!(for[T,E] Result<T,E>, sa::ResultMove_ESI<T,E>);

    assert_implements!(for[T] ops::Range<T>, sa::Range_SI<T>);
    assert_implements!(for[T] ops::Range<T>, sa::RangeRef_SI<T>);
    assert_implements!(for[T] ops::RangeInclusive<T>, sa::RangeRef_SI<T>);
    assert_implements!(for[T] ops::RangeFrom<T>, sa::RangeFrom_SI<T>);
    assert_implements!(for[T] ops::RangeTo<T>, sa::RangeTo_SI<T>);
    assert_implements!(for[T] ops::RangeToInclusive<T>, sa::RangeTo_SI<T>);
}
