mod ccc;

fn main() -> Result<(), ()> {
    use ccc::code_gen::code_generate;
    use ccc::parse::parse;
    use ccc::parse::token::tokenize;
    use std::env::args;

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return Err(());
    }

    let program = &args[1];

    match tokenize(&program) {
        Ok(mut token) => {
            match parse(&mut token) {
                Ok(node) => {
                    // eprintln!("{:?}", node);
                    code_generate(&node);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    Err(())
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            Err(())
        }
    }
}
