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

    // eprintln!("{}", program);
    if let Some(mut token) = token::tokenize(&program) {
        // eprintln!("{:?}", token);
        let node = parse::node(&mut token);
        // eprintln!("{:?}", node);
        assemble::assemble(&node);
    }
}