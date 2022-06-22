use crate::Container;
use std::error::Error;
use std::ops::{Deref, DerefMut};

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
        D: serde::Deserializer<'de>,
    {
        let value: Result<T, _> = <T as serde::Deserialize>::deserialize(deserializer);

        Ok(Self(value.unwrap_or_else(|_| <T as Default>::default())))
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
