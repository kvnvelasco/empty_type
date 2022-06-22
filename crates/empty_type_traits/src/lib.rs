mod empty;
mod fallible;

mod optional;

pub use empty::*;
pub use fallible::*;
pub use optional::*;

use std::error::Error;

pub trait Container {
    type Value;

    fn try_open(&mut self) -> Result<Self::Value, Box<dyn std::error::Error>>;

    fn open(&mut self) -> Self::Value {
        self.try_open().unwrap()
    }

    fn open_or_default(&mut self) -> Self::Value
    where
        Self::Value: Default,
    {
        self.try_open().unwrap_or_default()
    }

    fn try_open_with_meta(
        &mut self,
        field_name: &'static str,
    ) -> Result<Self::Value, Box<dyn std::error::Error>> {
        self.try_open().map_err(|_| {
            format!(
                "Failed to resolve field {}. Opened to `None` value",
                field_name
            )
            .into()
        })
    }

    fn open_with_meta(&mut self, field_name: &'static str) -> Self::Value {
        self.try_open_with_meta(field_name)
            .expect(concat!(stringify!(field_name), "Failed to resolve"))
    }
}

impl Container for bool {
    type Value = Self;

    fn try_open(&mut self) -> Result<Self::Value, Box<dyn Error>> {
        Ok(*self)
    }
}

impl<V> Container for Option<V> {
    type Value = V;

    fn try_open(&mut self) -> Result<Self::Value, Box<dyn Error>> {
        let value = std::mem::take(self);
        value.ok_or_else(|| "Option opened to `None value`".into())
    }
}

#[cfg(not(feature = "serde"))]
pub trait EmptyType<F> {
    type Container: Container;
    fn new_empty() -> Empty<Self::Container, F>;
}

#[cfg(feature = "serde")]
pub trait EmptyType<'de, F> {
    type Container: Container;

    fn new_empty() -> Empty<Self::Container, F>;

    fn deserialize_empty<D>(_deserializer: D) -> Result<Empty<Self::Container, F>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
