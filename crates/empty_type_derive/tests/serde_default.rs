/*
 * Copyright [2022] [Kevin Velasco]
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use empty_type::{deserialize_empty, EmptyType};
use empty_type_derive::EmptyType;
use serde::Deserialize;

#[derive(EmptyType, Deserialize)]
#[empty(deserialize, fail_safe)]
struct TestStruct {
    value: Inner,
}

#[derive(EmptyType, Deserialize)]
#[empty(deserialize)]
struct FieldTest {
    #[empty(fail_safe)]
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
fn field_test_serializes_to_default() {
    let json = r#" {}  "#;

    let mut de = serde_json::Deserializer::from_str(json);
    let value = deserialize_empty::<FieldTest, _>(&mut de).unwrap();

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

#[test]
fn invalid_de_still_produces_value_field() {
    let json = r#" {
        "value": "string"
    }  "#;

    let mut de = serde_json::Deserializer::from_str(json);
    let value = deserialize_empty::<FieldTest, _>(&mut de).unwrap();

    // This resolved to none but still worked
    assert!(value.value.is_none());

    let value = value.resolve();
    assert_eq!(value.value, Inner::default());
}
