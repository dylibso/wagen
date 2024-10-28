mod builder;
mod expr;
mod function;
mod index;
pub mod link;
mod type_list;

pub use builder::Builder;
pub use expr::Expr;
pub use function::Function;
pub use index::{FunctionIndex, Index};
pub use type_list::{Local, Param, TypeList};

pub use wasm_encoder::{
    self as encoder, BlockType, ConstExpr, ElementMode, ElementSection, ElementSegment, Elements,
    FieldType, HeapType, Instruction as Instr, MemArg, MemoryType, RefType, StorageType, TableType,
    ValType,
};

pub use wasmparser as parser;

handle!(GlobalHandle);
pub type GlobalIndex = Index<GlobalHandle>;

handle!(DataSegmentHandle);
pub type DataSegmentIndex = Index<GlobalHandle>;

handle!(MemoryHandle);
pub type MemoryIndex = Index<MemoryHandle>;

handle!(FunctionTypeHandle);
pub type FunctionTypeIndex = Index<FunctionTypeHandle>;

impl FunctionTypeIndex {
    pub fn ref_type(self, nullable: bool) -> RefType {
        RefType {
            nullable,
            heap_type: HeapType::Concrete(self.index()),
        }
    }

    pub fn val_type(self, nullable: bool) -> ValType {
        ValType::Ref(self.ref_type(nullable))
    }
}

handle!(ArrayTypeHandle);
pub type ArrayTypeIndex = Index<ArrayTypeHandle>;

impl ArrayTypeIndex {
    pub fn ref_type(self, nullable: bool) -> RefType {
        RefType {
            nullable,
            heap_type: HeapType::Concrete(self.index()),
        }
    }

    pub fn val_type(self, nullable: bool) -> ValType {
        ValType::Ref(self.ref_type(nullable))
    }

    pub fn array_new<'a>(&self) -> impl Expr<'a> {
        Instr::ArrayNew(self.index())
    }

    pub fn array_new_default<'a>(&self) -> impl Expr<'a> {
        Instr::ArrayNewDefault(self.index())
    }

    pub fn array_get<'a>(&self) -> impl Expr<'a> {
        Instr::ArrayGet(self.index())
    }

    pub fn array_get_s<'a>(&self) -> impl Expr<'a> {
        Instr::ArrayGetS(self.index())
    }

    pub fn array_get_u<'a>(&self) -> impl Expr<'a> {
        Instr::ArrayGetU(self.index())
    }

    pub fn array_set<'a>(&self) -> impl Expr<'a> {
        Instr::ArraySet(self.index())
    }
}

handle!(StructTypeHandle);
pub type StructTypeIndex = Index<StructTypeHandle>;

impl StructTypeIndex {
    pub fn ref_type(self, nullable: bool) -> RefType {
        RefType {
            nullable,
            heap_type: HeapType::Concrete(self.index()),
        }
    }

    pub fn val_type(self, nullable: bool) -> ValType {
        ValType::Ref(self.ref_type(nullable))
    }

    pub fn struct_new<'a>(&self) -> impl Expr<'a> {
        Instr::StructNew(self.index())
    }

    pub fn struct_new_default<'a>(&self) -> impl Expr<'a> {
        Instr::StructNewDefault(self.index())
    }

    pub fn struct_get<'a>(&self, field: u32) -> impl Expr<'a> {
        Instr::StructGet {
            struct_type_index: self.index(),
            field_index: field,
        }
    }

    pub fn struct_get_s<'a>(&self, field: u32) -> impl Expr<'a> {
        Instr::StructGetS {
            struct_type_index: self.index(),
            field_index: field,
        }
    }

    pub fn struct_get_u<'a>(&self, field: u32) -> impl Expr<'a> {
        Instr::StructGetU {
            struct_type_index: self.index(),
            field_index: field,
        }
    }

    pub fn struct_set<'a>(&self, field: u32) -> impl Expr<'a> {
        Instr::StructSet {
            struct_type_index: self.index(),
            field_index: field,
        }
    }
}

#[derive(Clone, Default)]
pub struct Module<'a> {
    types: wasm_encoder::TypeSection,
    globals: wasm_encoder::GlobalSection,
    funcs: wasm_encoder::FunctionSection,
    func_names: wasm_encoder::NameMap,
    global_names: wasm_encoder::NameMap,
    code: wasm_encoder::CodeSection,
    names: wasm_encoder::NameSection,
    exports: wasm_encoder::ExportSection,
    data: wasm_encoder::DataSection,
    memory: wasm_encoder::MemorySection,
    import_info: Vec<(String, u32)>,
    defs: Vec<Function<'a>>,
    global_defs: Vec<Global>,
    start: Option<FunctionIndex>,
    imports: wasm_encoder::ImportSection,
    tables: wasm_encoder::TableSection,
    elements: wasm_encoder::ElementSection,
}

pub struct Types<'a>(pub &'a mut wasm_encoder::TypeSection);

impl<'a> Types<'a> {
    pub fn push<F: FnOnce(wasm_encoder::CoreTypeEncoder)>(&mut self, f: F) -> u32 {
        f(self.0.ty());
        self.0.len() - 1
    }
}

