//! Define the [Ast] representation of [serde] serialization.

use std::fmt::Display;

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};

/// Represent calls made to [serde::Serializer] during serialization.
#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
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
    Some(Box<Ast>),

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
        value: Box<Ast>,
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
        value: Box<Ast>,
    },

    /// [serde::Serializer::serialize_seq]
    Seq {
        /// len
        len: Option<usize>,
        /// [serde::ser::SerializeSeq] operations
        ops: Vec<Seq>,
    },

    /// [serde::Serializer::serialize_tuple]
    Tuple {
        /// len
        len: usize,
        /// [serde::ser::SerializeTuple] operations
        ops: Vec<Tuple>,
    },
    /// [serde::Serializer::serialize_tuple_struct]
    TupleStruct {
        /// name
        name: &'static str,
        /// len
        len: usize,
        /// [serde::ser::SerializeTupleStruct] operations
        ops: Vec<TupleStruct>,
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
        ops: Vec<TupleVariant>,
    },

    /// [serde::Serializer::serialize_map]
    Map {
        /// len
        len: Option<usize>,
        /// [serde::ser::SerializeMap] operations
        ops: Vec<Map>,
    },
    /// [serde::Serializer::serialize_struct]
    Struct {
        /// name
        name: &'static str,
        /// len
        len: usize,
        /// [serde::ser::SerializeStruct] operations
        ops: Vec<Struct>,
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
        ops: Vec<StructVariant>,
    },
}
impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl Serialize for Ast {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Ast::Bool(v) => serializer.serialize_bool(*v),
            Ast::I8(v) => serializer.serialize_i8(*v),
            Ast::I16(v) => serializer.serialize_i16(*v),
            Ast::I32(v) => serializer.serialize_i32(*v),
            Ast::I64(v) => serializer.serialize_i64(*v),
            Ast::U8(v) => serializer.serialize_u8(*v),
            Ast::U16(v) => serializer.serialize_u16(*v),
            Ast::U32(v) => serializer.serialize_u32(*v),
            Ast::U64(v) => serializer.serialize_u64(*v),
            Ast::F32(v) => serializer.serialize_f32(*v),
            Ast::F64(v) => serializer.serialize_f64(*v),
            Ast::Char(v) => serializer.serialize_char(*v),
            Ast::Str(v) => serializer.serialize_str(v),
            Ast::Bytes(v) => serializer.serialize_bytes(v),
            Ast::None => serializer.serialize_none(),
            Ast::Some(v) => serializer.serialize_some(v),
            Ast::Unit => serializer.serialize_unit(),
            Ast::UnitStruct(name) => serializer.serialize_unit_struct(name),
            Ast::UnitVariant {
                name,
                variant_index,
                variant,
            } => serializer.serialize_unit_variant(name, *variant_index, variant),
            Ast::NewtypeStruct { name, value } => serializer.serialize_newtype_struct(name, value),
            Ast::NewtypeVariant {
                name,
                variant_index,
                variant,
                value,
            } => serializer.serialize_newtype_variant(name, *variant_index, variant, value),
            Ast::Seq { len, ops } => {
                let mut s = serializer.serialize_seq(*len)?;
                for op in ops {
                    match op {
                        Seq::Element { value } => s.serialize_element(value)?,
                    }
                }
                s.end()
            }
            Ast::Tuple { len, ops } => {
                let mut s = serializer.serialize_tuple(*len)?;
                for op in ops {
                    match op {
                        Tuple::Element { value } => s.serialize_element(value)?,
                    }
                }
                s.end()
            }
            Ast::TupleStruct { name, len, ops } => {
                let mut s = serializer.serialize_tuple_struct(name, *len)?;
                for op in ops {
                    match op {
                        TupleStruct::Field { value } => s.serialize_field(value)?,
                    }
                }
                s.end()
            }
            Ast::TupleVariant {
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
            Ast::Map { len, ops } => {
                let mut s = serializer.serialize_map(*len)?;
                for op in ops {
                    match op {
                        Map::Key { key } => s.serialize_key(key)?,
                        Map::Value { value } => s.serialize_value(value)?,
                    }
                }
                s.end()
            }
            Ast::Struct { name, len, ops } => {
                let mut s = serializer.serialize_struct(name, *len)?;
                for op in ops {
                    match op {
                        Struct::Field { key, value } => s.serialize_field(key, value)?,
                        Struct::Skip { key } => s.skip_field(key)?,
                    }
                }
                s.end()
            }
            Ast::StructVariant {
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
        }
    }
}

/// [serde::ser::SerializeTuple]
#[derive(Debug, Clone, PartialEq)]
pub enum Tuple {
    /// [serde::ser::SerializeTuple::serialize_element]
    Element {
        /// value
        value: Box<Ast>,
    },
}

/// [serde::ser::SerializeTupleStruct]
#[derive(Debug, Clone, PartialEq)]
pub enum TupleStruct {
    /// [serde::ser::SerializeTupleStruct::serialize_field]
    Field {
        /// value
        value: Box<Ast>,
    },
}

/// [serde::ser::SerializeTupleVariant]
#[derive(Debug, Clone, PartialEq)]
pub enum TupleVariant {
    /// [serde::ser::SerializeTupleVariant::serialize_field]
    Field {
        /// value
        value: Box<Ast>,
    },
}

/// [serde::ser::SerializeSeq]
#[derive(Debug, Clone, PartialEq)]
pub enum Seq {
    /// [serde::ser::SerializeSeq::serialize_element]
    Element {
        /// value
        value: Box<Ast>,
    },
}

/// [serde::ser::SerializeMap]
#[derive(Debug, Clone, PartialEq)]
pub enum Map {
    /// [serde::ser::SerializeMap::serialize_key]
    Key {
        /// key
        key: Box<Ast>,
    },
    /// [serde::ser::SerializeMap::serialize_value]
    Value {
        /// value
        value: Box<Ast>,
    },
}

/// [serde::ser::SerializeStruct]
#[derive(Debug, Clone, PartialEq)]
pub enum Struct {
    /// [serde::ser::SerializeStruct::serialize_field]
    Field {
        /// key
        key: &'static str,
        /// value
        value: Box<Ast>,
    },
    /// [serde::ser::SerializeStruct::skip_field]
    Skip {
        /// key
        key: &'static str,
    },
}

/// [serde::ser::SerializeStructVariant]
#[derive(Debug, Clone, PartialEq)]
pub enum StructVariant {
    /// [serde::ser::SerializeStructVariant::serialize_field]
    Field {
        /// key
        key: &'static str,
        /// value
        value: Box<Ast>,
    },
    /// [serde::ser::SerializeStructVariant::skip_field]
    Skip {
        /// key
        key: &'static str,
    },
}
