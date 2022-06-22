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

use crate::{Container, EmptyType};
use std::error::Error;
use std::ops::{Deref, DerefMut};

pub struct Empty<F>(pub F::Container, pub std::marker::PhantomData<F>)
where
    F: EmptyType;

impl<F> Empty<F>
where
    F: EmptyType,
{
    pub fn resolve(mut self) -> <Self as Container>::Value {
        match self.try_open() {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn try_resolve(mut self) -> Result<<Self as Container>::Value, Box<dyn std::error::Error>> {
        self.try_open()
    }
}

#[cfg(feature = "serde")]
impl<'de, F> serde::Deserialize<'de> for Empty<F>
where
    F: EmptyType,
    F::Container: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: F::Container = <F::Container as serde::Deserialize>::deserialize(deserializer)?;

        Ok(Self(value, Default::default()))
    }
}

impl<F> Deref for Empty<F>
where
    F: EmptyType,
{
    type Target = F::Container;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> DerefMut for Empty<F>
where
    F: EmptyType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<F> Container for Empty<F>
where
    F: EmptyType,
{
    type Value = <<F as EmptyType>::Container as Container>::Value;

    fn try_open(&mut self) -> Result<Self::Value, Box<dyn Error>> {
        self.0.try_open()
    }
}
