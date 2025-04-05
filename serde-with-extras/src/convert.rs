use serde_ast::{
    ast::{Final, Map, Seq, Struct, StructVariant, Tuple, TupleStruct, TupleVariant},
    into_extended, Ast, XAst,
};

use crate::ExtrasAttributes;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Extras is missing an expected field.")]
    ExtrasMissingField,
    #[error("Extras attrbutes are in the wrong format, expected stringified JSON.")]
    ExtrasAttributesWrongFormat,
    #[error("Failed to deserialize extras attributes: {0}")]
    DeserializeExtrasAttributes(#[source] serde_json::Error),
}

pub fn lift(ast: Ast) -> Result<XAst<ExtrasAttributes>, Error> {
    println!("{:?}", ast);

    Ok(match ast {
        XAst::TupleStruct { name, len, ops } => {
            if name == super::SERDE_EXTRAS_WELLKNOWN_NAME {
                let TupleStruct::Field { value: inner } =
                    ops.first().ok_or(Error::ExtrasMissingField)?;
                let TupleStruct::Field { value: extras } =
                    ops.get(1).ok_or(Error::ExtrasMissingField)?;

                let XAst::Str(extras_serialized) = extras.as_ref() else {
                    return Err(Error::ExtrasAttributesWrongFormat);
                };

                let extras: ExtrasAttributes = serde_json::from_str(extras_serialized)
                    .map_err(Error::DeserializeExtrasAttributes)?;

                XAst::X(extras, Box::new(lift(inner.as_ref().clone())?))
            } else {
                XAst::TupleStruct {
                    name,
                    len,
                    ops: ops
                        .into_iter()
                        .map(lift_tuple_struct)
                        .collect::<Result<Vec<_>, Error>>()?,
                }
            }
        }
        //
        // lift/pass types for the rest
        //
        XAst::Some(value) => XAst::Some(Box::new(lift(*value)?)),
        XAst::NewtypeStruct { name, value } => XAst::NewtypeStruct {
            name,
            value: Box::new(lift(*value)?),
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
            value: Box::new(lift(*value)?),
        },
        XAst::Seq { len, ops } => XAst::Seq {
            len,
            ops: ops
                .into_iter()
                .map(lift_seq)
                .collect::<Result<Vec<_>, Error>>()?,
        },
        XAst::Tuple { len, ops } => XAst::Tuple {
            len,
            ops: ops
                .into_iter()
                .map(lift_tuple)
                .collect::<Result<Vec<_>, Error>>()?,
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
            ops: ops
                .into_iter()
                .map(lift_tuple_variant)
                .collect::<Result<Vec<_>, Error>>()?,
        },
        XAst::Map { len, ops } => XAst::Map {
            len,
            ops: ops
                .into_iter()
                .map(lift_map)
                .collect::<Result<Vec<_>, Error>>()?,
        },
        XAst::Struct { name, len, ops } => XAst::Struct {
            name,
            len,
            ops: ops
                .into_iter()
                .map(lift_struct)
                .collect::<Result<Vec<_>, Error>>()?,
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
            ops: ops
                .into_iter()
                .map(lift_struct_variant)
                .collect::<Result<Vec<_>, Error>>()?,
        },
        _ => into_extended(ast),
    })
}

fn lift_seq(v: Seq<Final>) -> Result<Seq<ExtrasAttributes>, Error> {
    Ok(match v {
        Seq::Element { value } => Seq::Element {
            value: Box::new(lift(*value)?),
        },
    })
}
fn lift_tuple(v: Tuple<Final>) -> Result<Tuple<ExtrasAttributes>, Error> {
    Ok(match v {
        Tuple::Element { value } => Tuple::Element {
            value: Box::new(lift(*value)?),
        },
    })
}
fn lift_tuple_struct(v: TupleStruct<Final>) -> Result<TupleStruct<ExtrasAttributes>, Error> {
    Ok(match v {
        TupleStruct::Field { value } => TupleStruct::Field {
            value: Box::new(lift(*value)?),
        },
    })
}
fn lift_tuple_variant(v: TupleVariant<Final>) -> Result<TupleVariant<ExtrasAttributes>, Error> {
    Ok(match v {
        TupleVariant::Field { value } => TupleVariant::Field {
            value: Box::new(lift(*value)?),
        },
    })
}
fn lift_map(v: Map<Final>) -> Result<Map<ExtrasAttributes>, Error> {
    Ok(match v {
        Map::Key { key } => Map::Key {
            key: Box::new(lift(*key)?),
        },
        Map::Value { value } => Map::Value {
            value: Box::new(lift(*value)?),
        },
    })
}
fn lift_struct(v: Struct<Final>) -> Result<Struct<ExtrasAttributes>, Error> {
    Ok(match v {
        Struct::Field { key, value } => Struct::Field {
            key,
            value: Box::new(lift(*value)?),
        },
        Struct::Skip { key } => Struct::Skip { key },
    })
}
fn lift_struct_variant(v: StructVariant<Final>) -> Result<StructVariant<ExtrasAttributes>, Error> {
    Ok(match v {
        StructVariant::Field { key, value } => StructVariant::Field {
            key,
            value: Box::new(lift(*value)?),
        },
        StructVariant::Skip { key } => StructVariant::Skip { key },
    })
}
