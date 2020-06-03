mod ccc;

fn main() -> Result<(), ()> {
    use ccc::code_generate;
    use ccc::parse;
    use std::env::args;

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return Err(());
    }

    let program = &args[1];

    match parse(program) {
        Ok(parsed) => {
            code_generate(&parsed);
            Ok(())
        }
        _ => Err(()),
    }
}
