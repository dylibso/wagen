pub use wasm_encoder::{ConstExpr, Instruction as Instr, ValType};

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

#[derive(Clone)]
pub struct Function<'a> {
    pub name: String,
    pub body: Vec<Instr<'a>>,
    pub locals: Vec<ValType>,
    pub type_index: TypeIndex,
    pub index: FunctionIndex,
    pub export: Option<String>,
}

pub type GlobalIndex = u32;
pub type TypeIndex = u32;
pub type FunctionIndex = u32;

pub trait Expr<'a> {
    fn expr(&self) -> Vec<Instr<'a>>;
}

impl<'a> Expr<'a> for Instr<'a> {
    fn expr(&self) -> Vec<Instr<'a>> {
        vec![self.clone()]
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
            body: vec![],
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

            for instr in &def.body {
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
    pub fn instr(&mut self, instr: Instr<'a>) -> &mut Self {
        self.body.push(instr);
        self
    }

    pub fn expr(&mut self, expr: &impl Expr<'a>) -> &mut Self {
        self.body.extend(expr.expr());
        self
    }

    pub fn export(&mut self, name: impl Into<String>) -> &mut Self {
        self.export = Some(name.into());
        self
    }

    pub fn instrs(&mut self, instrs: impl IntoIterator<Item = Instr<'a>>) -> &mut Self {
        for instr in instrs.into_iter() {
            self.body.push(instr);
        }
        self
    }

    pub fn update_body(
        &mut self,
        f: impl Fn(&mut Vec<Instr<'a>>) -> anyhow::Result<()>,
    ) -> anyhow::Result<&mut Self> {
        f(&mut self.body)?;
        Ok(self)
    }

    pub fn index(&self) -> FunctionIndex {
        self.index
    }
}

struct Add1(GlobalIndex);

impl<'a> Expr<'a> for Add1 {
    fn expr(&self) -> Vec<Instr<'a>> {
        vec![
            Instr::LocalGet(0),
            Instr::GlobalGet(self.0),
            Instr::I32Add,
            Instr::Return,
        ]
    }
}

fn main() {
    let mut module = Module::new();

    // let mem_alloc = module
    //     .func("mem_alloc", [ValType::I32], [ValType::I32], [])
    //     .instrs([])
    //     .export("mem_alloc")
    //     .index();

    let aaa = module
        .global("one", ValType::I32, false, &ConstExpr::i32_const(1))
        .index();
    module.import("aaa", "bbb", Some("bbb"), [], []);
    module.import("aaa", "ccc", Some("ccc"), [], []);

    let add1 = module
        .func("add1", [ValType::I32], [ValType::I32], [])
        .expr(&Add1(aaa))
        .export("testing")
        .index();

    module
        .func("add2", [ValType::I32], [ValType::I32], [])
        .instrs([
            Instr::LocalGet(0),
            Instr::Call(add1),
            Instr::Call(add1),
            Instr::Return,
        ])
        .export("testing1");

    module.save("test.wasm").unwrap();
}
