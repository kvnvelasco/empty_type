use empty_type::EmptyType;
use empty_type_derive::EmptyType;
use serde::Deserialize;

#[derive(EmptyType, Deserialize)]
#[empty(deserialize)]
struct TestStruct {
    value: bool,
    valuer: String,
    missing: Vec<String>,
}

#[test]
fn empty_type_can_be_instantiated() {
    let empty = TestStruct::new_empty();
    assert_eq!(empty.value, false);
}

#[test]
fn empty_type_can_be_deserialized() {
    let json = r#"
        {
            "value": true,
            "valuer": "more value"
        }
    "#;

    let value: <TestStruct as EmptyType>::Container = serde_json::from_str(json).unwrap();
    assert!(value.value);
    assert!(value.valuer.is_some());

    assert!(value.missing.is_none());
}
