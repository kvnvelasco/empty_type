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
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::de::{
    self,
    value::{self, MapAccessDeserializer, SeqAccessDeserializer},
    Deserialize, Deserializer, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};

pub struct Fallible<T>(T);

impl<T> Deref for Fallible<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Fallible<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Fallible<T>
where
    T: Container,
    T: Default,
    T: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor: DelegateVisitor<T> = DelegateVisitor {
            original_type: Default::default(),
        };

        let value = deserializer.deserialize_any(visitor);

        Ok(Self(value.unwrap_or_default()))
    }
}

// The delegate visitor is will delegate any value it extracts out
// of the deserializer into the passed type. When it fails, it will
// return the default value T
struct DelegateVisitor<T> {
    original_type: PhantomData<T>,
}

#[cfg(feature = "serde")]
impl<'de, T> Visitor<'de> for DelegateVisitor<T>
where
    T: Deserialize<'de> + Default,
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Delegate visitor encountered an unexpected error"
        )
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let variant: Result<T, _> = data.variant().map(|(v, _)| v);
        Ok(variant.unwrap_or_default())
    }
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        Ok(T::deserialize(des).unwrap_or_default())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(T::default())
    }
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer);
        Ok(value.unwrap_or_default())
    }
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let des = SeqAccessDeserializer::new(seq);
        Ok(T::deserialize(des).unwrap_or_default())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer);
        Ok(value.unwrap_or_default())
    }
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let deserializer = IntoDeserializer::<'_, value::Error>::into_deserializer(v);
        Ok(T::deserialize(deserializer).unwrap_or_default())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(T::default())
    }
}

impl<T> Default for Fallible<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}

impl<V> Container for Fallible<V>
where
    V: Container,
    V::Value: Default,
{
    type Value = V::Value;

    fn try_open(&mut self) -> Result<Self::Value, Box<dyn Error>> {
        Ok(self.0.open_or_default())
    }
}
