use crate::ccc::error::CompileError;
use crate::ccc::parse::node::CompareKind;
use crate::ccc::parse::node::Node;
use crate::ccc::parse::node::NodeKind;
use crate::ccc::parse::node::UnaryKind;
use crate::ccc::parse::node::VariableType;
use crate::ccc::parse::token::token::TokenList;

pub fn program(token: &mut TokenList) -> Result<Node, CompileError> {
    let mut code = Vec::new();
    while !token.at_eof() {
        let mut vars = Vec::new();
        code.push(function(token, &mut vars)?);
    }
    Ok(Node::Program(code))
}

fn function(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    token.expect_reserved("int")?;
    if let Some(name) = token.expect_identify() {
        token.expect_reserved("(")?;
        let mut args = Vec::new();
        let mut i = 0;
        while !token.consume_reserved(")") {
            if i != 0 {
                token.expect_reserved(",")?;
            }
            token.expect_reserved("int")?;
            args.push(declaration(token, vars)?);
            i += 1;
            if i > 6 {
                return Err(CompileError::new(
                    "関数の引数は6こ以下です。",
                    token.position(),
                    token.code(),
                ));
            }
        }
        let stmt = statement(token, vars)?;
        Ok(Node::new_function(name, args, stmt))
    } else {
        Err(CompileError::new(
            "関数を定義してください。",
            token.position(),
            token.code(),
        ))
    }
}

