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
