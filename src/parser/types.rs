use wasm_encoder::*;

pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

pub enum Type {
    Array(StorageType, bool),
    Func(FuncType),
}

pub struct Import {
    pub module: String,
    pub name: String,
    pub ty: EntityType,
}
