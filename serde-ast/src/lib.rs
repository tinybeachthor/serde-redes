pub mod ser;
pub mod ast;

use serde::Serialize;

pub use ast::Ast;
pub use ser::Serializer;

pub fn to_ast<T>(value: &T) -> Result<Vec<Ast>, ser::Error>
where
    T: Serialize + ?Sized,
{
    let mut ops = Vec::new();
    let serializer = Serializer::new(&mut ops);
    value.serialize(serializer)?;
    Ok(ops)
}
