use wagen::*;

struct Add1;

impl<'a> Expr<'a> for Add1 {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            Instr::LocalGet(0),
            Instr::I32Const(1),
            Instr::I32Add,
            Instr::Return,
        ]);
    }
}

fn main() -> anyhow::Result<()> {
    let mut module = Module::new();
    let add1 = module
        .func("add1", [ValType::I32], [ValType::I32], [])
        .push(Add1)
        .export("add1")
        .index();

    let mut params = Params::new();
    let a = params.add("a", ValType::I32);
    module
        .func("add2", params, [ValType::I32], [])
        .push([
            Instr::LocalGet(a.into()),
            Instr::Call(add1),
            Instr::Call(add1),
            Instr::Return,
        ])
        .export("add2");

    let args: Vec<_> = std::env::args().skip(1).collect();
    let num = args[0].parse().unwrap_or_default();

    let (mut store, instance) = module.into_wasmtime_instance(None)?;
    let func = instance.get_func(&mut store, "add1").unwrap();
    let params = &[wasmtime::Val::I32(num)];
    let results = &mut [wasmtime::Val::I32(0)];
    func.call(&mut store, params, results)?;
    println!("{}", results[0].unwrap_i32());

    let func = instance.get_func(&mut store, "add2").unwrap();
    let params = &[wasmtime::Val::I32(num)];
    let results = &mut [wasmtime::Val::I32(0)];
    func.call(&mut store, params, results)?;
    println!("{}", results[0].unwrap_i32());
    Ok(())
}
