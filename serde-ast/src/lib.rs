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
//! This allows to see the serializattion calls made, inspect them, traverse or edit, or serialize with a [serde::Serializer].
//!
//! Serializing the [Ast] is equivalent to directly serializing the original value.
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
