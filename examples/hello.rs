use wagen::*;

fn main() {
    let mut module = Module::new();
    let extism = module.link_extism();

    module.memory(MemoryType {
        minimum: 1,
        maximum: None,
        memory64: false,
        shared: false,
    });

    let dataidx = 0x1;
    let s = "Hello, world!";
    let len = s.len();
    module.data_segment(&ConstExpr::i32_const(dataidx), s);

    let mut locals = TypeList::new();
    let a = locals.push(ValType::I64);
    module
        .func("hello", [], [ValType::I32], locals)
        .with_builder(|b| {
            b.push([
                //
                // Alloc
                Instr::I64Const(len as i64),
                Instr::Call(extism.alloc),
                Instr::LocalTee(a.into()),
                //
                // Load first half from memory and copy it to extism
                Instr::I32Const(0),
                Instr::I64Load(MemArg {
                    offset: 0x1,
                    align: 1,
                    memory_index: 0,
                }),
                Instr::Call(extism.store_u64),
                //
                // Load second half and copy it
                Instr::LocalGet(a.into()),
                Instr::I64Const(8),
                Instr::I64Add,
                Instr::I32Const(0),
                Instr::I64Load(MemArg {
                    offset: 0x9,
                    align: 1,
                    memory_index: 0,
                }),
                Instr::Call(extism.store_u64),
                //
                // Set output
                Instr::LocalGet(a.into()),
                Instr::I64Const(len as i64),
                Instr::Call(extism.output_set),
                Instr::I32Const(0),
            ]);
        })
        .export("hello");

    module.clone().save("hello.wasm").unwrap();

    let mut plugin = module.into_extism_plugin([], false).unwrap();
    let data: String = plugin.call("hello", "").unwrap();
    println!("{}", data);
}
