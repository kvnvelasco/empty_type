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

use crate::Container;
use std::error::Error;
use std::ops::{Deref, DerefMut};

pub struct Optional<T>(Option<T>);

impl<T> Default for Optional<T> {
    fn default() -> Self {
        Self(None)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Optional<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <Option<T> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

impl<T> Deref for Optional<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Optional<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V> Container for Optional<V> {
    type Value = Option<V>;

    /// Optionals will always return an option regardless of the underlying value
    fn try_open(&mut self) -> Result<Self::Value, Box<dyn Error>> {
        Ok(self.0.try_open().ok())
    }
}
