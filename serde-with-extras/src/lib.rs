mod convert;

use std::fmt::Debug;

use indexmap::IndexMap;
use serde::{ser::SerializeTupleStruct, Serialize};

// re-export indexmap macro to construct attributes
pub use indexmap::indexmap as extras;

pub const SERDE_EXTRAS_WELLKNOWN_NAME: &str = "__SERDE_EXTRAS__EXTRAS";

pub const EXTRAS_COMMENT_BEFORE: &str = "comment";
pub const EXTRAS_COMMENT_AFTER: &str = "comment-after";

pub type ExtrasAttributes = IndexMap<String, String>;

pub struct Extras<T> {
    inner: T,
    extras: ExtrasAttributes,
}
impl<T> Extras<T> {
    /// Construct new [Extras] from `inner` value and `extras`.
    pub fn new(inner: T, extras: ExtrasAttributes) -> Self {
        Self { inner, extras }
    }
}
impl<T: Serialize> Serialize for Extras<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let extras_serialized =
            serde_json::to_string(&self.extras).map_err(serde::ser::Error::custom)?;

        let mut ts = serializer.serialize_tuple_struct(SERDE_EXTRAS_WELLKNOWN_NAME, 2)?;
        ts.serialize_field(&self.inner)?;
        ts.serialize_field(&extras_serialized)?;
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to serialize to Ast: {0}")]
    ToAst(serde_ast::ser::Error),
    #[error("Failed to convert Ast to Extras: {0}")]
    LiftExtras(convert::Error),
}

/// Serialize a value into [XAst<ExtrasAttributes>].
pub fn to_with_extras<T>(value: &T) -> Result<serde_ast::XAst<ExtrasAttributes>, Error>
where
    T: Serialize + ?Sized,
{
    let ast = serde_ast::to_ast(value).map_err(Error::ToAst)?;
    let with_extras = convert::lift(ast).map_err(Error::LiftExtras)?;
    Ok(with_extras)
}
