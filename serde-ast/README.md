# serde-ast

[![Crates.io Version](https://img.shields.io/crates/v/serde-ast)](https://crates.io/crates/serde-ast)
[![docs.rs](https://img.shields.io/docsrs/serde-ast)](https://docs.rs/serde-ast/latest/serde_ast/)

Define an AST representation of `serde` serialization.

```rust
use serde::{Deserialize, Serialize};
use serde_ast::to_ast;

#[derive(Serialize, Deserialize)]
struct Example {
    hello: String,
}

let example = Example { hello: "World".to_string() };
let ast = to_ast(&example).expect("serialize to_ast");
println!("{}", ast);
```
```text
Struct {
    name: "Example",
    len: 1,
    ops: [
        Field {
            key: "hello",
            value: Str(
                "World",
            ),
        },
    ],
}
```

Serializing the `Ast` is equivalent to directly serializing the original value.

```rust
// serialize the ast
let output = serde_json::to_string(&ast).expect("serde_json::to_string");
// serialize the value directly
let direct = serde_json::to_string(&example).expect("serde_json::to_string");
// the result is the same
assert_eq!(output, direct);
```
