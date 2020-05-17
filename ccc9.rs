fn main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().collect();
  if args.len() != 2 {
    return Err("引数の個数が正しくありません。".to_string());
  }

  let program = &args[1];

  match Token::tokenize(&program) {
    Ok(mut token) => {
      let node = Node::node_expr(&mut token);
      assemblize(&node);
      Ok(())
    },
    Err(err) => Err(err)
  }
}

fn assemblize(node: &Node) {
  println!(".intel_syntax noprefix");
  println!(".global main");
  println!("main:");

  gen(node);

  println!("  pop rax");
  println!("  ret");
}

fn gen(node: &Node) {
  match node {
    Node::Num(_, i) => {
      println!("  push {}", i)
    },
    Node::BinaryOperator(kind, lhs, rhs) => {
      gen(&*lhs);
      gen(&*rhs);
      println!("  pop rdi");
      println!("  pop rax");
      match kind {
        NodeKind::Add => println!("  add rax, rdi"),
        NodeKind::Subtract => println!("  sub rax, rdi"),
        NodeKind::Multiply => println!("  imul rax, rdi"),
        NodeKind::Divide => {
          println!("  cqo");
          println!("  idiv rdi");
        },
        _ => {},
      }
      println!("  push rax");
    },
  }
}

enum NodeKind {
  Add,
  Subtract,
  Multiply,
  Divide,
  Integer,
}

enum Node {
  BinaryOperator(NodeKind, Box<Node>, Box<Node>),
  Num(NodeKind, i64),
}

impl Node {
  fn node_expr(token: &mut TokenList) -> Node {
    let mut node = Node::node_mul(token);
    loop {
      if token.consume("+") {
        node = Node::new_binary(NodeKind::Add, node, Node::node_mul(token));
      } else if token.consume("-") {
        node = Node::new_binary(NodeKind::Subtract, node, Node::node_mul(token));
      } else {
        return node;
      }
    }
  }

  fn node_mul(token: &mut TokenList) -> Node {
    let mut node = Node::node_unary(token);
    loop {
      if token.consume("*") {
        node = Node::new_binary(NodeKind::Multiply, node, Node::node_unary(token));
      } else if token.consume("/") {
        node = Node::new_binary(NodeKind::Divide, node, Node::node_unary(token));
      } else {
        return node;
      }
    }
  }

  fn node_primary(token: &mut TokenList) -> Node {
    if token.consume("(") {
      let node = Node::node_expr(token);
      token.expect(")");
      node
    } else {
      Node::node_number(token)
    }
  }

  fn node_unary(token: &mut TokenList) -> Node {
    if token.consume("+") {
      Node::node_primary(token)
    } else if token.consume("-") {
      Node::new_binary(NodeKind::Subtract, Node::Num(NodeKind::Integer, 0), Node::node_primary(token))
    } else {
      Node::node_primary(token)
    }
  }

  fn node_number(token: &mut TokenList) -> Node {
    Node::Num(NodeKind::Integer, token.expect_num().unwrap())
  }

  fn new_binary(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    Node::BinaryOperator(kind, Box::new(lhs), Box::new(rhs))
  }
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
      
      if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
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
    Token { kind, stri }
  }
}

struct TokenList {
  list : Vec<Token>,
}

impl TokenList {
  fn consume(&mut self, stri: &str) -> bool {
    if !self.at_eof() && self.list[0].stri == stri {
      self.list.remove(0);
      true
    } else {
      false
    }
  }

  fn expect(&mut self, stri: &str) -> bool {
    !self.at_eof() && self.list.remove(0).stri == stri
  }

  fn expect_num(&mut self) -> Result<i64, Token> {
    let token = self.list.remove(0);
    if let TokenKind::NUM(i) = token.kind {
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
