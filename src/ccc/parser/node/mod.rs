mod debug;

pub struct Program {
    codes: Vec<Function>,
}

pub struct Function {
    name: String,
    arguments: Vec<Variable>,
    statements: Vec<Statement>,
}

pub enum Statement {
    Declaration(Variable),
    Expression(Expression),
    Return(Expression),
    If {
        condition: Expression,
        true_statement: Box<Statement>,
    },
    IfElse {
        condition: Expression,
        true_statement: Box<Statement>,
        false_statement: Box<Statement>,
    },
    While {
        condition: Expression,
        statement: Box<Statement>,
    },
    For {
        init: Expression,
        condition: Expression,
        iteration: Expression,
        statement: Box<Statement>,
    },
    Block {
        statements: Vec<Statement>,
    },
}

pub enum Expression {
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    BinaryOperator {
        kind: BinaryKind,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOperator {
        kind: UnaryKind,
        expression: Box<Expression>,
    },
    Num(i64),
    LocalVariable(Variable),
}

#[derive(Clone)]
pub struct Variable {
    var_type: VariableType,
    name: String,
    offset: i64,
}

pub enum UnaryKind {
    Address,
    Deref,
}

pub enum BinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Compare(CompareKind),
    Assign,
}

pub enum CompareKind {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
}

pub enum VariableType {
    Int,
    Pointer(Box<VariableType>),
    Array(Box<VariableType>, i64),
}

impl Program {
    pub fn codes(&self) -> &Vec<Function> {
        &self.codes
    }
}

impl Function {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<Variable> {
        &self.arguments
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Expression {
    pub fn kind(&self) -> Result<VariableType, &'static str> {
        use Expression::{BinaryOperator, FunctionCall, Num, UnaryOperator};
        use VariableType::{Int, Pointer};
        match self {
            Num(_) | FunctionCall { .. } => Ok(Int),
            Expression::LocalVariable(Variable { var_type, .. }) => Ok(var_type.clone()),

            BinaryOperator { kind, left, right } => match kind {
                BinaryKind::Assign => left.kind(),
                BinaryKind::Compare(_) => Ok(Int),
                _ => match left.kind()? {
                    Int => right.kind(),
                    k => Ok(k),
                },
            },

            UnaryOperator { kind, expression } => match kind {
                UnaryKind::Address => Ok(Pointer(Box::new(expression.kind()?))),
                UnaryKind::Deref => match expression.kind()? {
                    VariableType::Pointer(t) | VariableType::Array(t, _) => Ok(*t),
                    VariableType::Int => Err("無効な参照です。"),
                },
            },
        }
    }
}

impl Variable {
    pub fn var_type(&self) -> &VariableType {
        &self.var_type
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }
}

impl VariableType {
    pub fn size(&self) -> i64 {
        use VariableType::{Array, Int, Pointer};
        match self {
            Int => 4,
            Pointer(_) => 8,
            Array(ref_type, size) => ref_type.size() * size,
        }
    }
}

impl PartialEq for VariableType {
    fn eq(&self, other: &Self) -> bool {
        use VariableType::{Array, Int, Pointer};
        match (self, other) {
            (Int, Int) => true,
            (Pointer(ty), Pointer(pe)) => ty == pe,
            (Array(ty, s), Array(pe, o)) => ty == pe && s == o,
            (_, _) => false,
        }
    }
}

impl Clone for VariableType {
    fn clone(&self) -> Self {
        use VariableType::{Array, Int, Pointer};
        match self {
            Int => Int,
            Pointer(b) => Pointer(Box::new(*b.clone())),
            Array(t, s) => Array(Box::new(*t.clone()), *s),
        }
    }
}

pub fn new_program(codes: Vec<Function>) -> Program {
    Program { codes }
}

pub fn new_binary(kind: BinaryKind, left: Expression, right: Expression) -> Expression {
    Expression::BinaryOperator {
        kind,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn new_compare(kind: CompareKind, left: Expression, right: Expression) -> Expression {
    Expression::BinaryOperator {
        kind: BinaryKind::Compare(kind),
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn new_return(expression: Expression) -> Statement {
    Statement::Return(expression)
}

pub fn new_if(condition: Expression, true_statement: Statement) -> Statement {
    Statement::If {
        condition,
        true_statement: Box::new(true_statement),
    }
}

pub fn new_if_else(
    condition: Expression,
    true_statement: Statement,
    false_statement: Statement,
) -> Statement {
    Statement::IfElse {
        condition,
        true_statement: Box::new(true_statement),
        false_statement: Box::new(false_statement),
    }
}

pub fn new_while(condition: Expression, statement: Statement) -> Statement {
    Statement::While {
        condition,
        statement: Box::new(statement),
    }
}

pub fn new_for(
    init: Expression,
    condition: Expression,
    iteration: Expression,
    statement: Statement,
) -> Statement {
    Statement::For {
        init,
        condition,
        iteration,
        statement: Box::new(statement),
    }
}

pub fn new_block(statements: Vec<Statement>) -> Statement {
    Statement::Block { statements }
}

pub fn new_function(
    name: String,
    arguments: Vec<Variable>,
    statements: Vec<Statement>,
) -> Function {
    Function {
        name,
        arguments,
        statements,
    }
}

pub fn new_unary(kind: UnaryKind, expression: Expression) -> Expression {
    Expression::UnaryOperator {
        kind,
        expression: Box::new(expression),
    }
}

pub fn new_variable(var_type: VariableType, name: String, offset: i64) -> Variable {
    Variable {
        var_type,
        name,
        offset,
    }
}
