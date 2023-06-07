use wagen::*;

struct Add1(GlobalIndex);

impl<'a> Expr<'a> for Add1 {
    fn expr(self) -> Builder<'a> {
        Builder::new([
            Instr::LocalGet(0),
            Instr::GlobalGet(self.0),
            Instr::I32Add,
            Instr::Return,
        ])
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
        .push(Add1(aaa))
        .export("testing")
        .index();

    module
        .func("add2", [ValType::I32], [ValType::I32], [])
        .push([
            Instr::LocalGet(0),
            Instr::Call(add1),
            Instr::Call(add1),
            Instr::Return,
        ])
        .export("testing1");

    module.save("test.wasm").unwrap();
}
