#![cfg_attr(docs_rs, feature(doc_cfg))]

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

pub trait EmptyType
where
    Self: Sized,
{
    type Container: Container + Default;

    fn new_container() -> Self::Container {
        Default::default()
    }
    fn new_empty() -> Empty<Self> {
        Empty(Self::new_container(), Default::default())
    }
}

/// Used to deserialize a given type into its coreesponding EmptyType
///
/// ```text
/// #[derive(EmptyType)
/// #[empty(deserialize)]
/// struct Data(usize);
///
/// let value = deserialize_empty::<Data, _>(&mut de);
/// ```
#[cfg(feature = "serde")]
#[cfg_attr(docs_rs, doc(cfg(feature = "serde")))]
pub fn deserialize_empty<'de, T, D>(de: D) -> Result<Empty<T>, D::Error>
where
    T: EmptyType,
    D: serde::Deserializer<'de>,
    T::Container: serde::Deserialize<'de>,
{
    use serde::Deserialize;
    let value = T::Container::deserialize(de)?;

    Ok(Empty(value, Default::default()))
}
