//! Define the [Ast] representation of [serde] serialization.

use std::fmt::Display;

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};

/// Represents an empty type. This can never be constructed.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Final {}

/// Define a closed [Ast] without any injected type extension.
pub type Ast = XAst<Final>;

/// Represent calls made to [serde::Serializer] during serialization.
///
/// This is an extensible
#[derive(Debug, Clone, PartialEq)]
pub enum XAst<X> {
    /// [serde::Serializer::serialize_bool]
    Bool(bool),

    /// [serde::Serializer::serialize_i8]
    I8(i8),
    /// [serde::Serializer::serialize_i16]
    I16(i16),
    /// [serde::Serializer::serialize_i32]
    I32(i32),
    /// [serde::Serializer::serialize_i64]
    I64(i64),

    /// [serde::Serializer::serialize_u8]
    U8(u8),
    /// [serde::Serializer::serialize_u16]
    U16(u16),
    /// [serde::Serializer::serialize_u32]
    U32(u32),
    /// [serde::Serializer::serialize_u64]
    U64(u64),

    /// [serde::Serializer::serialize_f32]
    F32(f32),
    /// [serde::Serializer::serialize_f64]
    F64(f64),

    /// [serde::Serializer::serialize_char]
    Char(char),
    /// [serde::Serializer::serialize_str]
    Str(String),
    /// [serde::Serializer::serialize_bytes]
    Bytes(Vec<u8>),

    /// [serde::Serializer::serialize_none]
    None,
    /// [serde::Serializer::serialize_some]
    Some(Box<XAst<X>>),

    /// [serde::Serializer::serialize_unit]
    Unit,
    /// [serde::Serializer::serialize_unit_struct]
    UnitStruct(&'static str),
    /// [serde::Serializer::serialize_unit_variant]
    UnitVariant {
        /// name
        name: &'static str,
        /// variant_index
        variant_index: u32,
        /// variant
        variant: &'static str,
    },

    /// [serde::Serializer::serialize_newtype_struct]
    NewtypeStruct {
        /// name
        name: &'static str,
        /// value
        value: Box<XAst<X>>,
    },
    /// [serde::Serializer::serialize_newtype_variant]
    NewtypeVariant {
        /// name
        name: &'static str,
        /// variant_index
        variant_index: u32,
        /// variant
        variant: &'static str,
        /// value
        value: Box<XAst<X>>,
    },

    /// [serde::Serializer::serialize_seq]
    Seq {
        /// len
        len: Option<usize>,
        /// [serde::ser::SerializeSeq] operations
        ops: Vec<Seq<X>>,
    },

    /// [serde::Serializer::serialize_tuple]
    Tuple {
        /// len
        len: usize,
        /// [serde::ser::SerializeTuple] operations
        ops: Vec<Tuple<X>>,
    },
    /// [serde::Serializer::serialize_tuple_struct]
    TupleStruct {
        /// name
        name: &'static str,
        /// len
        len: usize,
        /// [serde::ser::SerializeTupleStruct] operations
        ops: Vec<TupleStruct<X>>,
    },
    /// [serde::Serializer::serialize_tuple_variant]
    TupleVariant {
        /// name
        name: &'static str,
        /// variant_index
        variant_index: u32,
        /// variant
        variant: &'static str,
        /// len
        len: usize,
        /// [serde::ser::SerializeTupleVariant] operations
        ops: Vec<TupleVariant<X>>,
    },

    /// [serde::Serializer::serialize_map]
    Map {
        /// len
        len: Option<usize>,
        /// [serde::ser::SerializeMap] operations
        ops: Vec<Map<X>>,
    },
    /// [serde::Serializer::serialize_struct]
    Struct {
        /// name
        name: &'static str,
        /// len
        len: usize,
        /// [serde::ser::SerializeStruct] operations
        ops: Vec<Struct<X>>,
    },
    /// [serde::Serializer::serialize_struct_variant]
    StructVariant {
        /// name
        name: &'static str,
        /// variant_index
        variant_index: u32,
        /// variant
        variant: &'static str,
        /// len
        len: usize,
        /// [serde::ser::SerializeStructVariant] operations
        ops: Vec<StructVariant<X>>,
    },

    /// Allow arbitrary extensions of this enum by injecting an extension type.
    X(X),
}

impl<X> Display for XAst<X>
where
    X: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Serialize for XAst<Final> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bool(v) => serializer.serialize_bool(*v),
            Self::I8(v) => serializer.serialize_i8(*v),
            Self::I16(v) => serializer.serialize_i16(*v),
            Self::I32(v) => serializer.serialize_i32(*v),
            Self::I64(v) => serializer.serialize_i64(*v),
            Self::U8(v) => serializer.serialize_u8(*v),
            Self::U16(v) => serializer.serialize_u16(*v),
            Self::U32(v) => serializer.serialize_u32(*v),
            Self::U64(v) => serializer.serialize_u64(*v),
            Self::F32(v) => serializer.serialize_f32(*v),
            Self::F64(v) => serializer.serialize_f64(*v),
            Self::Char(v) => serializer.serialize_char(*v),
            Self::Str(v) => serializer.serialize_str(v),
            Self::Bytes(v) => serializer.serialize_bytes(v),
            Self::None => serializer.serialize_none(),
            Self::Some(v) => serializer.serialize_some(v),
            Self::Unit => serializer.serialize_unit(),
            Self::UnitStruct(name) => serializer.serialize_unit_struct(name),
            Self::UnitVariant {
                name,
                variant_index,
                variant,
            } => serializer.serialize_unit_variant(name, *variant_index, variant),
            Self::NewtypeStruct { name, value } => serializer.serialize_newtype_struct(name, value),
            Self::NewtypeVariant {
                name,
                variant_index,
                variant,
                value,
            } => serializer.serialize_newtype_variant(name, *variant_index, variant, value),
            Self::Seq { len, ops } => {
                let mut s = serializer.serialize_seq(*len)?;
                for op in ops {
                    match op {
                        Seq::Element { value } => s.serialize_element(&value)?,
                    }
                }
                s.end()
            }
            Self::Tuple { len, ops } => {
                let mut s = serializer.serialize_tuple(*len)?;
                for op in ops {
                    match op {
                        Tuple::Element { value } => s.serialize_element(value)?,
                    }
                }
                s.end()
            }
            Self::TupleStruct { name, len, ops } => {
                let mut s = serializer.serialize_tuple_struct(name, *len)?;
                for op in ops {
                    match op {
                        TupleStruct::Field { value } => s.serialize_field(value)?,
                    }
                }
                s.end()
            }
            Self::TupleVariant {
                name,
                variant_index,
                variant,
                len,
                ops,
            } => {
                let mut s =
                    serializer.serialize_tuple_variant(name, *variant_index, variant, *len)?;
                for op in ops {
                    match op {
                        TupleVariant::Field { value } => s.serialize_field(value)?,
                    }
                }
                s.end()
            }
            Self::Map { len, ops } => {
                let mut s = serializer.serialize_map(*len)?;
                for op in ops {
                    match op {
                        Map::Key { key } => s.serialize_key(key)?,
                        Map::Value { value } => s.serialize_value(value)?,
                    }
                }
                s.end()
            }
            Self::Struct { name, len, ops } => {
                let mut s = serializer.serialize_struct(name, *len)?;
                for op in ops {
                    match op {
                        Struct::Field { key, value } => s.serialize_field(key, value)?,
                        Struct::Skip { key } => s.skip_field(key)?,
                    }
                }
                s.end()
            }
            Self::StructVariant {
                name,
                variant_index,
                variant,
                len,
                ops,
            } => {
                let mut s =
                    serializer.serialize_struct_variant(name, *variant_index, variant, *len)?;
                for op in ops {
                    match op {
                        StructVariant::Field { key, value } => s.serialize_field(key, value)?,
                        StructVariant::Skip { key } => s.skip_field(key)?,
                    }
                }
                s.end()
            }
            Self::X(never) => match *never {},
        }
    }
}

