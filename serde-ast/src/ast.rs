use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Bool(bool),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    F32(f32),
    F64(f64),

    Char(char),
    Str(String),
    Bytes(Vec<u8>),

    None,
    Some(Box<Ast>),

    Unit,
    UnitStruct(&'static str),
    UnitVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    },

    NewtypeStruct {
        name: &'static str,
        value: Box<Ast>,
    },
    NewtypeVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: Box<Ast>,
    },

    Seq {
        len: Option<usize>,
        ops: Vec<Seq>,
    },

    Tuple {
        len: usize,
        ops: Vec<Tuple>,
    },
    TupleStruct {
        name: &'static str,
        len: usize,
        ops: Vec<TupleStruct>,
    },
    TupleVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        ops: Vec<TupleVariant>,
    },

    Map {
        len: Option<usize>,
        ops: Vec<Map>,
    },
    Struct {
        name: &'static str,
        len: usize,
        ops: Vec<Struct>,
    },
    StructVariant {
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
        ops: Vec<StructVariant>,
    },
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

#[derive(Debug, Clone, PartialEq)]
pub enum Tuple {
    Element { value: Box<Ast> },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleStruct {
    Field { value: Box<Ast> },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleVariant {
    Field { value: Box<Ast> },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Seq {
    Element { value: Box<Ast> },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Map {
    Key { key: Box<Ast> },
    Value { value: Box<Ast> },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Struct {
    Field { key: &'static str, value: Box<Ast> },
    Skip { key: &'static str },
}
#[derive(Debug, Clone, PartialEq)]
pub enum StructVariant {
    Field { key: &'static str, value: Box<Ast> },
    Skip { key: &'static str },
}