pub struct Tables<'a>(pub &'a mut wasm_encoder::TableSection);

impl<'a> Tables<'a> {
    pub fn push(&mut self, ty: TableType) -> u32 {
        self.0.table(ty);
        self.0.len() - 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    index: u32,
    export: Option<String>,
}

impl<'a> Expr<'a> for Global {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::GlobalGet(self.index));
    }
}

impl Global {
    pub fn export(&mut self, name: impl Into<String>) -> &mut Self {
        self.export = Some(name.into());
        self
    }

    pub fn index(&self) -> GlobalIndex {
        GlobalIndex::from(self.index)
    }

    pub fn set<'a>(&self) -> impl Expr<'a> {
        Instr::GlobalSet(self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub fields: Vec<FieldType>,
}

impl<T: IntoIterator<Item = FieldType>> From<T> for StructType {
    fn from(value: T) -> Self {
        StructType {
            fields: value.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub item: StorageType,
    pub mutable: bool,
}

impl<'a> Module<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn global(
        &mut self,
        name: impl AsRef<str>,
        ty: ValType,
        mutable: bool,
        shared: bool,
        init: &ConstExpr,
    ) -> &mut Global {
        self.globals.global(
            wasm_encoder::GlobalType {
                shared,
                val_type: ty,
                mutable,
            },
            &init,
        );
        let index = self.globals.len() - 1;
        self.global_names.append(index, name.as_ref());
        self.global_defs.push(Global {
            index,
            export: None,
        });
        self.global_defs.last_mut().unwrap()
    }

    pub fn import(
        &mut self,
        module: impl AsRef<str>,
        name: impl AsRef<str>,
        func_name: Option<&str>,
        params: impl IntoIterator<Item = ValType>,
        results: impl IntoIterator<Item = ValType>,
    ) -> FunctionIndex {
        let params = params.into_iter().collect::<Vec<_>>();
        let results = results.into_iter().collect::<Vec<_>>();
        self.types.ty().function(params.clone(), results.clone());
        let type_index = self.types.len() - 1;
        self.imports.import(
            module.as_ref(),
            name.as_ref(),
            wasm_encoder::EntityType::Function(type_index),
        );
        let idx = self.imports.len() - 1;
        self.import_info
            .push((func_name.unwrap_or(name.as_ref()).to_string(), idx));
        FunctionIndex::from(idx)
    }

    pub fn start(&mut self, f: FunctionIndex) -> &mut Self {
        self.start = Some(f);
        self
    }

    pub fn func(
        &mut self,
        name: impl AsRef<str>,
        params: impl Into<TypeList<Param>>,
        results: impl IntoIterator<Item = ValType>,
        locals: impl Into<TypeList<Local>>,
    ) -> &mut Function<'a> {
        let params = params.into().items.into_values().collect::<Vec<_>>();
        let results = results.into_iter().collect::<Vec<_>>();
        let type_index = self
            .types()
            .push(|t| t.function(params.clone(), results.clone()));
        self.funcs.function(type_index.into());
        let index = self.imports.len() + self.funcs.len() - 1;
        let f = Function {
            body: Builder::default(),
            name: name.as_ref().to_string(),
            locals: locals.into().items.into_values().collect(),
            type_index: FunctionTypeIndex::from(type_index),
            index,
            export: None,
        };
        self.defs.push(f);
        self.defs.last_mut().unwrap()
    }

    pub fn struct_type(&mut self, def: impl Into<StructType>) -> StructTypeIndex {
        StructTypeIndex::from(self.types().push(|t| t.struct_(def.into().fields.clone())))
    }

    pub fn array_type(&mut self, def: &StructType) -> ArrayTypeIndex {
        ArrayTypeIndex::from(self.types().push(|t| t.struct_(def.fields.clone())))
    }

    pub fn types<'b>(&'b mut self) -> Types<'b> {
        Types(&mut self.types)
    }

    pub fn tables<'b>(&'b mut self) -> Tables<'b> {
        Tables(&mut self.tables)
    }

    pub fn active_element(
        &mut self,
        table_index: Option<u32>,
        elements: Elements,
    ) -> &mut ElementSection {
        self.elements
            .active(table_index, &ConstExpr::i32_const(0), elements)
    }

    pub fn passive_element(&mut self, elements: Elements) -> &mut ElementSection {
        self.elements.passive(elements)
    }

    pub fn element_segment(&mut self, seg: ElementSegment) -> &mut ElementSection {
        self.elements.segment(seg)
    }

    pub fn finish(mut self) -> Vec<u8> {
        let mut module = wasm_encoder::Module::new();
        for i in self.import_info {
            self.func_names.append(i.1, &i.0);
        }

        for def in self.defs {
            let mut f = wasm_encoder::Function::new_with_locals_types(def.locals);

            for instr in &def.body.instrs {
                f.instruction(instr);
            }
            f.instruction(&Instr::End);
            self.code.function(&f);
            self.func_names.append(def.index, &def.name);

            if let Some(name) = def.export {
                self.exports
                    .export(&name, wasm_encoder::ExportKind::Func, def.index);
            }
        }

        for g in self.global_defs {
            if let Some(name) = g.export {
                self.exports
                    .export(&name, wasm_encoder::ExportKind::Global, g.index);
            }
        }

        module.section(&self.types);
        module.section(&self.imports);
        module.section(&self.funcs);
        module.section(&self.tables);
        module.section(&self.memory);
        module.section(&self.globals);
        module.section(&self.exports);

        if let Some(start) = self.start {
            module.section(&wasm_encoder::StartSection {
                function_index: start.0,
            });
        }

        module.section(&self.elements);
        module.section(&self.code);
        module.section(&self.data);

        // Set names
        self.names.functions(&self.func_names);
        self.names.globals(&self.global_names);
        module.section(&self.names);

        // Finish
        module.finish()
    }

    pub fn data_segment(&mut self, offset: &ConstExpr, data: impl AsRef<[u8]>) -> DataSegmentIndex {
        self.data.active(0, offset, data.as_ref().to_vec());
        DataSegmentIndex::from(self.data.len() - 1)
    }

    pub fn memory(&mut self, mt: MemoryType) -> MemoryIndex {
        self.memory.memory(mt);
        MemoryIndex::from(self.memory.len() - 1)
    }

    pub fn save(self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let bytes = self.finish();
        std::fs::write(path, bytes)?;
        Ok(())
    }

    pub fn validate(self) -> anyhow::Result<Vec<u8>> {
        let bytes = self.finish();
        validate(&bytes)?;
        Ok(bytes)
    }

    pub fn validate_save(self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let bytes = self.finish();
        validate(&bytes)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

pub fn validate(
    data: impl AsRef<[u8]>,
) -> Result<wasmparser::types::Types, wasmparser::BinaryReaderError> {
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(data.as_ref())
}

#[cfg(feature = "extism")]
impl<'a> Module<'a> {
    pub fn into_extism_manifest(self) -> extism::Manifest {
        let data = self.finish();
        extism::Manifest::new([extism::Wasm::data(data)])
    }

    pub fn into_extism_plugin(
        self,
        functions: impl IntoIterator<Item = extism::Function>,
        wasi: bool,
    ) -> anyhow::Result<extism::Plugin> {
        let manifest = self.into_extism_manifest();
        extism::Plugin::new(&manifest, functions, wasi)
    }
}

#[cfg(feature = "wasmtime")]
impl<'a> Module<'a> {
    pub fn into_wasmtime_instance(
        self,
        config: Option<wasmtime::Config>,
    ) -> anyhow::Result<(wasmtime::Store<()>, wasmtime::Instance)> {
        let data = self.finish();
        let config = config.unwrap_or_default();
        let engine = wasmtime::Engine::new(&config)?;
        let module = wasmtime::Module::new(&engine, data)?;
        let linker = wasmtime::Linker::new(&engine);
        let mut store = wasmtime::Store::new(&engine, ());
        let instance = linker.instantiate(&mut store, &module)?;
        Ok((store, instance))
    }
}

pub fn any_type(nullable: bool) -> ValType {
    ValType::Ref(RefType {
        nullable,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: encoder::AbstractHeapType::Any,
        },
    })
}

