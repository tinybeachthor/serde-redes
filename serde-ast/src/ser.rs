use std::fmt::Display;

use crate::{ast, to_ast, Ast};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("custom error")]
    Custom(String),
}
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
        where T: Display
    {
        Self::Custom(msg.to_string())
    }
}

pub struct Serializer<'ast> {
    ops: &'ast mut Vec<Ast>,
}
impl<'ast> Serializer<'ast> {
    pub fn new(ops: &'ast mut Vec::<Ast>) -> Self {
        Self {
            ops,
        }
    }
}
impl<'ast> serde::Serializer for Serializer<'ast>{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeSeq::<'ast>;
    type SerializeTuple = SerializeTuple::<'ast>;
    type SerializeTupleStruct = SerializeTupleStruct::<'ast>;
    type SerializeTupleVariant = SerializeTupleVariant::<'ast>;
    type SerializeMap = SerializeMap::<'ast>;
    type SerializeStruct = SerializeStruct::<'ast>;
    type SerializeStructVariant = SerializeStructVariant::<'ast>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::Bool(v));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::I8(v));
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::I16(v));
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::I32(v));
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::I64(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::U8(v));
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::U16(v));
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::U32(v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::U64(v));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::F32(v));
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::F64(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::Char(v));
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::Str(v.to_owned()));
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::Bytes(v.to_owned()));
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::None);
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.ops.push(Ast::Some(to_ast(value)?));
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::Unit);
        Ok(())
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::UnitStruct(name.to_owned()));
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Ast::UnitVariant {
            name: name.to_owned(),
            variant_index,
            variant: variant.to_owned(),
        });
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.ops.push(Ast::NewtypeStruct {
            name: name.to_owned(),
            value: to_ast(value)?,
        });
        Ok(())
    }
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.ops.push(Ast::NewtypeVariant {
            name: name.to_owned(),
            variant_index,
            variant: variant.to_owned(),
            value: to_ast(value)?,
        });
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq::new(
            self,
            len,
        ))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple::new(
            self,
            len,
        ))
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct::new(
            self,
            name.to_owned(),
            len,
        ))
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant::new(
            self,
            name.to_owned(),
            variant_index,
            variant.to_owned(),
            len,
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap::new(
            self,
            len,
        ))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct::new(
            self,
            name,
            len,
        ))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant::new(
            self,
            name,
            variant_index,
            variant,
            len,
        ))
    }
}
pub struct SerializeTuple<'ast> {
    s: Serializer::<'ast>,
    len: usize,
    inner_ops: Vec<ast::Tuple>,
}
impl<'ast> SerializeTuple<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        len: usize,
    ) -> Self {
        Self {
            s,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeTuple for SerializeTuple<'ast> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::Tuple::Element {
            value: to_ast(value)?,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::Tuple {
            len,
            ops: inner_ops,
        });
        Ok(())
    }
}
pub struct SerializeTupleStruct<'ast> {
    s: Serializer::<'ast>,
    name: String,
    len: usize,
    inner_ops: Vec<ast::TupleStruct>,
}
impl<'ast> SerializeTupleStruct<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        name: String,
        len: usize,
    ) -> Self {
        Self {
            s,
            name,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeTupleStruct for SerializeTupleStruct<'ast> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            name,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::TupleStruct {
            name,
            len,
            ops: inner_ops,
        });
        Ok(())
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::TupleStruct::Field {
            value: to_ast(value)?,
        });
        Ok(())
    }
}
pub struct SerializeTupleVariant<'ast> {
    s: Serializer::<'ast>,
    name: String,
    variant_index: u32,
    variant: String,
    len: usize,
    inner_ops: Vec<ast::TupleVariant>,
}
impl<'ast> SerializeTupleVariant<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        name: String,
        variant_index: u32,
        variant: String,
        len: usize,
    ) -> Self {
        Self {
            s,
            name,
            variant_index,
            variant,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeTupleVariant for SerializeTupleVariant<'ast> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            name,
            variant_index,
            variant,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::TupleVariant {
            name,
            variant_index,
            variant,
            len,
            ops: inner_ops,
        });
        Ok(())
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::TupleVariant::Field {
            value: to_ast(value)?,
        });
        Ok(())
    }
}

pub struct SerializeSeq<'ast> {
    s: Serializer::<'ast>,
    len: Option<usize>,
    inner_ops: Vec<ast::Seq>,
}
impl<'ast> SerializeSeq<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        len: Option<usize>,
    ) -> Self {
        Self {
            s,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeSeq for SerializeSeq<'ast> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::Seq::Element {
            value: to_ast(value)?,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::Seq {
            len,
            ops: inner_ops,
        });
        Ok(())
    }
}

pub struct SerializeMap<'ast> {
    s: Serializer::<'ast>,
    len: Option<usize>,
    inner_ops: Vec<ast::Map>,
}
impl<'ast> SerializeMap<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        len: Option<usize>,
    ) -> Self {
        Self {
            s,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeMap for SerializeMap<'ast> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::Map::Key {
            key: to_ast(key)?,
        });
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::Map::Value {
            value: to_ast(value)?,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::Map {
            len,
            ops: inner_ops,
        });
        Ok(())
    }
}

pub struct SerializeStruct<'ast> {
    s: Serializer::<'ast>,
    name: String,
    len: usize,
    inner_ops: Vec<ast::Struct>,
}
impl<'ast> SerializeStruct<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        name: &str,
        len: usize,
    ) -> Self {
        Self {
            s,
            name: name.to_owned(),
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeStruct for SerializeStruct<'ast> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::Struct::Field {
            key: key.to_owned(),
            value: to_ast(value)?,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            name,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::Struct {
            name,
            len,
            ops: inner_ops,
        });
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner_ops.push(ast::Struct::Skip {
            key: key.to_owned(),
        });
        Ok(())
    }
}

pub struct SerializeStructVariant<'ast> {
    s: Serializer::<'ast>,
    name: String,
    variant_index: u32,
    variant: String,
    len: usize,
    inner_ops: Vec<ast::StructVariant>,
}
impl<'ast> SerializeStructVariant<'ast> {
    pub fn new(
        s: Serializer::<'ast>,
        name: &str,
        variant_index: u32,
        variant: &str,
        len: usize,
    ) -> Self {
        Self {
            s,
            name: name.to_owned(),
            variant_index,
            variant: variant.to_owned(),
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ast> serde::ser::SerializeStructVariant for SerializeStructVariant<'ast> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(ast::StructVariant::Field {
            key: key.to_owned(),
            value: to_ast(value)?,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self {
            s,
            name,
            variant_index,
            variant,
            len,
            inner_ops,
         } = self;

        s.ops.push(Ast::StructVariant {
            name,
            variant_index,
            variant,
            len,
            ops: inner_ops,
        });
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner_ops.push(ast::StructVariant::Skip {
            key: key.to_owned(),
        });
        Ok(())
    }
}
