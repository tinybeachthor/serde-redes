use std::ops::Deref;

use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Metadata {
    items: IndexMap<String, String>,
}
impl Metadata {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
        }
    }
}
impl Deref for Metadata {
    type Target = IndexMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl From<IndexMap<String, String>> for Metadata {
    fn from(items: IndexMap<String, String>) -> Self {
        Self { items }
    }
}

pub trait SerdeMetadata: Serialize {
    type METADATA: Serialize + Default;

    fn get_metadata(&self) -> Self::METADATA {
        Self::METADATA::default()
    }
}
