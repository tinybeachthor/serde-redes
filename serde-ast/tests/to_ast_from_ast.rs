use serde::{Deserialize, Serialize};

use serde_ast::{to_ast, Ast};

#[derive(Serialize, Deserialize)]
struct Example {
    hello: String,
    nested: Nested,
    array: [usize; 3],
    vec: Vec<String>,
    vec2: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
struct Nested {
    a: u32,
    b: usize,
}

fn ast_to_json(ast: &Ast) -> String {
    let mut buf = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut buf);
    ast.serialize(&mut serializer).expect("serialize ast");
    String::from_utf8(buf).expect("valid UTF8")
}

#[test]
fn json_u32() {
    let example = 10u32;
    let expected = serde_json::to_string(&example).expect("serde_json::to_string");

    let ast = to_ast(&example).expect("serialize to_ast");
    assert_eq!(ast_to_json(&ast), expected);
}

#[test]
fn json_struct_nested() {
    let example = Example {
        hello: "World".to_string(),
        nested: Nested { a: 100, b: 42 },
        array: [1, 2, 3],
        vec: vec![String::from("hello"), String::from("world")],
        vec2: vec![],
    };
    let expected = serde_json::to_string(&example).expect("serde_json::to_string");

    let ast = to_ast(&example).expect("serialize to_ast");
    assert_eq!(ast_to_json(&ast), expected);
}
