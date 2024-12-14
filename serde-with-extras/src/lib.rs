use std::fmt::Debug;

use indexmap::IndexMap;
use serde::Serialize;

// re-export indexmap macro to construct attributes
pub use indexmap::indexmap as extras;

pub const EXTRAS_COMMENT_BEFORE: &str = "comment";
pub const EXTRAS_COMMENT_AFTER: &str = "comment-after";

#[derive(Debug, Clone, Serialize)]
pub struct Extras<T>
where
    T: Debug + Clone + Serialize,
{
    inner: T,
    extras: IndexMap<&'static str, String>,
}
impl<T> Extras<T>
where
    T: Debug + Clone + Serialize,
{
    /// Construct new [Extras] from `inner` value and `extras`.
    pub fn new(inner: T, extras: IndexMap<&'static str, String>) -> Self {
        Self { inner, extras }
    }
}
