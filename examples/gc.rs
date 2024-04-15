use wagen::*;

struct Init(Param, Param, StructTypeIndex);
struct Sum(Param, StructTypeIndex);
struct Add(Param, Param, StructTypeIndex);

impl<'a> Expr<'a> for Init {
    fn expr(self, builder: &mut Builder<'a>) {
        builder
            .push(self.0)
            .push(self.1)
            .push(self.2.struct_new())
            .return_()
    }
}

impl<'a> Expr<'a> for Sum {
    fn expr(self, builder: &mut Builder<'a>) {
        builder
            .push(self.0)
            .push(self.1.struct_get(0))
            .push(self.0)
            .push(self.1.struct_get(1))
            .push(Instr::I32Add)
            .return_()
    }
}

impl<'a> Expr<'a> for Add {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([
            // Set a field
            Instr::LocalGet(self.0.index()),
            Instr::LocalGet(self.0.index()),
            Instr::StructGet {
                struct_type_index: self.2.index(),
                field_index: 0,
            },
            Instr::LocalGet(self.1.index()),
            Instr::StructGet {
                struct_type_index: self.2.index(),
                field_index: 0,
            },
            Instr::I32Add,
            Instr::StructSet {
                struct_type_index: self.2.index(),
                field_index: 0,
            },
            // Set b field
            Instr::LocalGet(self.0.index()),
            Instr::LocalGet(self.0.index()),
            Instr::StructGet {
                struct_type_index: self.0.index(),
                field_index: 1,
            },
            Instr::LocalGet(self.1.index()),
            Instr::StructGet {
                struct_type_index: self.0.index(),
                field_index: 1,
            },
            Instr::I32Add,
            Instr::StructSet {
                struct_type_index: self.0.index(),
                field_index: 1,
            },
            Instr::Return,
        ]);
    }
}

fn main() -> anyhow::Result<()> {
    let mut module = Module::new();
    let field = field_type(StorageType::Val(ValType::I32), true);
    let idx = module.struct_type([field, field]);
    let t = ValType::Ref(RefType {
        nullable: false,
        heap_type: HeapType::Concrete(idx.index()),
    });

    let mut locals = TypeList::new();
    let a = locals.push(t);
    let b = locals.push(t);
    module
        .func("add", locals, [], [])
        .push(Add(a, b, idx))
        .export("add");

    let mut locals = TypeList::new();
    let a = locals.push(t);
    module
        .func("sum", locals, [ValType::I32], [])
        .push(Sum(a, idx))
        .export("sum")
        .index();

    let mut locals = TypeList::new();
    let a = locals.push(ValType::I32);
    let b = locals.push(ValType::I32);
    module
        .func("init", locals, [t], [])
        .push(Init(a, b, idx))
        .export("init");

    module.save("gc.wasm")?;
    Ok(())
}
