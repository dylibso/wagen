use wagen::*;

struct Init(TypeIndex);
struct Sum(TypeIndex);

impl<'a> Expr<'a> for Init {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            Instr::LocalGet(0),
            Instr::LocalGet(1),
            Instr::ArrayNewFixed(self.0, 2),
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
            Instr::ArrayGet(self.0),
            Instr::LocalGet(0),
            Instr::RefAsNonNull,
            Instr::I32Const(1),
            Instr::ArrayGet(self.0),
            Instr::I32Add,
            Instr::Return,
        ]);
    }
}

fn main() -> anyhow::Result<()> {
    let mut module = Module::new();
    let idx = module
        .types()
        .add(|x| x.array(&StorageType::Val(ValType::I32), true));
    let _sum = module
        .func(
            "sum",
            [ValType::Ref(RefType {
                nullable: false,
                heap_type: HeapType::Concrete(idx),
            })],
            [ValType::I32],
            [ValType::I32],
        )
        .push(Sum(idx))
        .export("sum")
        .index();

    module
        .func(
            "init",
            [ValType::I32, ValType::I32],
            [ValType::Ref(RefType {
                nullable: false,
                heap_type: HeapType::Concrete(idx),
            })],
            [],
        )
        .push(Init(idx))
        .export("init");

    module.save("sum.wasm")?;
    Ok(())
}
