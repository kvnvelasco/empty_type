use empty_type::EmptyType;
use empty_type_derive::EmptyType;

#[derive(EmptyType)]
struct TestStruct {
    value: Option<&'static str>,
}

#[derive(EmptyType)]
struct Data(String);

#[test]
fn empty_type_can_be_instantiated() {
    let empty = TestStruct::new_empty();
    assert!(empty.value.is_none());
}

#[test]
fn empty_type_can_be_unwrapped() {
    use std::ops::DerefMut;
    let mut empty = TestStruct::new_empty();
    let ptr = empty.value.deref_mut();
    *ptr = Some("str");

    let unwrapped = empty.resolve();
    assert!(unwrapped.value.is_some());
}
