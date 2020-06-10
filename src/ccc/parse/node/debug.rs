use std::fmt::{Debug, Formatter, Result};

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

impl Debug for super::StatementKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::StatementKind::{Block, For, If, IfElse, Return, While};
        match self {
            Return(expr) => write!(f, "Return {:?}", expr),
            If {
                condition,
                t_statement,
            } => write!(f, "If {:?} Then {:?}", condition, t_statement),
            IfElse {
                condition,
                t_statement,
                f_statement,
            } => write!(
                f,
                "If {:?} Then {:?} Else {:?}",
                condition, t_statement, f_statement
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

impl Debug for super::Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use super::Node::{
            BinaryOperator, Function, FunctionCall, LocalVariable, Num, Program, Statement,
            UnaryOperator,
        };
        match self {
            Program(funcs) => {
                write!(f, "Program {{ ")?;
                for func in funcs {
                    write!(f, "{:?} ", &func)?;
                }
                write!(f, "}}")
            }
            Function {
                name,
                args,
                statement,
            } => {
                write!(f, "{} ( ", name)?;
                for arg in args {
                    write!(f, "{:?} ", arg)?;
                }
                write!(f, ") {:?}", statement)
            }
            Statement(kind) => write!(f, "{:?}", kind),
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
            LocalVariable(ty, ofs) => write!(f, "{:?}:{}", ty, ofs),
        }
    }
}
