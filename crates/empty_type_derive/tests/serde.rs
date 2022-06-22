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
