use serde_ast::{
    ast::{self, XAst},
    to_xast_with_serializer, Ast,
};

use crate::Extras;

pub use serde_ast::ser::Error;

/// [Serializer] is a [serde::Serializer] for [Ast].
#[derive(Debug, Clone)]
pub struct Serializer {
    inner: serde_ast::Serializer,
}
impl Serializer {
    /// Create a new [Serializer].
    pub fn new() -> Self {
        Self {
            inner: serde_ast::Serializer::new(),
        }
    }
}

impl serde::Serializer for Serializer {
    type Ok = XAst<Extras<Ast>>;
    type Error = Error;

    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleStructVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    // The rest of these are just serializing to [serde_ast::Ast],
    // and lifting the output value to [XAst<Extras<Ast>>].

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bool(v).map(serde_ast::into_extended)
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i8(v).map(serde_ast::into_extended)
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i16(v).map(serde_ast::into_extended)
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i32(v).map(serde_ast::into_extended)
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i64(v).map(serde_ast::into_extended)
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u8(v).map(serde_ast::into_extended)
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u16(v).map(serde_ast::into_extended)
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u32(v).map(serde_ast::into_extended)
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u64(v).map(serde_ast::into_extended)
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f32(v).map(serde_ast::into_extended)
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f64(v).map(serde_ast::into_extended)
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_char(v).map(serde_ast::into_extended)
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v).map(serde_ast::into_extended)
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bytes(v).map(serde_ast::into_extended)
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none().map(serde_ast::into_extended)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner.serialize_some(value).map(serde_ast::into_extended)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit().map(serde_ast::into_extended)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name).map(serde_ast::into_extended)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
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
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq::new(self, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple::new(self, len))
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
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct SerializeSeq {
    serializer: Serializer,
    len: Option<usize>,
    inner_ops: Vec<ast::Seq<Extras<Ast>>>,
}
impl SerializeSeq {
    pub fn new(serializer: Serializer, len: Option<usize>) -> Self {
        Self {
            serializer,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeSeq for SerializeSeq {
    type Ok = XAst<Extras<Ast>>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Seq::Element {
            value: Box::new(to_xast_with_serializer(value, self.serializer.clone())?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let SerializeSeq { len, inner_ops, .. } = self;
        Ok(XAst::Seq {
            len,
            ops: inner_ops,
        })
    }
}

/// [serde::ser::SerializeTuple] for [Serializer]
#[derive(Debug, Clone)]
pub struct SerializeTuple {
    serializer: Serializer,
    len: usize,
    inner_ops: Vec<ast::Tuple<Extras<Ast>>>,
}
impl SerializeTuple {
    /// Create new [SerializeTuple].
    pub fn new(serializer: Serializer, len: usize) -> Self {
        Self {
            serializer,
            len,
            inner_ops: Vec::new(),
        }
    }
}
impl serde::ser::SerializeTuple for SerializeTuple {
    type Ok = XAst<Extras<Ast>>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.inner_ops.push(ast::Tuple::Element {
            value: Box::new(to_xast_with_serializer(value, self.serializer.clone())?),
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let Self { len, inner_ops, .. } = self;

        Ok(XAst::Tuple {
            len,
            ops: inner_ops,
        })
    }
}

/// [serde::ser::SerializeTupleStruct] for [Serializer]
#[derive(Debug, Clone, PartialEq)]
pub struct SerializeTupleStruct {
    name: &'static str,
    len: usize,
    inner_ops: Vec<ast::TupleStruct<Final>>,
}
impl SerializeTupleStruct {
    /// Create new [SerializeTupleStruct].
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

/// [serde::ser::SerializeTupleVariant] for [Serializer]
#[derive(Debug, Clone, PartialEq)]
pub struct SerializeTupleVariant {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    len: usize,
    inner_ops: Vec<ast::TupleVariant<Final>>,
}
impl SerializeTupleVariant {
    /// Create new [SerializeTupleVariant].
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


/// [serde::ser::SerializeMap] for [Serializer]
#[derive(Debug, Clone, PartialEq)]
pub struct SerializeMap {
    len: Option<usize>,
    inner_ops: Vec<ast::Map<Final>>,
}
impl SerializeMap {
    /// Create new [SerializeMap].
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

/// [serde::ser::SerializeStruct] for [Serializer]
#[derive(Debug, Clone, PartialEq)]
pub struct SerializeStruct {
    name: &'static str,
    len: usize,
    inner_ops: Vec<ast::Struct<Final>>,
}
impl SerializeStruct {
    /// Create new [SerializeStruct].
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

/// [serde::ser::SerializeStructVariant] for [Serializer]
#[derive(Debug, Clone, PartialEq)]
pub struct SerializeStructVariant {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    len: usize,
    inner_ops: Vec<ast::StructVariant<Final>>,
}
impl SerializeStructVariant {
    /// Create new [SerializeStructVariant].
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
