mod token;
mod parse;
mod assemble;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return;
    }

    let program = &args[1];

    match token::tokenize(&program) {
        Ok(mut token) => {
            eprintln!("{:?}", token);
            match parse::node(&mut token) {
                Ok(node) => {
                    eprintln!("{:?}", node);
                    assemble::assemble(&node)
                }
                Err(e) => {
                    eprintln!("{:?}", e)
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e)
        }
    }
}