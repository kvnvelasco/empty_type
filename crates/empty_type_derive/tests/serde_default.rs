use empty_type::{deserialize_empty, EmptyType};
use empty_type_derive::EmptyType;
use serde::Deserialize;

#[derive(EmptyType, Deserialize)]
#[empty(deserialize, default)]
struct TestStruct {
    value: Inner,
}

#[derive(Deserialize, Default, Debug, PartialEq)]
struct Inner(Vec<bool>);

#[test]
fn empty_type_can_be_instantiated() {
    let empty = TestStruct::new_empty();
    assert!(empty.value.is_none());
}

#[test]
fn empty_type_deserializes_to_default() {
    let json = r#" {}  "#;

    let mut de = serde_json::Deserializer::from_str(json);
    let value = deserialize_empty::<TestStruct, _>(&mut de).unwrap();

    // This resolved to none but still worked
    assert!(value.value.is_none());

    let value = value.resolve();
    assert_eq!(value.value, Inner::default());
}

#[test]
fn invalid_deserialization_still_produces_values() {
    let json = r#" {
        "value": "string"
    }  "#;

    let mut de = serde_json::Deserializer::from_str(json);
    let value = deserialize_empty::<TestStruct, _>(&mut de).unwrap();

    // This resolved to none but still worked
    assert!(value.value.is_none());

    let value = value.resolve();
    assert_eq!(value.value, Inner::default());
}
