use wagen::*;

fn check_vowel<'a>(ch: char) -> Builder<'a> {
    let cap = ch.to_ascii_uppercase();
    let gen = |c: char| [Instr::LocalGet(0), Instr::I32Const(c as i32), Instr::I32Eq];
    Builder::default()
        .push(gen(cap))
        .push(gen(ch))
        .push(Instr::I32Or)
        .to_owned()
}

fn is_vowel<'a>() -> Builder<'a> {
    Builder::default()
        .push(check_vowel('a'))
        .push(check_vowel('e'))
        .push(check_vowel('i'))
        .push(check_vowel('o'))
        .push(check_vowel('u'))
        .push([Instr::I32Or, Instr::I32Or, Instr::I32Or, Instr::I32Or])
        .to_owned()
}

fn main() {
    let mut module = Module::new();
    let extism = module.link_extism();

    let is_vowel = module
        .func("is_vowel", [ValType::I32], [ValType::I32], [])
        .push(is_vowel)
        .index();

    module
        .func(
            "count_vowels",
            [],
            [ValType::I32],
            [ValType::I64, ValType::I32, ValType::I64],
        )
        .with_builder(|b| {
            let index = 0;
            let count = 1;
            let pointer = 2;
            b.r#loop(BlockType::Empty, |b| {
                b.push([
                    // Load current index
                    Instr::LocalGet(index),
                    Instr::Call(extism.input_load_u8),
                    // Check if the result is a vowel and store it in `1`
                    Instr::Call(is_vowel),
                    Instr::LocalGet(count),
                    Instr::I32Add,
                    Instr::LocalSet(count),
                ])
                .local_incr(0, ValType::I64, true)
                .push([
                    // Check index variable
                    Instr::Call(extism.input_length),
                    Instr::I64LeU,
                    Instr::BrIf(0),
                ]);
            })
            .push([
                Instr::I64Const(std::mem::size_of::<u64>() as i64),
                Instr::Call(extism.alloc),
                Instr::LocalTee(pointer),
                Instr::LocalGet(count),
                Instr::I64ExtendI32U,
                Instr::Call(extism.store_u64),
                Instr::LocalGet(pointer),
                Instr::I64Const(std::mem::size_of::<u64>() as i64),
                Instr::Call(extism.output_set),
                Instr::I32Const(0),
            ]);
        })
        .export("count_vowels");

    module.clone().save("count_vowels.wasm").unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let data = module
        .into_extism_plugin([], false)
        .unwrap()
        .call(
            "count_vowels",
            args.first().map(|x| x.as_str()).unwrap_or("this is a test"),
        )
        .unwrap()
        .to_vec();
    println!("{}", i64::from_le_bytes(data.try_into().unwrap()));
}