pub fn none_type(nullable: bool) -> ValType {
    ValType::Ref(RefType {
        nullable,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: encoder::AbstractHeapType::None,
        },
    })
}

pub fn i31_type(nullable: bool) -> ValType {
    ValType::Ref(RefType {
        nullable,
        heap_type: HeapType::I31,
    })
}

pub fn eq_type(nullable: bool) -> ValType {
    ValType::Ref(RefType {
        nullable,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: encoder::AbstractHeapType::Eq,
        },
    })
}

pub fn func_type(nullable: bool) -> ValType {
    ValType::Ref(RefType {
        nullable,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: encoder::AbstractHeapType::Func,
        },
    })
}

pub fn extern_type(nullable: bool) -> ValType {
    ValType::Ref(RefType {
        nullable,
        heap_type: HeapType::Abstract {
            shared: false,
            ty: encoder::AbstractHeapType::Extern,
        },
    })
}

pub fn field_type(storage: StorageType, mutable: bool) -> FieldType {
    FieldType {
        element_type: storage,
        mutable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_valid_module() {
        // From examples/add1.rs
        struct Add1(Param);
        impl<'a> Expr<'a> for Add1 {
            fn expr(self, builder: &mut Builder<'a>) {
                builder
                    .push(self.0)
                    .push(1i32)
                    .push(Instr::I32Add)
                    .return_()
            }
        }
        let mut module = Module::new();
        let mut params = TypeList::new();
        let a = params.push(ValType::I32);
        let add1 = module
            .func("add1", params, [ValType::I32], [])
            .push(Add1(a))
            .export("add1")
            .index();
        let mut params = TypeList::new();
        let a = params.push(ValType::I32);
        module
            .func("add2", params, [ValType::I32], [])
            .push(a)
            .push(add1)
            .push(add1)
            .push(Instr::Return)
            .export("add2");
        assert!(module.validate().is_ok());
    }

    #[test]
    fn generate_empty_module() {
        let module = Module::new();
        assert!(module.validate().is_ok());
    }
}
