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
pub enum StatementKind {
    Return(Box<Node>),
    If {
        condition: Box<Node>,
        t_statement: Box<Node>,
    },
    IfElse {
        condition: Box<Node>,
        t_statement: Box<Node>,
        f_statement: Box<Node>,
    },
    While {
        condition: Box<Node>,
        statement: Box<Node>,
    },
    For {
        init: Box<Node>,
        condition: Box<Node>,
        iteration: Box<Node>,
        statement: Box<Node>,
    },
    Block {
        statements: Vec<Node>,
    }
}

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    Function {
        name: String,
        args: Vec<Node>,
        statement: Box<Node>
    },
    Statement(StatementKind),
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    BinaryOperator {
        kind: NodeKind,
        left: Box<Node>,
        right: Box<Node>
    },
    Num(i64),
    LocalVariable(i64),
}

pub struct ParseError(String);

pub fn node(token: &mut TokenList) -> Result<Node, ParseError> {
    let mut code = Vec::new();
    while !token.at_eof() {
        let mut vars = Vec::new();
        code.push(function(token, &mut vars)?);
    }
    Ok(Node::Program(code))
}

fn function(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if let Some(name) = token.expect_ident() {
        token.expect_reserved("(")?;
        let mut args = Vec::new();
        let mut i = 0;
        while !token.consume(")") {
            if i != 0 {
                token.expect_reserved(",")?;
            }
            args.push(ident(token, vars)?);
            i += 1;
            
            if i > 6 {
                return Err(ParseError("関数の引数は6こ以下です。".to_string()))
            }
        }
        let stmt = statement(token, vars)?;
        Ok(Node::new_function(name, args, stmt))
    } else {
        Err(ParseError("関数を定義してください。".to_string()))
    }
}

fn statement(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if token.consume("{") {
        let mut vect = Vec::new();
        loop {
            if token.consume("}") {
                return Ok(Node::new_block(vect));
            }
            vect.push(statement(token, vars)?);
        }
    } else if token.consume("if") {
        token.expect_reserved("(")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        let node = if token.consume("else") {
            Node::new_if_else(cond, stmt, statement(token, vars)?)
        } else {
            Node::new_if(cond, stmt)
        };
        return Ok(node);
    } else if token.consume("while") {
        token.expect_reserved("(")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        return Ok(Node::new_while(cond, stmt));
    } else if token.consume("for") {
        token.expect_reserved("(")?;
        let init = expression(token, vars)?;
        token.expect_reserved(";")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(";")?;
        let iter = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        return Ok(Node::new_for(init, cond, iter, stmt));
    }
    let node = if token.consume("return") {
        Node::new_return(expression(token, vars)?)
    } else {
        expression(token, vars)?
    };
    token.expect_reserved(";")?;
    Ok(node)
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
        token.expect_reserved(")")?;
        Ok(node)
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
        Err(ParseError("数ではありません。".to_string()))
    }
}

fn ident(token: &mut TokenList, vars: &mut Vec<String>) -> Result<Node, ParseError> {
    if let Some(ident) = token.expect_ident() {
        if token.consume("(") {
            if token.consume(")") {
                return Ok(Node::FunctionCall{ name: ident, args: Vec::default() });
            }
            let mut vect = Vec::new();
            let mut i = 0;
            while !token.consume(")") {
                if i != 0 {
                    token.expect_reserved(",")?;
                }
                vect.push(expression(token, vars)?);
                i += 1;
                
                if i > 6 {
                    return Err(ParseError("関数の引数は6こ以下です。".to_string()))
                }
            }
            return Ok(Node::FunctionCall{ name: ident, args: vect });
        }
        let len = vars.len();
        for i in 0 .. len {
            if vars[i] == ident {
                return Ok(Node::LocalVariable((i as i64 + 1) * 8))
            }
        }
        if len < 26 {
            vars.push(ident);
            Ok(Node::LocalVariable((len as i64 + 1) * 8))
        } else {
            Err(ParseError("変数の数が多すぎます。".to_string()))
        }
    } else {
        Err(ParseError("識別子ではありません。".to_string()))
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
        Node::Statement(StatementKind::Return(Box::new(expr)))
    }

    fn new_if(condition: Self, t_statement: Self) -> Self {
        Node::Statement(StatementKind::If {
            condition: Box::new(condition),
            t_statement: Box::new(t_statement),
        })
    }

    fn new_if_else(condition: Self, t_statement: Self, f_statement: Self) -> Self {
        Node::Statement(StatementKind::IfElse {
            condition: Box::new(condition),
            t_statement: Box::new(t_statement),
            f_statement: Box::new(f_statement),
        })
    }

    fn new_while(condition: Self, statement: Self) -> Self {
        Node::Statement(StatementKind::While {
            condition: Box::new(condition),
            statement: Box::new(statement),
        })
    }

    fn new_for(init: Self, condition: Self, iteration: Self, statement: Self) -> Self {
        Node::Statement(StatementKind::For {
            init: Box::new(init),
            condition: Box::new(condition),
            iteration: Box::new(iteration),
            statement: Box::new(statement),
        })
    }

    fn new_block(vect: Vec<Self>) -> Self {
        Node::Statement(StatementKind::Block {
            statements: vect,
        })
    }

    fn new_function(name: String, args: Vec<Self>, statement: Self) -> Self {
        Node::Function {
            name,
            args,
            statement: Box::new(statement),
        }
    }
}

impl ParseError {
    pub fn new(error: String) -> ParseError {
        ParseError(error)
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "ParseError")?;
        writeln!(f, "{}", self.0)
    }
}