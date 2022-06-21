use std::ops::Deref;
use std::ops::DerefMut;

pub struct Empty<E, F>(pub E, pub std::marker::PhantomData<F>)
where
    E: Unwrap;

impl<E, F> Deref for Empty<E, F>
where
    E: Unwrap,
{
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E, F> DerefMut for Empty<E, F>
where
    E: Unwrap,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E, F> Unwrap for Empty<E, F>
where
    E: Unwrap,
{
    type Value = E::Value;
    fn unwrap(self) -> Self::Value {
        self.0.unwrap()
    }
    fn unwrap_with_hint(self, hint: &'static str) -> Self::Value {
        self.0.unwrap_with_hint(hint)
    }
    fn unwrap_or_default(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Default,
    {
        self.0.unwrap_or_default()
    }
}

pub struct Optional<T>(T);

impl<T> Default for Optional<Option<T>> {
    fn default() -> Self {
        Self(None)
    }
}

/// Struct may fail to deserialize. If so, will revert to the default value
pub struct Fallible<T>(T);

impl<T> Unwrap for Fallible<T>
where
    T: Unwrap,
    <T as Unwrap>::Value: Default,
{
    type Value = T::Value;

    fn unwrap(self) -> Self::Value {
        self.0.unwrap_or_default()
    }
    fn unwrap_with_hint(self, _hint: &'static str) -> Self::Value
    where
        Self: Sized,
    {
        self.unwrap()
    }
    fn unwrap_or_default(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Default,
    {
        self.unwrap()
    }
}

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
    T: Unwrap,
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

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Optional<Option<T>>
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

impl<T> Deref for Optional<Option<T>> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Optional<Option<T>> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Unwrap for Optional<Option<T>> {
    type Value = Option<T>;

    fn unwrap(self) -> Self::Value {
        self.0
    }
    fn unwrap_with_hint(self, _hint: &'static str) -> Self::Value
    where
        Self: Sized,
    {
        self.unwrap()
    }
    fn unwrap_or_default(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Default,
    {
        self.0
    }
}

pub trait Unwrap {
    type Value;

    fn unwrap(self) -> Self::Value;

    fn unwrap_with_hint(self, _hint: &'static str) -> Self::Value
    where
        Self: Sized;

    fn unwrap_or_default(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Default,
    {
        unimplemented!()
    }
}

impl Unwrap for bool {
    type Value = bool;

    fn unwrap(self) -> bool {
        self
    }

    fn unwrap_with_hint(self, _hint: &'static str) -> bool {
        self
    }
    fn unwrap_or_default(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Default,
    {
        self
    }
}

impl<T> Unwrap for Option<T> {
    type Value = T;

    fn unwrap(self) -> T {
        self.unwrap()
    }

    fn unwrap_with_hint(self, hint: &'static str) -> T {
        self.expect(hint)
    }

    fn unwrap_or_default(self) -> Self::Value
    where
        Self: Sized,
        Self::Value: Default,
    {
        self.unwrap_or_default()
    }
}

#[cfg(not(feature = "serde"))]
pub trait EmptyType<E, F> {
    type Container: Unwrap;
    fn new_empty() -> Empty<Self::Container, F>;
}

#[cfg(feature = "serde")]
pub trait EmptyType<'de, F> {
    type Container: Unwrap;

    fn new_empty() -> Empty<Self::Container, F>;

    fn deserialize_empty<D>(_deserializer: D) -> Result<Empty<Self::Container, F>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
