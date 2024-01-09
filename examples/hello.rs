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

    let dataidx = 0x13;
    let s = "Hello, world!";
    let len = s.len();
    module.data_segment(&ConstExpr::i32_const(dataidx), s);

    let mut locals = TypeList::new();
    let a = locals.push(ValType::I64);
    module
        .func("hello", [], [ValType::I32], locals)
        .with_builder(|b| {
            b.push(len as i64)
                .push(extism.alloc)
                .push(a.tee())
                .push(0i32)
                .push(Instr::I64Load(MemArg {
                    offset: dataidx as u64,
                    align: 1,
                    memory_index: 0,
                }))
                .push(extism.store_u64)
                .push(a)
                .push(8i64)
                .push(Instr::I64Add)
                .push(0i32)
                .push(Instr::I64Load(MemArg {
                    offset: dataidx as u64 + 8,
                    align: 1,
                    memory_index: 0,
                }))
                .push(extism.store_u64)
                .push(a)
                .push(len as i64)
                .push(extism.output_set)
                .push(0i32)
                .return_()
        })
        .export("hello");

    module.clone().save("hello.wasm").unwrap();

    let mut plugin = module.into_extism_plugin([], false).unwrap();
    let data: String = plugin.call("hello", "").unwrap();
    println!("{}", data);
}
