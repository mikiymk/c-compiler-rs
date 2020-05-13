fn main() -> Result<(), ()> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    eprintln!("引数の個数が正しくありません。");
    return Err(());
  }

  let mut token = Token::tokenize(&args[1]).unwrap();
  let mut _re = ReadString::read(&args[1]);

  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");
  println!("  mov rax, {}", token.expect_num().unwrap());

  while ! token.at_eof() {
    if token.expect().stri == "+" {
      println!("  add rax, {}", token.expect_num().unwrap());
      continue;
    }

    println!("  sub rax, {}", token.expect_num().unwrap());
  }

  println!("  ret");
  Ok(())
}

#[derive(Debug)]
enum TokenKind {
  RESERVED,
  NUM(i64),
}

#[derive(Debug)]
struct Token {
  kind : TokenKind,
  stri : String,
}

impl Token {
  fn tokenize(code: &String) -> Result<TokenList, String> {
    let mut vect = Vec::new();
    let mut re = ReadString::read(&code);

    while ! re.end() {
      let c = re.get_char();
      if c == ' ' {
        re.skip();
        continue;
      }
      
      if c == '+' || c == '-' {
        vect.push(Token::new(TokenKind::RESERVED, c.to_string()));
        re.skip();
        continue;
      }

      if c == '1' || c == '2' || c == '3' || c == '4' || c == '5' ||
      c == '6' || c == '7' || c == '8' || c == '9' || c == '0' {
        let lo = re.to_long();
        vect.push(Token::new(TokenKind::NUM(lo), lo.to_string()));
        continue;
      }

      return Err(format!("トークナイズ出来ません。"));
    }
    Ok(TokenList{ list: vect })
  }

  fn new(kind: TokenKind, stri: String) -> Token {
    Token {
      kind,
      stri
    }
  }
}

struct TokenList {
  list: Vec<Token>,
}

impl TokenList {
  fn expect(&mut self) -> Token {
    self.list.remove(0)
  }

  fn expect_num(&mut self) -> Result<i64, Token> {
    use TokenKind::NUM;
    let token = self.expect();
    if let NUM(i) = token.kind {
      Ok(i)
    } else {
      Err(token)
    }
  }

  fn at_eof(&mut self) -> bool {
    self.list.len() == 0
  }
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

  fn skip(&mut self) {
    self.cur += 1;
  }

  fn get_char(&self) -> char {
    self.strv[self.cur]
  }

  fn end(&self) -> bool {
    self.strv.len() <= self.cur
  }
}
