use token::TokenList;

pub enum NodeKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Compare(CompareKind),
}

pub enum CompareKind {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
}

pub enum Node {
    BinaryOperator {
        kind: NodeKind,
        left: Box<Node>,
        right: Box<Node>
    },
    Num(i64),
}

pub fn node(token: &mut TokenList) -> Node {
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