use std::fmt::Display;

use crate::{to_ast, MapOp, Op, SeqOp, StructOp, StructVariantOp};

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

pub struct Serializer<'ops> {
    ops: &'ops mut Vec<Op>,
}
impl<'ops> Serializer<'ops> {
    pub fn new(ops: &'ops mut Vec::<Op>) -> Self {
        Self {
            ops,
        }
    }
}
impl<'ops> serde::Serializer for Serializer<'ops> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeSeq::<'ops>;
    type SerializeTuple = SerializeTuple::<'ops>;
    type SerializeTupleStruct = SerializeTupleStruct::<'ops>;
    type SerializeTupleVariant = SerializeTupleVariant::<'ops>;
    type SerializeMap = SerializeMap::<'ops>;
    type SerializeStruct = SerializeStruct::<'ops>;
    type SerializeStructVariant = SerializeStructVariant::<'ops>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::Bool(v));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::I8(v));
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::I16(v));
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::I32(v));
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::I64(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::U8(v));
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::U16(v));
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::U32(v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::U64(v));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::F32(v));
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::F64(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::Char(v));
        Ok(())
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::Str(v.to_owned()));
        Ok(())
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::Bytes(v.to_owned()));
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::None);
        Ok(())
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.ops.push(Op::Some(to_ast(value)?));
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::Unit);
        Ok(())
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::UnitStruct(name.to_owned()));
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ops.push(Op::UnitVariant {
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
        self.ops.push(Op::NewtypeStruct {
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
        self.ops.push(Op::NewtypeVariant {
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
        todo!()
    }
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
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
pub struct SerializeSeq<'ops> {
    s: Serializer::<'ops>,
    len: Option<usize>,
    inner_ops: Vec<SeqOp>,
}
impl<'ops> SerializeSeq<'ops> {
    pub fn new(
        s: Serializer::<'ops>,
        len: Option<usize>,
    ) -> Self {
        Self {
            s,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ops> serde::ser::SerializeSeq for SerializeSeq<'ops> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(SeqOp::Element {
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

        s.ops.push(Op::Seq {
            len,
            ops: inner_ops,
        });
        Ok(())
    }
}

pub struct SerializeMap<'ops> {
    s: Serializer::<'ops>,
    len: Option<usize>,
    inner_ops: Vec<MapOp>,
}
impl<'ops> SerializeMap<'ops> {
    pub fn new(
        s: Serializer::<'ops>,
        len: Option<usize>,
    ) -> Self {
        Self {
            s,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl<'ops> serde::ser::SerializeMap for SerializeMap<'ops> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(MapOp::Key {
            key: to_ast(key)?,
        });
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(MapOp::Value {
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

        s.ops.push(Op::Map {
            len,
            ops: inner_ops,
        });
        Ok(())
    }
}

pub struct SerializeStruct<'ops> {
    s: Serializer::<'ops>,
    name: String,
    len: usize,
    inner_ops: Vec<StructOp>,
}
impl<'ops> SerializeStruct<'ops> {
    pub fn new(
        s: Serializer::<'ops>,
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
impl<'ops> serde::ser::SerializeStruct for SerializeStruct<'ops> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(StructOp::Field {
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

        s.ops.push(Op::Struct {
            name,
            len,
            ops: inner_ops,
        });
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner_ops.push(StructOp::Skip {
            key: key.to_owned(),
        });
        Ok(())
    }
}

pub struct SerializeStructVariant<'ops> {
    s: Serializer::<'ops>,
    name: String,
    variant_index: u32,
    variant: String,
    len: usize,
    inner_ops: Vec<StructVariantOp>,
}
impl<'ops> SerializeStructVariant<'ops> {
    pub fn new(
        s: Serializer::<'ops>,
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
impl<'ops> serde::ser::SerializeStructVariant for SerializeStructVariant<'ops> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize
    {
        self.inner_ops.push(StructVariantOp::Field {
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

        s.ops.push(Op::StructVariant {
            name,
            variant_index,
            variant,
            len,
            ops: inner_ops,
        });
        Ok(())
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner_ops.push(StructVariantOp::Skip {
            key: key.to_owned(),
        });
        Ok(())
    }
}
