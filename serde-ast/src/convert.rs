use crate::{
    ast::{self, Final, Map, Seq, Struct, StructVariant, Tuple, TupleStruct, TupleVariant},
    XAst,
};

/// Convert [XAst<Final>] into extended [XAst].
pub fn into_extended<T>(value: XAst<ast::Final>) -> XAst<T> {
    match value {
        XAst::Bool(v) => XAst::Bool(v),
        XAst::I8(v) => XAst::I8(v),
        XAst::I16(v) => XAst::I16(v),
        XAst::I32(v) => XAst::I32(v),
        XAst::I64(v) => XAst::I64(v),
        XAst::U8(v) => XAst::U8(v),
        XAst::U16(v) => XAst::U16(v),
        XAst::U32(v) => XAst::U32(v),
        XAst::U64(v) => XAst::U64(v),
        XAst::F32(v) => XAst::F32(v),
        XAst::F64(v) => XAst::F64(v),
        XAst::Char(v) => XAst::Char(v),
        XAst::Str(v) => XAst::Str(v),
        XAst::Bytes(vec) => XAst::Bytes(vec),
        XAst::None => XAst::None,
        XAst::Some(xast) => XAst::Some(Box::new(into_extended(*xast))),
        XAst::Unit => XAst::Unit,
        XAst::UnitStruct(v) => XAst::UnitStruct(v),
        XAst::UnitVariant {
            name,
            variant_index,
            variant,
        } => XAst::UnitVariant {
            name,
            variant_index,
            variant,
        },
        XAst::NewtypeStruct { name, value } => XAst::NewtypeStruct {
            name,
            value: Box::new(into_extended(*value)),
        },
        XAst::NewtypeVariant {
            name,
            variant_index,
            variant,
            value,
        } => XAst::NewtypeVariant {
            name,
            variant_index,
            variant,
            value: Box::new(into_extended(*value)),
        },
        XAst::Seq { len, ops } => XAst::Seq {
            len,
            ops: ops.into_iter().map(into_extended_seq).collect(),
        },
        XAst::Tuple { len, ops } => XAst::Tuple {
            len,
            ops: ops.into_iter().map(into_extended_tuple).collect(),
        },
        XAst::TupleStruct { name, len, ops } => XAst::TupleStruct {
            name,
            len,
            ops: ops.into_iter().map(into_extended_tuple_struct).collect(),
        },
        XAst::TupleVariant {
            name,
            variant_index,
            variant,
            len,
            ops,
        } => XAst::TupleVariant {
            name,
            variant_index,
            variant,
            len,
            ops: ops.into_iter().map(into_extended_tuple_variant).collect(),
        },
        XAst::Map { len, ops } => XAst::Map {
            len,
            ops: ops.into_iter().map(into_extended_map).collect(),
        },
        XAst::Struct { name, len, ops } => XAst::Struct {
            name,
            len,
            ops: ops.into_iter().map(into_extended_struct).collect(),
        },
        XAst::StructVariant {
            name,
            variant_index,
            variant,
            len,
            ops,
        } => XAst::StructVariant {
            name,
            variant_index,
            variant,
            len,
            ops: ops.into_iter().map(into_extended_struct_variant).collect(),
        },
        XAst::X(v) => match v {},
    }
}

fn into_extended_seq<T>(v: Seq<Final>) -> Seq<T> {
    match v {
        Seq::Element { value } => Seq::Element {
            value: Box::new(into_extended(*value)),
        },
    }
}

fn into_extended_tuple<T>(v: Tuple<Final>) -> Tuple<T> {
    match v {
        Tuple::Element { value } => Tuple::Element {
            value: Box::new(into_extended(*value)),
        },
    }
}

fn into_extended_tuple_struct<T>(v: TupleStruct<Final>) -> TupleStruct<T> {
    match v {
        TupleStruct::Field { value } => TupleStruct::Field {
            value: Box::new(into_extended(*value)),
        },
    }
}

fn into_extended_tuple_variant<T>(v: TupleVariant<Final>) -> TupleVariant<T> {
    match v {
        TupleVariant::Field { value } => TupleVariant::Field {
            value: Box::new(into_extended(*value)),
        },
    }
}

fn into_extended_map<T>(v: Map<Final>) -> Map<T> {
    match v {
        Map::Key { key } => Map::Key {
            key: Box::new(into_extended(*key)),
        },
        Map::Value { value } => Map::Value {
            value: Box::new(into_extended(*value)),
        },
    }
}

fn into_extended_struct<T>(v: Struct<Final>) -> Struct<T> {
    match v {
        Struct::Field { key, value } => Struct::Field {
            key,
            value: Box::new(into_extended(*value)),
        },
        Struct::Skip { key } => Struct::Skip { key },
    }
}

fn into_extended_struct_variant<T>(v: StructVariant<Final>) -> StructVariant<T> {
    match v {
        StructVariant::Field { key, value } => StructVariant::Field {
            key,
            value: Box::new(into_extended(*value)),
        },
        StructVariant::Skip { key } => StructVariant::Skip { key },
    }
}
