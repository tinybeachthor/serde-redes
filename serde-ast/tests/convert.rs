use std::collections::HashMap;

use serde::Serialize;
use serde_ast::{into_extended, to_ast, XAst};

#[derive(Serialize)]
struct Example {
    string: Option<String>,
    sequence: Vec<u8>,
}

#[test]
fn convert_to_extended() {
    let example = Example {
        string: Some("string".to_string()),
        sequence: vec![0, 1, 2, 3],
    };
    let final_ast = to_ast(&example).unwrap();
    let _extended: XAst<HashMap<String, u8>> = into_extended(final_ast);
}
