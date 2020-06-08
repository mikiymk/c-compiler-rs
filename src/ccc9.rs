mod ccc;

fn main() -> Result<(), ccc::error::CompileError> {
    use ccc::code_generate;
    use ccc::error::CompileError;
    use ccc::parse;
    use std::env::args;

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return Err(CompileError::new("引数の個数が正しくありません。", 0, ""));
    }

    let program = &args[1];
    let parsed = parse(program)?;
    // eprintln!("{:?}", parsed);
    code_generate(&parsed);
    Ok(())
}
