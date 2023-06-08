pub mod link;

pub use wasm_encoder::{BlockType, ConstExpr, Instruction as Instr, MemArg, MemoryType, ValType};

#[cfg(feature = "extism")]
pub use extism;

#[cfg(feature = "wasmtime")]
pub use wasmtime;

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

#[derive(Default, Clone)]
pub struct Builder<'a> {
    pub instrs: Vec<Instr<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new(init: impl IntoIterator<Item = Instr<'a>>) -> Self {
        Builder {
            instrs: init.into_iter().collect(),
        }
    }

    pub fn push(&mut self, x: impl Expr<'a>) -> &mut Self {
        self.instrs.extend(x.expr().instrs);
        self
    }

    pub fn local_incr(&mut self, index: u32, ty: ValType, tee: bool) -> &mut Self {
        self.push(Instr::LocalGet(index));
        match ty {
            ValType::I64 => {
                self.push([Instr::I64Const(1), Instr::I64Add]);
            }
            ValType::I32 => {
                self.push([Instr::I32Const(1), Instr::I32Add]);
            }
            ValType::F64 => {
                self.push([Instr::F64Const(1.0), Instr::F64Add]);
            }
            ValType::F32 => {
                self.push([Instr::F32Const(1.0), Instr::F32Add]);
            }
            x => panic!("Invalid type in `local_incr`: {x:?}"),
        }

        if tee {
            self.push(Instr::LocalTee(index));
        } else {
            self.push(Instr::LocalSet(index));
        }
        self
    }

    pub fn block<F: Fn(&mut Self)>(&mut self, bt: BlockType, expr: F) -> &mut Self {
        self.push(Instr::Block(bt));
        expr(self);
        self.push(Instr::End)
    }

    pub fn r#loop<F: Fn(&mut Self)>(&mut self, bt: BlockType, expr: F) -> &mut Self {
        self.push(Instr::Loop(bt));
        expr(self);
        self.push(Instr::End)
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

impl<'a> From<Vec<Instr<'a>>> for Builder<'a> {
    fn from(instrs: Vec<Instr<'a>>) -> Self {
        Builder { instrs }
    }
}

pub trait Expr<'a> {
    fn expr(self) -> Builder<'a>;
}

impl<'a, F: Fn() -> Builder<'a>> Expr<'a> for F {
    fn expr(self) -> Builder<'a> {
        self()
    }
}

impl<'a> Expr<'a> for Vec<Instr<'a>> {
    fn expr(self) -> Builder<'a> {
        self.into()
    }
}

impl<'a> Expr<'a> for &[Instr<'a>] {
    fn expr(self) -> Builder<'a> {
        self.to_vec().into()
    }
}

impl<'a, const N: usize> Expr<'a> for [Instr<'a>; N] {
    fn expr(self) -> Builder<'a> {
        self.to_vec().into()
    }
}

impl<'a> Expr<'a> for Instr<'a> {
    fn expr(self) -> Builder<'a> {
        Builder::new([self])
    }
}

impl<'a> Expr<'a> for Builder<'a> {
    fn expr(self) -> Builder<'a> {
        self
    }
}

#[derive(Clone)]
pub struct Global {
    index: u32,
    export: Option<String>,
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
        self.import_info.push((
            func_name.unwrap_or(name.as_ref()).to_string(),
            self.imports.len() - 1,
        ));
        // self.funcs.function(type_index);
        // let index = self.funcs.len() - 1;
        self.imports.len() - 1
    }

    pub fn start(&mut self, f: FunctionIndex) -> &mut Self {
        self.start = Some(f);
        self
    }

    pub fn func(
        &mut self,
        name: impl AsRef<str>,
        params: impl IntoIterator<Item = ValType>,
        results: impl IntoIterator<Item = ValType>,
        locals: impl IntoIterator<Item = ValType>,
    ) -> &mut Function<'a> {
        let params = params.into_iter().collect::<Vec<_>>();
        let results = results.into_iter().collect::<Vec<_>>();
        self.types.function(params.clone(), results.clone());
        let type_index = self.types.len() - 1;
        self.funcs.function(type_index);
        let index = self.imports.len() + self.funcs.len() - 1;
        // self.func_names.append(index, name.as_ref());
        let f = Function {
            body: Builder::default(),
            name: name.as_ref().to_string(),
            locals: locals.into_iter().collect(),
            type_index,
            index,
            export: None,
        };
        self.defs.push(f);
        self.defs.last_mut().unwrap()
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
        extism::Manifest::new([extism::manifest::Wasm::Data {
            data,
            meta: extism::manifest::WasmMetadata {
                name: Some("main".to_string()),
                hash: None,
            },
        }])
    }

    pub fn into_extism_plugin<'b>(
        self,
        functions: impl IntoIterator<Item = extism::Function>,
        wasi: bool,
    ) -> anyhow::Result<extism::Plugin<'b>> {
        let manifest = self.into_extism_manifest();
        extism::Plugin::create_with_manifest(&manifest, functions, wasi)
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
