pub mod ser;
pub mod op;

use serde::Serialize;

pub use op::*;
pub use ser::Serializer;

pub fn to_ast<T>(value: &T) -> Result<Vec<Op>, ser::Error>
where
    T: Serialize + ?Sized,
{
    let mut ops = Vec::new();
    let serializer = Serializer::new(&mut ops);
    value.serialize(serializer)?;
    Ok(ops)
}
