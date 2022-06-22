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

#![cfg_attr(docs_rs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
#[cfg_attr(docs_rs, doc(cfg(feature = "derive")))]
mod proc_macro {
    pub use empty_type_derive::EmptyType;
}

#[cfg(feature = "derive")]
pub use proc_macro::EmptyType;

pub use empty_type_traits::{Container, Empty, EmptyType, Fallible, Optional};

#[cfg(feature = "serde")]
#[cfg_attr(docs_rs, doc(cfg(feature = "serde")))]
pub use empty_type_traits::deserialize_empty;
