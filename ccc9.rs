fn main() -> Result<(), ()> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    eprintln!("引数の個数が正しくありません。");
    return Err(());
  }

  let mut re = ReadString::read(&args[1]);

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");
  println!("  mov rax, {}", re.to_long());

  while ! re.end() {
    if re.expect('+') {
      println!("  add rax, {}", re.to_long());
      continue;
    }

    if re.expect('-') {
      println!("  sub rax, {}", re.to_long());
      continue;
    }

    eprintln!("unexpected character: '{}'", re.get_char());
    return Err(());
  }

  println!("  ret");
  return Ok(());
}

struct ReadString {
  stri : String,
  strv : Vec<char>,
  cur : usize,
}

impl ReadString {
  fn read(s: &String) -> ReadString {
    ReadString {
      stri : s.to_string(),
      strv : s.chars().collect(),
      cur : 0,
    }
  }

  fn to_long(&mut self) -> i64 {
    let mut len = self.cur;
    while len + 1 <= self.stri.len() && self.stri[self.cur..(len + 1)].parse::<i64>().is_ok() {
      len += 1
    }
    let ret = self.stri[self.cur..len].parse().unwrap();
    self.cur = len;
    ret
  }

  fn expect(&mut self, c: char) -> bool {
    if self.strv[self.cur] == c {
      self.cur += 1;
      true
    } else {
      false
    }
  }

  fn get_char(&self) -> char {
    self.strv[self.cur]
  }

  fn end(&self) -> bool {
    self.strv.len() <= self.cur
  }
}
