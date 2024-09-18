use serde::{Deserialize, Serialize};

use serde_ast::to_ast;

#[derive(Serialize, Deserialize)]
struct Example {
    hello: String,
}

fn main() {
    let example = Example {
        hello: "World".to_string(),
    };

    let ast = to_ast(&example).expect("serialize to_ast");
    println!("{}", ast);
}
