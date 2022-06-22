use empty_type::{deserialize_empty, EmptyType};
use empty_type_derive::EmptyType;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(EmptyType, Deserialize)]
#[empty(deserialize)]
struct TestStruct<'a> {
    #[serde(borrow)]
    value: Inner<'a>,
}

/// https://github.com/serde-rs/serde/issues/1852
#[derive(Deserialize)]
struct Inner<'a>(#[serde(borrow)] Cow<'a, str>);

#[test]
fn empty_type_can_be_instantiated() {
    let empty = TestStruct::new_empty();
    assert!(empty.value.is_none());
}

#[test]
fn empty_type_can_be_deserialized() {
    let json = r#"
        {
            "value": "true"
        }
    "#;

    let mut de = serde_json::Deserializer::from_str(json);
    let value = deserialize_empty::<TestStruct, _>(&mut de).unwrap();

    assert!(value.value.is_some());

    let value = value.resolve();

    assert!(
        matches!(&value.value, &Inner(Cow::Borrowed(_))),
        "{:?}",
        &value.value.0
    );
}
