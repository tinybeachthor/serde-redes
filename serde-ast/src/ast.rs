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
    Some(Vec<Ast>),

    Unit,
    UnitStruct(String),
    UnitVariant {
        name: String,
        variant_index: u32,
        variant: String,
    },

    NewtypeStruct {
        name: String,
        value: Vec<Ast>,
    },
    NewtypeVariant {
        name: String,
        variant_index: u32,
        variant: String,
        value: Vec<Ast>,
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
        name: String,
        len: usize,
        ops: Vec<TupleStruct>,
    },
    TupleVariant {
        name: String,
        variant_index: u32,
        variant: String,
        len: usize,
        ops: Vec<TupleVariant>,
    },

    Map {
        len: Option<usize>,
        ops: Vec<Map>,
    },
    Struct {
        name: String,
        len: usize,
        ops: Vec<Struct>,
    },
    StructVariant {
        name: String,
        variant_index: u32,
        variant: String,
        len: usize,
        ops: Vec<StructVariant>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Tuple {
    Element {
        value: Vec<Ast>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleStruct {
    Field {
        value: Vec<Ast>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum TupleVariant {
    Field {
        value: Vec<Ast>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Seq {
    Element {
        value: Vec<Ast>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Map {
    Key {
        key: Vec<Ast>,
    },
    Value {
        value: Vec<Ast>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum Struct {
    Field {
        key: String,
        value: Vec<Ast>,
    },
    Skip {
        key: String,
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum StructVariant {
    Field {
        key: String,
        value: Vec<Ast>,
    },
    Skip {
        key: String,
    }
}
