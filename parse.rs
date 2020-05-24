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
    ReturnStatement(Box<Node>),
    BinaryOperator {
        kind: NodeKind,
        left: Box<Node>,
        right: Box<Node>
    },
    Num(i64),
    LocalVariable(i64),
}

pub struct ParseError;

pub fn node(token: &mut TokenList) -> Result<Node, ParseError> {
    let mut code = Vec::new();
    let mut vars = Vec::new();
    while !token.at_eof() {
        code.push(statement(token, &mut vars)?);
    }
    Ok(Node::Statements(code))
}

fn statement(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    let node = if token.consume("return") {
        Node::new_return(expression(token, vars)?)
    } else {
        expression(token, vars)?
    };
    if token.consume(";") {
        Ok(node)
    } else {
        Err(ParseError)
    }
}

fn expression(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    assign(token, vars)
}

fn assign(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    let node = equality(token, vars)?;
    if token.consume("=") {
        Ok(Node::new_binary(NodeKind::Assign, node, assign(token, vars)?))
    } else {
        Ok(node)
    }
}

fn equality(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    let mut node = relational(token, vars)?;
    loop {
        if token.consume("==") {
            node = Node::new_compare(CompareKind::Equal, node, relational(token, vars)?);
        } else if token.consume("!=") {
            node = Node::new_compare(CompareKind::NotEqual, node, relational(token, vars)?);
        } else {
            return Ok(node);
        }
    }
}

fn relational(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    let mut node = add(token, vars)?;
    loop {
        if token.consume("<") {
            node = Node::new_compare(CompareKind::LessThan, node, add(token, vars)?);
        } else if token.consume("<=") {
            node = Node::new_compare(CompareKind::LessEqual, node, add(token, vars)?);
        } else if token.consume(">") {
            node = Node::new_compare(CompareKind::LessThan, add(token, vars)?, node);
        } else if token.consume(">=") {
            node = Node::new_compare(CompareKind::LessEqual, add(token, vars)?, node);
        } else {
            return Ok(node);
        }
    }
}

fn add(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    let mut node = mul(token, vars)?;
    loop {
        if token.consume("+") {
            node = Node::new_binary(NodeKind::Add, node, mul(token, vars)?);
        } else if token.consume("-") {
            node = Node::new_binary(NodeKind::Subtract, node, mul(token, vars)?);
        } else {
            return Ok(node);
        }
    }
}

fn mul(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    let mut node = unary(token, vars)?;
    loop {
        if token.consume("*") {
            node = Node::new_binary(NodeKind::Multiply, node, unary(token, vars)?);
        } else if token.consume("/") {
            node = Node::new_binary(NodeKind::Divide, node, unary(token, vars)?);
        } else {
            return Ok(node);
        }
    }
}

fn unary(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if token.consume("+") {
        Ok(primary(token, vars)?)
    } else if token.consume("-") {
        Ok(Node::new_binary(NodeKind::Subtract, Node::Num(0), primary(token, vars)?))
    } else {
        Ok(primary(token, vars)?)
    }
}

fn primary(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if token.consume("(") {
        let node = expression(token, vars)?;
        if token.consume(")") {
            Ok(node)
        } else {
            Err(ParseError)
        }
    } else if token.consume_ident() {
        Ok(ident(token, vars)?)
    } else {
        Ok(number(token, vars)?)
    }
}

fn number(token: &mut TokenList, _vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if let Some(i) = token.expect_num() {
        Ok(Node::Num(i))
    } else {
        Err(ParseError)
    }
}

fn ident(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if let Some(ident) = token.expect_ident() {
        let len = vars.len();
        for i in 0 .. len {
            if vars[i] == ident {
                return Ok(Node::LocalVariable(i as i64 * 8))
            }
        }
        if len < 26 {
            vars.push(ident);
            Ok(Node::LocalVariable(len as i64 * 8))
        } else {
            Err(ParseError)
        }
    } else {
        Err(ParseError)
    }
}

impl Node {
    fn new_binary(kind: NodeKind, left: Self, right: Self) -> Self {
        Node::BinaryOperator{
            kind, 
            left: Box::new(left), 
            right: Box::new(right)
        }
    }

    fn new_compare(kind: CompareKind, left: Self, right: Self) -> Self {
        Node::BinaryOperator{
            kind: NodeKind::Compare(kind), 
            left: Box::new(left), 
            right: Box::new(right)
        }
    }

    fn new_return(expr: Self) -> Self {
        Node::ReturnStatement(Box::new(expr))
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError")
    }
}