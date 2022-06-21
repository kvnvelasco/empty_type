use empty_type_derive::EmptyType;
use empty_type_traits as empty_type;
use empty_type_traits::EmptyType;
use empty_type_traits::Unwrap;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(EmptyType, Deserialize)]
#[empty(deserialize, bounds = "'a")]
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
    let value = TestStruct::deserialize_empty(&mut de).unwrap();

    assert!(value.value.is_some());

    let value = value.unwrap();

    assert!(
        matches!(&value.value, &Inner(Cow::Borrowed(_))),
        "{:?}",
        &value.value.0
    );
}
