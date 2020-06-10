mod ccc;

fn main() -> Result<(), ccc::error::CompileError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return Err(ccc::error::CompileError::new(
            "引数の個数が正しくありません。",
            0,
            "",
        ));
    }

    let program = &args[1];
    let parsed = ccc::parse(program)?;
    eprintln!("{:?}", parsed);
    ccc::code_generate(&parsed);
    Ok(())
}
