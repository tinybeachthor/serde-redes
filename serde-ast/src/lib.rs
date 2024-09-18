pub mod ast;
pub mod ser;

use serde::Serialize;

pub use ast::Ast;
pub use ser::Serializer;

pub fn to_ast<T>(value: &T) -> Result<Ast, ser::Error>
where
    T: Serialize + ?Sized,
{
    let serializer = Serializer::new();
    value.serialize(serializer)
}
