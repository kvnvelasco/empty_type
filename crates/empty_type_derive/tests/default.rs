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
