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
