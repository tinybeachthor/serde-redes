use serde::Serialize;
use serde_metadata::{Metadata, SerdeMetadata};

#[derive(Serialize)]
pub struct Simple {
    a: String,
    b: u64,
}

#[derive(Serialize)]
pub struct SimpleMetadata {
    a: Metadata,
    b: Metadata,
}
impl Default for SimpleMetadata {
    fn default() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}

impl SerdeMetadata for Simple {
    type METADATA = SimpleMetadata;
}

#[test]
fn get_metadata() {
}
