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

    if let Some(mut token) = token::tokenize(&program) {
        let node = parse::node(&mut token);
        assemble::assemble(&node);
    }
}