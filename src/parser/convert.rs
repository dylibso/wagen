use crate::parser::types::*;
use wasm_encoder::*;

pub trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<HeapType> for wasmparser::HeapType {
    fn convert(self) -> HeapType {
        use wasmparser::HeapType::*;
        match self {
            Indexed(i) => HeapType::Indexed(i),
            Func => HeapType::Func,
            Extern => HeapType::Extern,
            Any => HeapType::Any,
            None => HeapType::None,
            NoExtern => HeapType::NoExtern,
            NoFunc => HeapType::NoFunc,
            Eq => HeapType::Eq,
            Struct => HeapType::Struct,
            Array => HeapType::Array,
            I31 => HeapType::I31,
        }
    }
}

impl Convert<RefType> for wasmparser::RefType {
    fn convert(self) -> RefType {
        match self {
            wasmparser::RefType::FUNCREF => RefType::FUNCREF,
            wasmparser::RefType::EXTERNREF => RefType::EXTERNREF,
            t => RefType {
                nullable: t.is_nullable(),
                heap_type: t.heap_type().convert(),
            },
        }
    }
}

impl Convert<ValType> for wasmparser::ValType {
    fn convert(self) -> ValType {
        match self {
            wasmparser::ValType::I32 => ValType::I32,
            wasmparser::ValType::I64 => ValType::I64,
            wasmparser::ValType::F32 => ValType::F32,
            wasmparser::ValType::F64 => ValType::F64,
            wasmparser::ValType::V128 => ValType::V128,
            wasmparser::ValType::Ref(r) => ValType::Ref(r.convert()),
        }
    }
}

impl Convert<FuncType> for wasmparser::FuncType {
    fn convert(self) -> FuncType {
        FuncType {
            params: self.params().to_vec().convert(),
            results: self.results().to_vec().convert(),
        }
    }
}

impl Convert<StorageType> for wasmparser::StorageType {
    fn convert(self) -> StorageType {
        match self {
            wasmparser::StorageType::I8 => StorageType::I8,
            wasmparser::StorageType::I16 => StorageType::I16,
            wasmparser::StorageType::Val(x) => StorageType::Val(x.convert()),
        }
    }
}

impl<X: Convert<T>, T> Convert<Vec<T>> for Vec<X> {
    fn convert(self) -> Vec<T> {
        self.into_iter().map(Convert::convert).collect()
    }
}

impl Convert<Type> for wasmparser::Type {
    fn convert(self) -> Type {
        match self {
            wasmparser::Type::Func(ft) => Type::Func(ft.convert()),
            wasmparser::Type::Array(t) => Type::Array(t.element_type.convert(), t.mutable),
        }
    }
}

impl Convert<EntityType> for wasmparser::TypeRef {
    fn convert(self) -> EntityType {
        match self {
            wasmparser::TypeRef::Func(f) => EntityType::Function(f),
            wasmparser::TypeRef::Table(t) => EntityType::Table(TableType {
                element_type: t.element_type.convert(),
                minimum: t.initial,
                maximum: t.maximum,
            }),
            wasmparser::TypeRef::Memory(m) => EntityType::Memory(m.convert()),
            wasmparser::TypeRef::Global(g) => EntityType::Global(GlobalType {
                val_type: g.content_type.convert(),
                mutable: g.mutable,
            }),
            wasmparser::TypeRef::Tag(t) => EntityType::Tag(TagType {
                kind: match t.kind {
                    wasmparser::TagKind::Exception => TagKind::Exception,
                },
                func_type_idx: t.func_type_idx,
            }),
        }
    }
}

impl<'a> Convert<Import> for wasmparser::Import<'a> {
    fn convert(self) -> Import {
        Import {
            module: self.module.to_string(),
            name: self.name.to_string(),
            ty: self.ty.convert(),
        }
    }
}

impl Convert<BlockType> for wasmparser::BlockType {
    fn convert(self) -> BlockType {
        match self {
            wasmparser::BlockType::Empty => BlockType::Empty,
            wasmparser::BlockType::Type(t) => BlockType::Result(t.convert()),
            wasmparser::BlockType::FuncType(t) => BlockType::FunctionType(t),
        }
    }
}

impl Convert<MemArg> for wasmparser::MemArg {
    fn convert(self) -> MemArg {
        MemArg {
            offset: self.offset,
            align: self.align as u32,
            memory_index: self.memory,
        }
    }
}

impl Convert<MemoryType> for wasmparser::MemoryType {
    fn convert(self) -> MemoryType {
        MemoryType {
            minimum: self.initial,
            maximum: self.maximum,
            memory64: self.memory64,
            shared: self.shared,
        }
    }
}
