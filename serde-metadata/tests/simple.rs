use indexmap::indexmap;
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
            a: Metadata::from(indexmap! {
                "comment".to_string() => "field: a".to_string(),
            }),
            b: Metadata::from(indexmap! {
                "comment".to_string() => "field: b".to_string(),
            }),
        }
    }
}

impl SerdeMetadata for Simple {
    type METADATA = SimpleMetadata;
}

#[test]
fn get_metadata() {
    let simple = Simple {
        a: "hello".to_string(),
        b: 42,
    };
    let metadata = simple.get_metadata();

    insta::assert_snapshot!(serde_json::to_string(&metadata).unwrap());
}
