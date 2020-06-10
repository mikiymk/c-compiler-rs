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
    },
}

#[derive(Debug)]
pub enum VariableType {
    Int,
    Pointer(Box<VariableType>),
    Array(Box<VariableType>, i64),
}

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    Function {
        name: String,
        args: Vec<Node>,
        statement: Box<Node>,
    },
    Statement(StatementKind),
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    BinaryOperator {
        kind: NodeKind,
        left: Box<Node>,
        right: Box<Node>,
    },
    UnaryOperator {
        kind: UnaryKind,
        expression: Box<Node>,
    },
    Num(i64),
    LocalVariable(VariableType, i64),
}

impl Node {
    pub fn new_binary(kind: NodeKind, left: Self, right: Self) -> Self {
        Node::BinaryOperator {
            kind,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn new_compare(kind: CompareKind, left: Self, right: Self) -> Self {
        Node::BinaryOperator {
            kind: NodeKind::Compare(kind),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn new_return(expr: Self) -> Self {
        Node::Statement(StatementKind::Return(Box::new(expr)))
    }

    pub fn new_if(condition: Self, t_statement: Self) -> Self {
        Node::Statement(StatementKind::If {
            condition: Box::new(condition),
            t_statement: Box::new(t_statement),
        })
    }

    pub fn new_if_else(condition: Self, t_statement: Self, f_statement: Self) -> Self {
        Node::Statement(StatementKind::IfElse {
            condition: Box::new(condition),
            t_statement: Box::new(t_statement),
            f_statement: Box::new(f_statement),
        })
    }

    pub fn new_while(condition: Self, statement: Self) -> Self {
        Node::Statement(StatementKind::While {
            condition: Box::new(condition),
            statement: Box::new(statement),
        })
    }

    pub fn new_for(init: Self, condition: Self, iteration: Self, statement: Self) -> Self {
        Node::Statement(StatementKind::For {
            init: Box::new(init),
            condition: Box::new(condition),
            iteration: Box::new(iteration),
            statement: Box::new(statement),
        })
    }

    pub fn new_block(vect: Vec<Self>) -> Self {
        Node::Statement(StatementKind::Block { statements: vect })
    }

    pub fn new_function(name: String, args: Vec<Self>, statement: Self) -> Self {
        Node::Function {
            name,
            args,
            statement: Box::new(statement),
        }
    }

    pub fn new_unary(kind: UnaryKind, expression: Self) -> Self {
        Node::UnaryOperator {
            kind,
            expression: Box::new(expression),
        }
    }

    pub fn kind(&self) -> Result<VariableType, &'static str> {
        use VariableType::Int;
        use VariableType::Pointer;
        match self {
            Node::Num(_) => Ok(Int),
            Node::Program(_) | Node::Function { .. } | Node::Statement(_) => {
                Err("値ではありません。")
            }
            Node::FunctionCall { .. } => Ok(Int),

            Node::BinaryOperator { kind, left, right } => match kind {
                NodeKind::Assign => left.kind(),
                NodeKind::Compare(_) => Ok(Int),
                _ => match left.kind()? {
                    Int => right.kind(),
                    k => Ok(k),
                },
            },
            Node::UnaryOperator { kind, expression } => match kind {
                UnaryKind::Address => Ok(Pointer(Box::new(expression.kind()?))),
                UnaryKind::Deref => match expression.kind()? {
                    VariableType::Int => Err("無効な参照です。"),
                    VariableType::Pointer(t) | VariableType::Array(t, _) => Ok(*t),
                },
            },
            Node::LocalVariable(t, _) => Ok(t.clone()),
        }
    }
}

impl VariableType {
    pub fn size(&self) -> i64 {
        match self {
            VariableType::Int => 4,
            VariableType::Pointer(_) => 8,
            VariableType::Array(t, s) => t.size() * s,
        }
    }
}

impl PartialEq for VariableType {
    fn eq(&self, other: &Self) -> bool {
        use VariableType::{Int, Pointer};
        match (self, other) {
            (Int, Int) => true,
            (Pointer(s), Pointer(o)) => s == o,
            (_, _) => false,
        }
    }
}

impl Clone for VariableType {
    fn clone(&self) -> Self {
        match self {
            VariableType::Int => VariableType::Int,
            VariableType::Pointer(b) => VariableType::Pointer(Box::new(*b.clone())),
            VariableType::Array(t, s) => VariableType::Array(Box::new(*t.clone()), *s),
        }
    }
}
