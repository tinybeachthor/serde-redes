use std::fmt::Display;

use crate::{ast, to_ast, Ast};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error")]
    Custom(String),
}
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

pub struct Serializer {}
impl<'ast> Serializer {
    pub fn new() -> Self {
        Self {}
    }
}
impl serde::Serializer for Serializer {
    type Ok = Ast;
    type Error = Error;

    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::Bool(v))
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::I8(v))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::I16(v))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::I32(v))
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::I64(v))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::U8(v))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::U16(v))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::U32(v))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::U64(v))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::F32(v))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::F64(v))
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::Char(v))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::Str(v.to_owned()))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::Bytes(v.to_owned()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::None)
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Ok(Ast::Some(Box::new(to_ast(value)?)))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::Unit)
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::UnitStruct(name))
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Ast::UnitVariant {
            name,
            variant_index,
            variant,
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Ok(Ast::NewtypeStruct {
            name,
            value: Box::new(to_ast(value)?),
        })
    }
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Ok(Ast::NewtypeVariant {
            name,
            variant_index,
            variant,
            value: Box::new(to_ast(value)?),
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq::new(len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple::new(len))
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct::new(name, len))
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant::new(
            name,
            variant_index,
            variant,
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap::new(len))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct::new(name, len))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant::new(
            name,
            variant_index,
            variant,
            len,
        ))
    }
}
pub struct SerializeTuple {
    len: usize,
    inner_ops: Vec<ast::Tuple>,
}
impl SerializeTuple {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeTuple for SerializeTuple {
    type Ok = Ast;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Tuple::Element {
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self { len, inner_ops } = self;

        Ok(Ast::Tuple {
            len,
            ops: inner_ops,
        })
    }
}
pub struct SerializeTupleStruct {
    name: &'static str,
    len: usize,
    inner_ops: Vec<ast::TupleStruct>,
}
impl SerializeTupleStruct {
    pub fn new(name: &'static str, len: usize) -> Self {
        Self {
            name,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = Ast;
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            name,
            len,
            inner_ops,
        } = self;

        Ok(Ast::TupleStruct {
            name,
            len,
            ops: inner_ops,
        })
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::TupleStruct::Field {
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }
}
pub struct SerializeTupleVariant {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    len: usize,
    inner_ops: Vec<ast::TupleVariant>,
}
impl SerializeTupleVariant {
    pub fn new(name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Self {
        Self {
            name,
            variant_index,
            variant,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Ast;
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            name,
            variant_index,
            variant,
            len,
            inner_ops,
        } = self;

        Ok(Ast::TupleVariant {
            name,
            variant_index,
            variant,
            len,
            ops: inner_ops,
        })
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::TupleVariant::Field {
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }
}

pub struct SerializeSeq {
    len: Option<usize>,
    inner_ops: Vec<ast::Seq>,
}
impl SerializeSeq {
    pub fn new(len: Option<usize>) -> Self {
        Self {
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeSeq for SerializeSeq {
    type Ok = Ast;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Seq::Element {
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self { len, inner_ops } = self;

        Ok(Ast::Seq {
            len,
            ops: inner_ops,
        })
    }
}

pub struct SerializeMap {
    len: Option<usize>,
    inner_ops: Vec<ast::Map>,
}
impl SerializeMap {
    pub fn new(len: Option<usize>) -> Self {
        Self {
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeMap for SerializeMap {
    type Ok = Ast;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Map::Key {
            key: Box::new(to_ast(key)?),
        });
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Map::Value {
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self { len, inner_ops } = self;

        Ok(Ast::Map {
            len,
            ops: inner_ops,
        })
    }
}

pub struct SerializeStruct {
    name: &'static str,
    len: usize,
    inner_ops: Vec<ast::Struct>,
}
impl SerializeStruct {
    pub fn new(name: &'static str, len: usize) -> Self {
        Self {
            name,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeStruct for SerializeStruct {
    type Ok = Ast;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Struct::Field {
            key,
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            name,
            len,
            inner_ops,
        } = self;

        Ok(Ast::Struct {
            name,
            len,
            ops: inner_ops,
        })
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner_ops.push(ast::Struct::Skip { key });
        Ok(())
    }
}

pub struct SerializeStructVariant {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    len: usize,
    inner_ops: Vec<ast::StructVariant>,
}
impl SerializeStructVariant {
    pub fn new(name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Self {
        Self {
            name,
            variant_index,
            variant,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Ast;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::StructVariant::Field {
            key,
            value: Box::new(to_ast(value)?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            name,
            variant_index,
            variant,
            len,
            inner_ops,
        } = self;

        Ok(Ast::StructVariant {
            name,
            variant_index,
            variant,
            len,
            ops: inner_ops,
        })
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner_ops.push(ast::StructVariant::Skip { key });
        Ok(())
    }
}
