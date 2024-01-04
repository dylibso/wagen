use wagen::*;

struct Init;
struct Sum;

impl<'a> Expr<'a> for Init {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            Instr::LocalGet(0),
            Instr::LocalGet(1),
            Instr::ArrayNewFixed(0, 2),
            Instr::Return,
        ]);
    }
}

impl<'a> Expr<'a> for Sum {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            Instr::I32Const(1),
            Instr::LocalSet(1),
            Instr::LocalGet(0),
            Instr::RefAsNonNull,
            Instr::I32Const(0),
            Instr::ArrayGet(0),
            Instr::LocalGet(0),
            Instr::RefAsNonNull,
            Instr::I32Const(1),
            Instr::ArrayGet(0),
            Instr::I32Add,
            Instr::Return,
        ]);
    }
}

fn main() -> anyhow::Result<()> {
    let mut module = Module::new();
    module.types().array(&StorageType::Val(ValType::I32), true);
    let _sum = module
        .func(
            "sum",
            [ValType::Ref(RefType {
                nullable: false,
                heap_type: HeapType::Concrete(0),
            })],
            [ValType::I32],
            [ValType::I32],
        )
        .push(Sum)
        .export("sum")
        .index();

    module
        .func(
            "init",
            [ValType::I32, ValType::I32],
            [ValType::Ref(RefType {
                nullable: false,
                heap_type: HeapType::Concrete(0),
            })],
            [],
        )
        .push(Init)
        .export("init");

    module.save("sum.wasm")?;
    Ok(())
}
