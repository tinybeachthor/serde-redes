#![deny(
    rust_2018_compatibility,
    rust_2021_compatibility,
    rust_2024_compatibility,
    future_incompatible,
    nonstandard_style,
    let_underscore,
    keyword_idents,
    unused_variables
)]
#![warn(unused, missing_docs)]

//! Implements an [Ast] representation of [serde] serialization.
//!
//! This allows to see the serialization calls made, inspect them, traverse, edit, or serialize with a [serde::Serializer].
//!
//! ```
//! # use serde::{Deserialize, Serialize};
//! # use serde_ast::to_ast;
//! #[derive(Serialize, Deserialize)]
//! struct Example {
//!     hello: String,
//! }
//! let example = Example { hello: "World".to_string() };
//! let ast = to_ast(&example).expect("serialize to_ast");
//! println!("{}", ast);
//! ```
//! ```text
//! Struct {
//!     name: "Example",
//!     len: 1,
//!     ops: [
//!         Field {
//!             key: "hello",
//!             value: Str(
//!                 "World",
//!             ),
//!         },
//!     ],
//! }
//! ```
//!
//! Serializing the [Ast] is equivalent to directly serializing the original value.
//!
//! ```
//! # use serde::{Deserialize, Serialize};
//! # use serde_ast::to_ast;
//! # #[derive(Serialize, Deserialize)]
//! # struct Example {
//! #     hello: String,
//! # }
//! # let example = Example { hello: "World".to_string() };
//! # let ast = to_ast(&example).expect("serialize to_ast");
//! // serialize the ast
//! let output = serde_json::to_string(&ast).expect("serde_json::to_string");
//! // serialize the value directly
//! let direct = serde_json::to_string(&example).expect("serde_json::to_string");
//! // the result is the same
//! assert_eq!(output, direct);
//! ```

pub mod ast;
pub mod ser;

use serde::Serialize;

pub use ast::Ast;
pub use ser::Serializer;

/// Serialize a value into [Ast].
///
/// Serializing the [Ast] is equivalent to directly serializing the value.
pub fn to_ast<T>(value: &T) -> Result<Ast, ser::Error>
where
    T: Serialize + ?Sized,
{
    let serializer = Serializer::new();
    value.serialize(serializer)
}
