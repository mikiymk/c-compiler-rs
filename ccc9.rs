fn main() -> Result<(), ()> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    eprintln!("引数の個数が正しくありません。");
    return Err(());
  }

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");
  println!("  mov rax, {}", args[1].parse::<i32>().unwrap());
  println!("  ret");
  return Ok(());
}
