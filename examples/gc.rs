use wagen::*;

struct Init(TypeIndex);
struct Sum(TypeIndex);

impl<'a> Expr<'a> for Init {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            Instr::LocalGet(0),
            Instr::LocalGet(1),
            Instr::StructNew(self.0),
            Instr::Return,
        ]);
    }
}

impl<'a> Expr<'a> for Sum {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            Instr::LocalGet(0),
            Instr::StructGet(self.0, 0),
            Instr::LocalGet(0),
            Instr::StructGet(self.0, 1),
            Instr::I32Add,
            Instr::Return,
        ]);
    }
}

fn main() -> anyhow::Result<()> {
    let mut module = Module::new();
    let field = FieldType {
        element_type: StorageType::Val(ValType::I32),
        mutable: true,
    };
    let idx = module.types().add(|x| x.struct_([field, field]));
    let t = ValType::Ref(RefType {
        nullable: false,
        heap_type: HeapType::Concrete(idx),
    });
    let _sum = module
        .func("sum", [t], [ValType::I32], [])
        .push(Sum(idx))
        .export("sum")
        .index();

    module
        .func("init", [ValType::I32, ValType::I32], [t], [])
        .push(Init(idx))
        .export("init");

    module.save("gc.wasm")?;
    Ok(())
}