fn statement(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    if token.consume_reserved("{") {
        let mut vect = Vec::new();
        loop {
            if token.consume_reserved("}") {
                return Ok(Node::new_block(vect));
            }
            vect.push(statement(token, vars)?);
        }
    } else if token.consume_reserved("if") {
        token.expect_reserved("(")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        let node = if token.consume_reserved("else") {
            Node::new_if_else(cond, stmt, statement(token, vars)?)
        } else {
            Node::new_if(cond, stmt)
        };
        return Ok(node);
    } else if token.consume_reserved("while") {
        token.expect_reserved("(")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        return Ok(Node::new_while(cond, stmt));
    } else if token.consume_reserved("for") {
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
    let node = if token.consume_reserved("return") {
        Node::new_return(expression(token, vars)?)
    } else if token.consume_reserved("int") {
        declaration(token, vars)?
    } else {
        expression(token, vars)?
    };
    token.expect_reserved(";")?;
    Ok(node)
}

fn expression(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    assign(token, vars)
}

fn assign(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    let node = equality(token, vars)?;
    if token.consume_reserved("=") {
        Ok(Node::new_binary(
            NodeKind::Assign,
            node,
            assign(token, vars)?,
        ))
    } else {
        Ok(node)
    }
}

fn equality(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    let mut node = relational(token, vars)?;
    loop {
        if token.consume_reserved("==") {
            node = Node::new_compare(CompareKind::Equal, node, relational(token, vars)?);
        } else if token.consume_reserved("!=") {
            node = Node::new_compare(CompareKind::NotEqual, node, relational(token, vars)?);
        } else {
            return Ok(node);
        }
    }
}

fn relational(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    let mut node = add(token, vars)?;
    loop {
        if token.consume_reserved("<") {
            node = Node::new_compare(CompareKind::LessThan, node, add(token, vars)?);
        } else if token.consume_reserved("<=") {
            node = Node::new_compare(CompareKind::LessEqual, node, add(token, vars)?);
        } else if token.consume_reserved(">") {
            node = Node::new_compare(CompareKind::LessThan, add(token, vars)?, node);
        } else if token.consume_reserved(">=") {
            node = Node::new_compare(CompareKind::LessEqual, add(token, vars)?, node);
        } else {
            return Ok(node);
        }
    }
}

fn add(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    let mut node = mul(token, vars)?;
    let rate = match &node {
        Node::LocalVariable(VariableType::Pointer(t), _) => match &**t {
            VariableType::Int => 4,
            VariableType::Pointer(_) => 8,
        },
        _ => 1,
    };
    loop {
        if token.consume_reserved("+") {
            let mul = mul(token, vars)?;
            let rated = if rate != 1 {
                Node::new_binary(NodeKind::Multiply, mul, Node::Num(rate))
            } else {
                mul
            };
            node = Node::new_binary(NodeKind::Add, node, rated);
        } else if token.consume_reserved("-") {
            let mul = mul(token, vars)?;
            let rated = if rate != 1 {
                Node::new_binary(NodeKind::Multiply, mul, Node::Num(rate))
            } else {
                mul
            };
            node = Node::new_binary(NodeKind::Subtract, node, rated);
        } else {
            return Ok(node);
        }
    }
}

fn mul(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    let mut node = unary(token, vars)?;
    loop {
        if token.consume_reserved("*") {
            node = Node::new_binary(NodeKind::Multiply, node, unary(token, vars)?);
        } else if token.consume_reserved("/") {
            node = Node::new_binary(NodeKind::Divide, node, unary(token, vars)?);
        } else {
            return Ok(node);
        }
    }
}

fn unary(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    if token.consume_reserved("+") {
        Ok(primary(token, vars)?)
    } else if token.consume_reserved("-") {
        Ok(Node::new_binary(
            NodeKind::Subtract,
            Node::Num(0),
            primary(token, vars)?,
        ))
    } else if token.consume_reserved("*") {
        Ok(Node::new_unary(UnaryKind::Deref, unary(token, vars)?))
    } else if token.consume_reserved("&") {
        Ok(Node::new_unary(UnaryKind::Address, unary(token, vars)?))
    } else {
        Ok(primary(token, vars)?)
    }
}

fn primary(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    if token.consume_reserved("(") {
        let node = expression(token, vars)?;
        token.expect_reserved(")")?;
        Ok(node)
    } else if token.next_identify() {
        Ok(identify(token, vars)?)
    } else {
        Ok(number(token, vars)?)
    }
}

fn number(
    token: &mut TokenList,
    _vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    Ok(Node::Num(token.expect_num()?))
}

fn identify(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    if let Some(identify) = token.expect_identify() {
        if token.consume_reserved("(") {
            if token.consume_reserved(")") {
                return Ok(Node::FunctionCall {
                    name: identify,
                    args: Vec::default(),
                });
            }
            let mut vect = Vec::new();
            let mut i = 0;
            while !token.consume_reserved(")") {
                if i != 0 {
                    token.expect_reserved(",")?;
                }
                vect.push(expression(token, vars)?);
                i += 1;
                if i > 6 {
                    return Err(CompileError::new(
                        "関数の引数は6こ以下です。",
                        token.position(),
                        token.code(),
                    ));
                }
            }
            return Ok(Node::FunctionCall {
                name: identify,
                args: vect,
            });
        }
        let mut ofs = 8;
        for (var, t, i) in &*vars {
            if var == &identify {
                return Ok(Node::LocalVariable(t.clone(), ofs));
            }
            ofs += i;
        }

        Err(CompileError::new(
            "宣言された変数ではありません。",
            token.position(),
            token.code(),
        ))
    } else {
        Err(CompileError::new(
            "識別子ではありません。",
            token.position(),
            token.code(),
        ))
    }
}

fn declaration(
    token: &mut TokenList,
    vars: &mut Vec<(String, VariableType, i64)>,
) -> Result<Node, CompileError> {
    let (t, s) = declaration_identify(token)?;
    let mut ofs = 8;
    for (var, _, i) in &*vars {
        if *var == s {
            return Err(CompileError::new(
                "すでに宣言された変数です。",
                token.position(),
                token.code(),
            ));
        }
        ofs += i;
    }
    let size = match &t {
        VariableType::Int => 8,
        VariableType::Pointer(_) => 8,
    };

    vars.push((s, t.clone(), size));

    Ok(Node::LocalVariable(t, ofs))
}

fn declaration_identify(token: &mut TokenList) -> Result<(VariableType, String), CompileError> {
    if token.consume_reserved("*") {
        let (t, s) = declaration_identify(token)?;
        Ok((VariableType::Pointer(Box::new(t)), s))
    } else {
        match token.expect_identify() {
            Some(s) => Ok((VariableType::Int, s)),
            None => Err(CompileError::new(
                "宣言が変数ではありません。",
                token.position(),
                token.code(),
            )),
        }
    }
}
