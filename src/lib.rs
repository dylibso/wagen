mod builder;
mod expr;
pub mod link;

pub use builder::Builder;
pub use expr::Expr;

pub use wasm_encoder::{
    self as encoder, BlockType, ConstExpr, FieldType, HeapType, Instruction as Instr, MemArg,
    MemoryType, RefType, StorageType, ValType,
};

pub use wasmparser as parser;

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
}

pub struct Types<'a>(pub &'a mut wasm_encoder::TypeSection);

impl<'a> Types<'a> {
    pub fn add<F: FnOnce(&mut wasm_encoder::TypeSection) -> &mut wasm_encoder::TypeSection>(
        &mut self,
        f: F,
    ) -> TypeIndex {
        f(&mut self.0);
        self.0.len() - 1
    }
}

#[derive(Clone)]
pub struct Function<'a> {
    pub name: String,
    pub body: Builder<'a>,
    pub locals: Vec<ValType>,
    pub type_index: TypeIndex,
    pub index: FunctionIndex,
    pub export: Option<String>,
}

pub type GlobalIndex = u32;
pub type TypeIndex = u32;
pub type FunctionIndex = u32;
pub type DataSegmentIndex = u32;
pub type MemoryIndex = u32;

#[derive(Clone)]
pub struct Global {
    index: u32,
    export: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Local(pub u32);

impl Local {
    pub fn index(&self) -> u32 {
        self.0
    }
}

impl From<Local> for u32 {
    fn from(value: Local) -> Self {
        value.0
    }
}

#[derive(Clone, Default, Debug)]
pub struct Locals {
    next: u32,
    map: std::collections::BTreeMap<u32, (String, ValType)>,
}

impl<T: IntoIterator<Item = ValType>> From<T> for Locals {
    fn from(value: T) -> Self {
        let mut dest = Locals::new();
        for x in value.into_iter() {
            dest.add("", x);
        }
        dest
    }
}

impl Locals {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: impl Into<String>, ty: ValType) -> Local {
        let n = self.next;
        self.map.insert(n, (name.into(), ty));
        self.next += 1;
        Local(n)
    }

    pub fn name(&self, local: Local) -> Option<&str> {
        self.map.get(&local.0).map(|x| x.0.as_str())
    }

    pub fn ty(&self, local: Local) -> Option<ValType> {
        self.map.get(&local.0).map(|x| x.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Param(pub u32);

impl Param {
    pub fn index(&self) -> u32 {
        self.0
    }
}

impl From<Param> for u32 {
    fn from(value: Param) -> Self {
        value.0
    }
}

#[derive(Clone, Default, Debug)]
pub struct Params {
    next: u32,
    map: std::collections::BTreeMap<u32, (String, ValType)>,
}

impl<T: IntoIterator<Item = ValType>> From<T> for Params {
    fn from(value: T) -> Self {
        let mut dest = Params::new();
        for x in value.into_iter() {
            dest.add("", x);
        }
        dest
    }
}

impl Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: impl Into<String>, ty: ValType) -> Local {
        let n = self.next;
        self.map.insert(n, (name.into(), ty));
        self.next += 1;
        Local(n)
    }

    pub fn name(&self, local: Local) -> Option<&str> {
        self.map.get(&local.0).map(|x| x.0.as_str())
    }

    pub fn ty(&self, local: Local) -> Option<ValType> {
        self.map.get(&local.0).map(|x| x.1)
    }
}

impl Global {
    pub fn export(&mut self, name: impl Into<String>) -> &mut Self {
        self.export = Some(name.into());
        self
    }

    pub fn index(&self) -> GlobalIndex {
        self.index
    }
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
        init: &ConstExpr,
    ) -> &mut Global {
        self.globals.global(
            wasm_encoder::GlobalType {
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
        self.types.function(params.clone(), results.clone());
        let type_index = self.types.len() - 1;
        self.imports.import(
            module.as_ref(),
            name.as_ref(),
            wasm_encoder::EntityType::Function(type_index),
        );
        let idx = self.imports.len() - 1;
        self.import_info
            .push((func_name.unwrap_or(name.as_ref()).to_string(), idx));
        idx
    }

    pub fn start(&mut self, f: FunctionIndex) -> &mut Self {
        self.start = Some(f);
        self
    }

    pub fn func(
        &mut self,
        name: impl AsRef<str>,
        params: impl Into<Params>,
        results: impl IntoIterator<Item = ValType>,
        locals: impl Into<Locals>,
    ) -> &mut Function<'a> {
        let params = params
            .into()
            .map
            .into_values()
            .map(|x| x.1)
            .collect::<Vec<_>>();
        let results = results.into_iter().collect::<Vec<_>>();
        let type_index = self
            .types()
            .add(|t| t.function(params.clone(), results.clone()));
        self.funcs.function(type_index);
        let index = self.imports.len() + self.funcs.len() - 1;
        let f = Function {
            body: Builder::default(),
            name: name.as_ref().to_string(),
            locals: locals.into().map.into_values().map(|x| x.1).collect(),
            type_index,
            index,
            export: None,
        };
        self.defs.push(f);
        self.defs.last_mut().unwrap()
    }

    pub fn types<'b>(&'b mut self) -> Types<'b> {
        Types(&mut self.types)
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

        let table = wasm_encoder::TableSection::new();

        module.section(&self.types);
        module.section(&self.imports);
        module.section(&self.funcs);
        module.section(&table);
        module.section(&self.memory);
        module.section(&self.globals);
        module.section(&self.exports);

        if let Some(start) = self.start {
            module.section(&wasm_encoder::StartSection {
                function_index: start,
            });
        }

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
        self.data.len() - 1
    }

    pub fn memory(&mut self, mt: MemoryType) -> MemoryIndex {
        self.memory.memory(mt);
        self.memory.len() - 1
    }

    pub fn save(self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let bytes = self.finish();
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

impl<'a> Function<'a> {
    pub fn push(&mut self, expr: impl Expr<'a>) -> &mut Self {
        self.body.push(expr);
        self
    }

    pub fn export(&mut self, name: impl Into<String>) -> &mut Self {
        self.export = Some(name.into());
        self
    }

    pub fn builder(&mut self) -> &mut Builder<'a> {
        &mut self.body
    }

    pub fn with_builder(&mut self, f: impl Fn(&mut Builder)) -> &mut Self {
        f(&mut self.body);
        self
    }

    pub fn index(&self) -> FunctionIndex {
        self.index
    }
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
