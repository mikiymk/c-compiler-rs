use super::node::BinaryKind;
use super::node::CompareKind;
use super::node::Node;
use super::node::UnaryKind;
use super::node::VariableType;
use super::token::token::TokenList;
use crate::ccc::error::CompileError;

type ParseResult = Result<Node, CompileError>;
type VariableList = Vec<(String, VariableType, i64)>;

pub fn program(token: &mut TokenList) -> ParseResult {
    let mut code = Vec::new();
    while !token.at_eof() {
        code.push(function(token)?);
    }
    Ok(Node::Program(code))
}

fn function(token: &mut TokenList) -> ParseResult {
    let mut vars = Vec::new();
    token.expect_reserved("int")?;
    if let Some(name) = token.expect_identify() {
        token.expect_reserved("(")?;
        let mut args = Vec::new();
        let mut multi = false;
        while !token.consume_reserved(")") {
            if multi {
                token.expect_reserved(",")?;
            }
            args.push(declaration(token, &mut vars)?);
            multi = true;
        }
        let stmt = statement(token, &mut vars)?;
        Ok(Node::new_function(name, args, stmt))
    } else {
        Err(token.error("関数を定義してください。"))
    }
}

fn statement(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    if token.consume_reserved("{") {
        let mut vect = Vec::new();
        while !token.consume_reserved("}") {
            vect.push(statement(token, vars)?);
        }
        Ok(Node::new_block(vect))
    } else if token.consume_reserved("if") {
        token.expect_reserved("(")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        if token.consume_reserved("else") {
            Ok(Node::new_if_else(cond, stmt, statement(token, vars)?))
        } else {
            Ok(Node::new_if(cond, stmt))
        }
    } else if token.consume_reserved("while") {
        token.expect_reserved("(")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        Ok(Node::new_while(cond, stmt))
    } else if token.consume_reserved("for") {
        token.expect_reserved("(")?;
        let init = expression(token, vars)?;
        token.expect_reserved(";")?;
        let cond = expression(token, vars)?;
        token.expect_reserved(";")?;
        let iter = expression(token, vars)?;
        token.expect_reserved(")")?;
        let stmt = statement(token, vars)?;
        Ok(Node::new_for(init, cond, iter, stmt))
    } else if token.consume_reserved("return") {
        let node = Node::new_return(expression(token, vars)?);
        token.expect_reserved(";")?;
        Ok(node)
    } else if token.next_reserved("int") {
        let node = declaration(token, vars)?;
        token.expect_reserved(";")?;
        Ok(node)
    } else {
        let node = expression(token, vars)?;
        token.expect_reserved(";")?;
        Ok(node)
    }
}

fn expression(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    assign(token, vars)
}

fn assign(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    let node = equality(token, vars)?;
    if token.consume_reserved("=") {
        Ok(Node::new_binary(
            BinaryKind::Assign,
            node,
            assign(token, vars)?,
        ))
    } else {
        Ok(node)
    }
}

fn equality(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
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

fn relational(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
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

fn add(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    let mut node = mul(token, vars)?;
    let rate = match node.kind() {
        Ok(VariableType::Pointer(t)) | Ok(VariableType::Array(t, _)) => t.size(),
        _ => 1,
    };
    loop {
        if token.consume_reserved("+") {
            let mul = mul(token, vars)?;
            let rated = if rate != 1 {
                Node::new_binary(BinaryKind::Multiply, mul, Node::Num(rate))
            } else {
                mul
            };
            node = Node::new_binary(BinaryKind::Add, node, rated);
        } else if token.consume_reserved("-") {
            let mul = mul(token, vars)?;
            let rated = if rate != 1 {
                Node::new_binary(BinaryKind::Multiply, mul, Node::Num(rate))
            } else {
                mul
            };
            node = Node::new_binary(BinaryKind::Subtract, node, rated);
        } else {
            return Ok(node);
        }
    }
}

fn mul(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    let mut node = unary(token, vars)?;
    loop {
        if token.consume_reserved("*") {
            node = Node::new_binary(BinaryKind::Multiply, node, unary(token, vars)?);
        } else if token.consume_reserved("/") {
            node = Node::new_binary(BinaryKind::Divide, node, unary(token, vars)?);
        } else {
            return Ok(node);
        }
    }
}

fn unary(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    if token.consume_reserved("+") {
        Ok(primary(token, vars)?)
    } else if token.consume_reserved("-") {
        Ok(Node::new_binary(
            BinaryKind::Subtract,
            Node::Num(0),
            primary(token, vars)?,
        ))
    } else if token.consume_reserved("*") {
        Ok(Node::new_unary(UnaryKind::Deref, unary(token, vars)?))
    } else if token.consume_reserved("&") {
        Ok(Node::new_unary(UnaryKind::Address, unary(token, vars)?))
    } else if token.consume_reserved("sizeof") {
        match unary(token, vars)?.kind() {
            Ok(t) => Ok(Node::Num(t.size())),
            Err(s) => Err(token.error(s)),
        }
    } else {
        Ok(primary(token, vars)?)
    }
}

fn primary(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
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

fn number(token: &mut TokenList, _vars: &mut VariableList) -> ParseResult {
    Ok(Node::Num(token.expect_num()?))
}

fn identify(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    if let Some(name) = token.expect_identify() {
        if token.consume_reserved("(") {
            let mut args = Vec::new();
            let mut multi = false;
            while !token.consume_reserved(")") {
                if multi {
                    token.expect_reserved(",")?;
                }
                args.push(expression(token, vars)?);
                multi = true;
            }
            return Ok(Node::FunctionCall { name, args });
        }

        let mut ofs = 0;
        for (var, t, i) in &*vars {
            ofs += i;
            if var == &name {
                return Ok(Node::LocalVariable(t.clone(), ofs));
            }
        }

        Err(token.error("宣言された変数ではありません。"))
    } else {
        Err(token.error("識別子ではありません。"))
    }
}

fn declaration(token: &mut TokenList, vars: &mut VariableList) -> ParseResult {
    token.expect_reserved("int")?;
    let (t, s) = declaration_identify(token)?;
    let mut ofs = 0;
    for (var, _, i) in &*vars {
        ofs += i;
        if *var == s {
            return Err(token.error("すでに宣言された変数です。"));
        }
    }
    if token.consume_reserved("[") {
        let size = token.expect_num()?;
        token.expect_reserved("]")?;
        let byte_size = t.size() * size;
        let ty = VariableType::Array(Box::new(t), size);
        vars.push((s, ty.clone(), byte_size));
        Ok(Node::LocalVariable(ty, ofs + byte_size))
    } else {
        let i = t.size();
        vars.push((s, t.clone(), i));
        Ok(Node::LocalVariable(t, ofs + i))
    }
}

fn declaration_identify(token: &mut TokenList) -> Result<(VariableType, String), CompileError> {
    if token.consume_reserved("*") {
        let (t, s) = declaration_identify(token)?;
        Ok((VariableType::Pointer(Box::new(t)), s))
    } else {
        match token.expect_identify() {
            Some(s) => Ok((VariableType::Int, s)),
            None => Err(token.error("宣言が変数ではありません。")),
        }
    }
}
