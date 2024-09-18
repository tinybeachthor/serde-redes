# serde-ast

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

Serializing the [Ast] is equivalent to directly serializing the original value.

```rust
// serialize the ast
let output = serde_json::to_string(&ast).expect("serde_json::to_string");
// serialize the value directly
let direct = serde_json::to_string(&example).expect("serde_json::to_string");
// the result is the same
assert_eq!(output, direct);
```
