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
pub enum UnaryKind {
    Address,
    Deref,
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
pub enum VariableType {
    Int,
    Pointer(Box<VariableType>),
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
    UnaryOperator {
        kind: UnaryKind,
        expression: Box<Node>,
    },
    Num(i64),
    LocalVariable(VariableType, i64),
}