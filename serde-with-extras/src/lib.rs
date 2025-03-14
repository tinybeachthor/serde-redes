use std::fmt::Debug;

use indexmap::IndexMap;
use serde::{ser::SerializeTupleStruct, Serialize};

// re-export indexmap macro to construct attributes
pub use indexmap::indexmap as extras;

pub mod ser;

pub const SERDE_EXTRAS_WELLKNOWN_NAME: &str = "__SERDE_EXTRAS__EXTRAS";

pub const EXTRAS_COMMENT_BEFORE: &str = "comment";
pub const EXTRAS_COMMENT_AFTER: &str = "comment-after";

pub struct Extras<T> {
    inner: T,
    extras: IndexMap<&'static str, String>,
}

impl<T> Extras<T> {
    /// Construct new [Extras] from `inner` value and `extras`.
    pub fn new(inner: T, extras: IndexMap<&'static str, String>) -> Self {
        Self { inner, extras }
    }
}

impl<T: Serialize> Serialize for Extras<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ts = serializer.serialize_tuple_struct(SERDE_EXTRAS_WELLKNOWN_NAME, 2)?;
        ts.serialize_field(&self.inner)?;
        ts.serialize_field(&self.extras)?;
        ts.end()
    }
}

impl<T: Debug> Debug for Extras<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extras")
            .field("inner", &self.inner)
            .field("extras", &self.extras)
            .finish()
    }
}

impl<T: Clone> Clone for Extras<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            extras: self.extras.clone(),
        }
    }
}
