use wagen::*;

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

fn main() -> anyhow::Result<()> {
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
