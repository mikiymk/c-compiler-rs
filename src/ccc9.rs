mod ccc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return;
    }

    let program = &args[1];
    match ccc::compile(program) {
        Ok(()) => eprintln!("Success"),
        Err(e) => eprintln!("Error {:?}", e),
    }
}
