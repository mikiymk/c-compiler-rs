use std::fmt::{Debug, Formatter, Result};

impl Debug for super::Program {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Program {{ ")?;
        for func in &self.codes {
            write!(f, "{:?} ", &func)?;
        }
        write!(f, "}}")
    }
}

impl Debug for super::Function {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} (", self.name)?;
        for arg in &self.arguments {
            write!(f, "{:?} ", arg)?;
        }
        write!(f, ") {{ ")?;
        for stmt in &self.statements {
            write!(f, "{:?}; ", stmt)?;
        }
        write!(f, "}}")
    }
}

impl Debug for super::Statement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::Statement::{Block, Declaration, Expression, For, If, IfElse, Return, While};
        match self {
            Return(expr) => write!(f, "Return {:?}", expr),
            Declaration(var) => write!(f, "Declaration {:?}", var),
            Expression(expr) => write!(f, "{:?}", expr),
            If {
                condition,
                true_statement,
            } => write!(f, "If {:?} Then {:?}", condition, true_statement),
            IfElse {
                condition,
                true_statement,
                false_statement,
            } => write!(
                f,
                "If {:?} Then {:?} Else {:?}",
                condition, true_statement, false_statement
            ),
            While {
                condition,
                statement,
            } => write!(f, "While {:?} Do {:?}", condition, statement),
            For {
                init,
                condition,
                iteration,
                statement,
            } => write!(
                f,
                "For {:?} {:?} {:?} Do {:?}",
                init, condition, iteration, statement
            ),
            Block { statements } => {
                write!(f, "{{ ")?;
                for statement in statements {
                    write!(f, "{:?}; ", statement)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Debug for super::Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::Expression::{BinaryOperator, FunctionCall, LocalVariable, Num, UnaryOperator};
        match self {
            FunctionCall { name, args } => {
                write!(f, "{} ( ", name)?;
                for arg in args {
                    write!(f, "{:?} ", arg)?;
                }
                write!(f, ")")
            }
            BinaryOperator { kind, left, right } => {
                write!(f, "({:?} {:?} {:?})", kind, left, right)
            }
            UnaryOperator { kind, expression } => write!(f, "({:?} {:?})", kind, expression),
            Num(i) => write!(f, "{}", i),
            LocalVariable(v) => write!(f, "{:?}", v),
        }
    }
}

impl Debug for super::Variable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {:?}:{}", self.name, self.var_type, self.offset)
    }
}

impl Debug for super::BinaryKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::BinaryKind::{Add, Assign, Compare, Divide, Multiply, Subtract};
        match self {
            Add => write!(f, "Add"),
            Subtract => write!(f, "Sub"),
            Multiply => write!(f, "Mul"),
            Divide => write!(f, "Div"),
            Compare(k) => write!(f, "Cmp:{:?}", k),
            Assign => write!(f, "Assign"),
        }
    }
}

impl Debug for super::CompareKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::CompareKind::{Equal, LessEqual, LessThan, NotEqual};
        match self {
            Equal => write!(f, "Eq"),
            NotEqual => write!(f, "Ne"),
            LessThan => write!(f, "Lt"),
            LessEqual => write!(f, "Le"),
        }
    }
}

impl Debug for super::UnaryKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::UnaryKind::{Address, Deref};
        match self {
            Address => write!(f, "Addr"),
            Deref => write!(f, "Deref"),
        }
    }
}

impl Debug for super::VariableType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::VariableType::{Array, Int, Pointer};
        match self {
            Int => write!(f, "Int"),
            Pointer(ty) => write!(f, "P({:?})", ty),
            Array(ty, size) => write!(f, "{:?}[{}]", ty, size),
        }
    }
}
