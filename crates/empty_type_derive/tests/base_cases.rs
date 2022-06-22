use empty_type_derive::EmptyType;
use empty_type_traits as empty_type;
use empty_type_traits::EmptyType;
use std::ops::Deref;

#[derive(EmptyType)]
struct TestStruct {
    value: Option<&'static str>,
}

#[test]
fn empty_type_can_be_instantiated() {
    let empty = TestStruct::new_empty();
    assert!(empty.value.is_none());
}

#[test]
fn empty_type_can_be_unwrapped() {
    use std::ops::DerefMut;
    let mut empty = TestStruct::new_empty();
    let mut ptr = empty.value.deref_mut();
    *ptr = Some("str");

    let unwrapped = empty.resolve();
    assert!(unwrapped.value.is_some());
}
