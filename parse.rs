use token::TokenList;

#[derive(Debug)]
pub enum NodeKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Compare(CompareKind),
    Assign,
}

#[derive(Debug)]
pub enum CompareKind {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
}

#[derive(Debug)]
pub enum Node {
    Statements(Vec<Node>),
    BinaryOperator {
        kind: NodeKind,
        left: Box<Node>,
        right: Box<Node>
    },
    Num(i64),
    LocalVariable(i64),
}

pub fn node(token: &mut TokenList) -> Node {
    let mut code = Vec::new();
    while !token.at_eof() {
        code.push(statement(token));
    }
    Node::Statements(code)
}

fn statement(token: &mut TokenList) -> Node {
    let node = expression(token);
    token.expect(";");
    node
}

fn expression(token: &mut TokenList) -> Node {
    assign(token)
}

fn assign(token: &mut TokenList) -> Node {
    let node = equality(token);
    if token.consume("=") {
        Node::new_binary(NodeKind::Assign, node, assign(token))
    } else {
        node
    }
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

fn unary(token: &mut TokenList) -> Node {
    if token.consume("+") {
        primary(token)
    } else if token.consume("-") {
        Node::new_binary(NodeKind::Subtract, Node::Num(0), primary(token))
    } else {
        primary(token)
    }
}

fn primary(token: &mut TokenList) -> Node {
    if token.consume("(") {
        let node = expression(token);
        token.expect(")");
        node
    } else if token.consume_ident() {
        ident(token)
    } else {
        number(token)
    }
}

fn number(token: &mut TokenList) -> Node {
    Node::Num(token.expect_num().unwrap())
}

fn ident(token: &mut TokenList) -> Node {
    Node::LocalVariable(token.expect_ident().unwrap() * 8)
}

impl Node {
    fn new_binary(kind: NodeKind, left: Node, right: Node) -> Self {
        Node::BinaryOperator{
            kind, 
            left: Box::new(left), 
            right: Box::new(right)
        }
    }

    fn new_compare(kind: CompareKind, left: Node, right: Node) -> Self {
        Node::BinaryOperator{
            kind: NodeKind::Compare(kind), 
            left: Box::new(left), 
            right: Box::new(right)
        }
    }
}