use empty_type_derive::EmptyType;
use empty_type_traits as empty_type;
use empty_type_traits::EmptyType;

#[derive(EmptyType, Default)]
#[empty(default)]
struct TestStruct {
    value: Nested,
}

#[derive(Default)]
struct Nested {
    value: bool,
}

#[test]
fn empty_type_can_be_instantiated() {
    let empty = TestStruct::new_empty();
    assert!(empty.value.is_none());
}

#[test]
fn unwrapping_produces_default_value() {
    let empty = TestStruct::new_empty();
    let full = empty.resolve();

    assert_eq!(full.value.value, false);
}
