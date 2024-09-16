#[derive(Debug, Clone, PartialEq)]
pub enum Op {
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
    Some(Vec<Op>),

    Unit,
    UnitStruct(String),
    UnitVariant {
        name: String,
        variant_index: u32,
        variant: String,
    },

    NewtypeStruct {
        name: String,
        value: Vec<Op>,
    },
    NewtypeVariant {
        name: String,
        variant_index: u32,
        variant: String,
        value: Vec<Op>,
    },

    Seq {
        len: Option<usize>,
        ops: Vec<SeqOp>,
    },

    Tuple {
        len: usize,
        ops: Vec<TupleOp>,
    },
    TupleStruct {
        name: String,
        len: usize,
        ops: Vec<TupleStructOp>,
    },
    TupleVariant {
        name: String,
        variant_index: u32,
        variant: String,
        len: usize,
        ops: Vec<TupleVariantOp>,
    },

    Map {
        len: Option<usize>,
        ops: Vec<MapOp>,
    },
    Struct {
        name: String,
        len: usize,
        ops: Vec<StructOp>,
    },
    StructVariant {
        name: String,
        variant_index: u32,
        variant: String,
        len: usize,
        ops: Vec<StructVariantOp>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleOp {
    Element {
        value: Vec<Op>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleStructOp {
    Field {
        value: Vec<Op>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleVariantOp {
    Field {
        value: Vec<Op>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum SeqOp {
    Element {
        value: Vec<Op>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum MapOp {
    Key {
        key: Vec<Op>,
    },
    Value {
        value: Vec<Op>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum StructOp {
    Field {
        key: String,
        value: Vec<Op>,
    },
    Skip {
        key: String,
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum StructVariantOp {
    Field {
        key: String,
        value: Vec<Op>,
    },
    Skip {
        key: String,
    }
}
