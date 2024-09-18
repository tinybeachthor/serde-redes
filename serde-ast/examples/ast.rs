use serde::{Deserialize, Serialize};

use serde_ast::to_ast;

#[derive(Serialize, Deserialize)]
struct Example {
    hello: String,
    nested: Nested,
}

#[derive(Serialize, Deserialize)]
struct Nested {
    a: u32,
    b: usize,
}

fn main() {
    let example = Example {
        hello: "World".to_string(),
        nested: Nested { a: 100, b: 42 },
    };

    let ast = to_ast(&example).expect("serialize to_ast");

    println!("{}", ast);
}
