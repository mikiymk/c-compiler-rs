mod ccc;

fn main() -> Result<(), ()> {
    use std::env::args;
    use ccc::token::tokenize;
    use ccc::parse::node;
    use ccc::code_gen::code_generate;

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return Err(());
    }

    let program = &args[1];

    match tokenize(&program) {
        Ok(mut token) => {
            // eprintln!("{:?}", token);
            match node(&mut token) {
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