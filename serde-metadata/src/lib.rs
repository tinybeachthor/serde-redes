use std::ops::Deref;

use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Metadata {
    items: IndexMap<String, String>,
}
impl Deref for Metadata {
    type Target = IndexMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

pub trait SerdeMetadata: Serialize {
    type METADATA: Serialize + Default;
}
