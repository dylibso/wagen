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
            [ValType::I64, ValType::I32],
        )
        .with_builder(|b| {
            b.r#loop(BlockType::Empty, |b| {
                b.push([
                    // Load current index
                    Instr::LocalGet(0),
                    Instr::Call(extism.input_load_u8),
                    // Check if the result is a vowel and store it in `1`
                    Instr::Call(is_vowel),
                    Instr::LocalGet(1),
                    Instr::I32Add,
                    Instr::LocalSet(1),
                ])
                .local_incr(0, ValType::I64, true)
                .push([
                    // Check index variable
                    Instr::Call(extism.input_length),
                    Instr::I64LeU,
                    Instr::BrIf(0),
                ]);
            })
            .push(Instr::LocalGet(1));
        })
        .export("count_vowels");

    module.save("test.wasm").unwrap();
}