/// [serde::ser::SerializeTuple]
#[derive(Debug, Clone, PartialEq)]
pub enum Tuple<X> {
    /// [serde::ser::SerializeTuple::serialize_element]
    Element {
        /// value
        value: Box<XAst<X>>,
    },
}

/// [serde::ser::SerializeTupleStruct]
#[derive(Debug, Clone, PartialEq)]
pub enum TupleStruct<X> {
    /// [serde::ser::SerializeTupleStruct::serialize_field]
    Field {
        /// value
        value: Box<XAst<X>>,
    },
}

/// [serde::ser::SerializeTupleVariant]
#[derive(Debug, Clone, PartialEq)]
pub enum TupleVariant<X> {
    /// [serde::ser::SerializeTupleVariant::serialize_field]
    Field {
        /// value
        value: Box<XAst<X>>,
    },
}

/// [serde::ser::SerializeSeq]
#[derive(Debug, Clone, PartialEq)]
pub enum Seq<X> {
    /// [serde::ser::SerializeSeq::serialize_element]
    Element {
        /// value
        value: Box<XAst<X>>,
    },
}

/// [serde::ser::SerializeMap]
#[derive(Debug, Clone, PartialEq)]
pub enum Map<X> {
    /// [serde::ser::SerializeMap::serialize_key]
    Key {
        /// key
        key: Box<XAst<X>>,
    },
    /// [serde::ser::SerializeMap::serialize_value]
    Value {
        /// value
        value: Box<XAst<X>>,
    },
}

/// [serde::ser::SerializeStruct]
#[derive(Debug, Clone, PartialEq)]
pub enum Struct<X> {
    /// [serde::ser::SerializeStruct::serialize_field]
    Field {
        /// key
        key: &'static str,
        /// value
        value: Box<XAst<X>>,
    },
    /// [serde::ser::SerializeStruct::skip_field]
    Skip {
        /// key
        key: &'static str,
    },
}

/// [serde::ser::SerializeStructVariant]
#[derive(Debug, Clone, PartialEq)]
pub enum StructVariant<X> {
    /// [serde::ser::SerializeStructVariant::serialize_field]
    Field {
        /// key
        key: &'static str,
        /// value
        value: Box<XAst<X>>,
    },
    /// [serde::ser::SerializeStructVariant::skip_field]
    Skip {
        /// key
        key: &'static str,
    },
}
