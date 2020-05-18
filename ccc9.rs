fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません。");
        return;
    }

    let program = &args[1];

    if let Some(mut token) = tokenize(&program) {
        let node = node(&mut token);
        assemblize(&node);
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
        Node::Num(i) => {
            println!("  push {}", i)
        },
        Node::BinaryOperator{kind, left, right} => {
            gen(&*left);
            gen(&*right);
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
            }
            println!("  push rax");
        },
        Node::CompareOperator{kind, left, right} => {
            gen(&*left);
            gen(&*right);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  cmp rax, rdi");
            match kind {
                CompareKind::Equal => println!("  sete al"),
                CompareKind::NotEqual => println!("  setne al"),
                CompareKind::LessThan => println!("  setl al"),
                CompareKind::LessEqual => println!("  setle al"),
            }
            println!("  movzb rax, al");
            println!("  push rax");
        },
    }
}

fn tokenize(code: &String) -> Option<TokenList> {
    let mut vect = Vec::new();
    let codev = code.chars().collect::<Vec<char>>();
    let len = codev.len();
    let mut cur = 0;

    while cur < len {
        match codev[cur] {
            ' ' => {
                cur += 1;
            }
            '+' | '-' | '*' | '/' | '(' | ')' => {
                vect.push(Token::new(TokenKind::RESERVED, codev[cur].to_string()));
                cur += 1;
            }
            '0' ..= '9' => {
                let (lo, c) = str_to_long(code, cur);
                cur = c;
                vect.push(Token::new(TokenKind::NUM(lo), lo.to_string()));
            },
            '=' | '!' | '<' | '>' => {
                let next = codev[cur + 1];
                if next == '=' {
                    vect.push(Token::new(TokenKind::RESERVED, format!("{}{}", codev[cur], codev[cur + 1])));
                    cur += 2;
                } else {
                    vect.push(Token::new(TokenKind::RESERVED, codev[cur].to_string()));
                    cur += 1;
                }
            },
            _ => {
                eprintln!("トークナイズ出来ません。");
                return None;
            }
        }
    }
    Some(TokenList{ list: vect })
}

fn str_to_long(code: &String, cursor: usize) -> (i64, usize) {
    let mut len = cursor;
    while len + 1 <= code.len() && code[cursor..len + 1].parse::<i64>().is_ok() {
      len += 1
    }
    (code[cursor..len].parse().unwrap(), len)
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

    fn expect_num(&mut self) -> Option<i64> {
        let token = self.list.remove(0);
        if let TokenKind::NUM(i) = token.kind {
            Some(i)
        } else {
            None
        }
    }

    fn at_eof(&self) -> bool {
        self.list.len() == 0
    }
}

fn node(token: &mut TokenList) -> Node {
    equality(token)
}

fn equality(token: &mut TokenList) -> Node {
    let mut node = relational(token);
    loop {
        if token.consume("==") {
            node = Node::new_compare(CompareKind::Equal, node, relational(token));
        } else if token.consume("!=") {
            node = Node::new_compare(CompareKind::NotEqual, node, relational(token));
        } else {
            return node;
        }
    }
}

fn relational(token: &mut TokenList) -> Node {
    let mut node = add(token);
    loop {
        if token.consume("<") {
            node = Node::new_compare(CompareKind::LessThan, node, add(token));
        } else if token.consume("<=") {
            node = Node::new_compare(CompareKind::LessEqual, node, add(token));
        } else if token.consume(">") {
            node = Node::new_compare(CompareKind::LessThan, add(token), node);
        } else if token.consume(">=") {
            node = Node::new_compare(CompareKind::LessEqual, add(token), node);
        } else {
            return node;
        }
    }
}

fn add(token: &mut TokenList) -> Node {
    let mut node = mul(token);
    loop {
        if token.consume("+") {
            node = Node::new_binary(NodeKind::Add, node, mul(token));
        } else if token.consume("-") {
            node = Node::new_binary(NodeKind::Subtract, node, mul(token));
        } else {
            return node;
        }
    }
}

fn mul(token: &mut TokenList) -> Node {
    let mut node = unary(token);
    loop {
        if token.consume("*") {
            node = Node::new_binary(NodeKind::Multiply, node, unary(token));
        } else if token.consume("/") {
            node = Node::new_binary(NodeKind::Divide, node, unary(token));
        } else {
            return node;
        }
    }
  }

fn primary(token: &mut TokenList) -> Node {
    if token.consume("(") {
        let node = node(token);
        token.expect(")");
        node
    } else {
        number(token)
    }
}

fn unary(token: &mut TokenList) -> Node {
    if token.consume("+") {
        primary(token)
    } else if token.consume("-") {
        Node::new_binary(NodeKind::Subtract, Node::Num(0), primary(token))
    } else {
        primary(token)
    }
}

fn number(token: &mut TokenList) -> Node {
    Node::Num(token.expect_num().unwrap())
}

enum NodeKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum CompareKind {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
}

enum Node {
    BinaryOperator {
        kind: NodeKind,
        left: Box<Node>,
        right: Box<Node>
    },
    CompareOperator {
        kind: CompareKind,
        left: Box<Node>,
        right: Box<Node>
    },
    Num(i64),
}

impl Node {
    fn new_binary(kind: NodeKind, left: Node, right: Node) -> Node {
        Node::BinaryOperator{
            kind, 
            left: Box::new(left), 
            right: Box::new(right)
        }
    }

    fn new_compare(kind: CompareKind, left: Node, right: Node) -> Node {
        Node::CompareOperator{
            kind, 
            left: Box::new(left), 
            right: Box::new(right)
        }
    }
}