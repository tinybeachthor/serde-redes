use quickcheck::quickcheck;
use serde::Serialize;

use serde_ast::{to_ast, Ast};

fn ast_to_json(ast: &Ast) -> String {
    let mut buf = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut buf);
    ast.serialize(&mut serializer).expect("serialize ast");
    String::from_utf8(buf).expect("valid UTF8")
}

quickcheck! {
    fn vec_usize(example: Vec<usize>) -> bool {
        let expected = serde_json::to_string(&example).expect("serde_json::to_string");
        let ast = to_ast(&example).expect("serialize to_ast");
        ast_to_json(&ast) == expected
    }
    fn string(example: String) -> bool {
        let expected = serde_json::to_string(&example).expect("serde_json::to_string");
        let ast = to_ast(&example).expect("serialize to_ast");
        ast_to_json(&ast) == expected
    }
    fn pair(example: (String, char)) -> bool {
        let expected = serde_json::to_string(&example).expect("serde_json::to_string");
        let ast = to_ast(&example).expect("serialize to_ast");
        ast_to_json(&ast) == expected
    }
    fn integers(example: (u8, u16, u32, u64, i8, i16, i32, i64)) -> bool {
        let expected = serde_json::to_string(&example).expect("serde_json::to_string");
        let ast = to_ast(&example).expect("serialize to_ast");
        ast_to_json(&ast) == expected
    }
    fn floats(example: (f32, f64)) -> bool {
        let expected = serde_json::to_string(&example).expect("serde_json::to_string");
        let ast = to_ast(&example).expect("serialize to_ast");
        ast_to_json(&ast) == expected
    }
    fn hashmap(example: std::collections::HashMap<String, String>) -> bool {
        let expected = serde_json::to_string(&example).expect("serde_json::to_string");
        let ast = to_ast(&example).expect("serialize to_ast");
        ast_to_json(&ast) == expected
    }
}
