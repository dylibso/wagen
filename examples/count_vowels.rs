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

fn is_vowel<'a>(builder: &mut Builder<'a>) {
    builder
        .push(check_vowel('a'))
        .push(check_vowel('e'))
        .push(check_vowel('i'))
        .push(check_vowel('o'))
        .push(check_vowel('u'))
        .push([Instr::I32Or, Instr::I32Or, Instr::I32Or, Instr::I32Or]);
}

fn main() {
    let mut module = Module::new();
    let extism = module.link_extism();

    let is_vowel = module
        .func("is_vowel", [ValType::I32], [ValType::I32], [])
        .push(is_vowel)
        .index();
    let mut locals = TypeList::<Local>::new();
    let index = locals.push(ValType::I64);
    let count = locals.push(ValType::I32);
    let pointer = locals.push(ValType::I64);
    module
        .func("count_vowels", [], [ValType::I32], locals)
        .with_builder(|b| {
            b.loop_(BlockType::Empty, |b: &mut Builder| {
                b.push(index)
                    .push(extism.input_load_u8)
                    .push(is_vowel)
                    .push(count)
                    .push(Instr::I32Add)
                    .push(count.set())
                    .local_incr(index, ValType::I64)
                    .push(index)
                    .push(extism.input_length)
                    .push([Instr::I64LeU, Instr::BrIf(0)]);
            })
            .push(std::mem::size_of::<u64>() as i64)
            .push(extism.alloc)
            .push(Instr::LocalTee(pointer.into()))
            .push(count)
            .push(Instr::I64ExtendI32U)
            .push(extism.store_u64)
            .push(pointer)
            .push(std::mem::size_of::<u64>() as i64)
            .push(extism.output_set)
            .push(0i32)
            .return_()
        })
        .export("count_vowels");

    module.clone().save("count_vowels.wasm").unwrap();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let data: i64 = module
        .into_extism_plugin([], false)
        .unwrap()
        .call(
            "count_vowels",
            args.first().map(|x| x.as_str()).unwrap_or("this is a test"),
        )
        .unwrap();
    println!("{}", data);
}
