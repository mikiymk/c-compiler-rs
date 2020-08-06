mod label;
mod register;

use crate::ccc::parser::node::{
    BinaryKind, CompareKind, Expression, Function, Program, Statement, UnaryKind, Variable,
    VariableType::Array,
};
use label::Label;

/*

汎用レジスタ
AX  アキュムレータ
BX  ベースレジスタ
CX  カウントレジスタ
DX  データレジスタ
->0X (16bit) = 0H (8bit) | 0L (8bit)
上位8bitと下位8bitで名前が別れてる?
インデックスレジスタ
SI  ソースインデックス
DI  ディスティネーションインデックス
特殊レジスタ
BP  ベースポインタ
SP  スタックポインタ
IP  インストラクションポインタ
セグメントレジスタ
CS  コードセグメント
DS  データセグメント
ES  エクストラセグメント
SS  スタックセグメント
フラグレジスタ

MOV dst, src
dstの領域にsrcの値を書き込む
srcは変更されない

*/

pub fn code_generate(program: &Program) -> () {
    println!(".intel_syntax noprefix");
    println!(".global main");

    generate_program(program, &mut Label::new());
}

fn generate_program(program: &Program, label: &mut Label) -> () {
    for code in program.codes() {
        generate_function(code, label);
    }
}

fn generate_function(function: &Function, label: &mut Label) -> () {
    label.f_label(function.name());
    label.push("rbp");
    label.mov("rbp", "rsp");
    label.sub("rsp", 208);

    let arguments = function.arguments();
    for i in 0..arguments.len() {
        gen_parameter(&arguments[i], label, i);
    }

    let statements = function.statements();
    for statement in statements {
        generate_statement(statement, label);
    }

    label.pop("rax");
    label.mov("rsp", "rbp");
    label.pop("rbp");
    label.ret();
}

fn generate_statement(node: &Statement, label: &mut Label) -> () {
    use Statement::{Block, Declaration, Expression, For, If, IfElse, Return, While};
    match node {
        Return(expr) => {
            generate_expression(expr, label);
            label.pop("rax");
            label.mov("rsp", "rbp");
            label.pop("rbp");
            label.ret();
        }

        Declaration(var) => generate_variable(var, label),
        Expression(expr) => generate_expression(expr, label),

        If {
            condition,
            true_statement,
        } => {
            let l = label.get();
            gen_condition(&*condition, label);
            label.je(l);
            generate_statement(&*true_statement, label);
            label.l_label(l);
        }

        IfElse {
            condition,
            true_statement,
            false_statement,
        } => {
            let lelse = label.get();
            let lend = label.get();
            gen_condition(&*condition, label);
            label.je(lelse);
            generate_statement(&*true_statement, label);
            label.jmp(lend);
            label.l_label(lelse);
            generate_statement(&*false_statement, label);
            label.l_label(lend);
        }

        While {
            condition,
            statement,
        } => {
            let lbegin = label.get();
            let lend = label.get();
            label.l_label(lbegin);
            gen_condition(&*condition, label);
            label.je(lend);
            generate_statement(&*statement, label);
            label.jmp(lbegin);
            label.l_label(lend);
        }

        For {
            init,
            condition,
            iteration,
            statement,
        } => {
            let lbegin = label.get();
            let lend = label.get();
            generate_expression(&*init, label);
            label.l_label(lbegin);
            gen_condition(&*condition, label);
            label.je(lend);
            generate_statement(&*statement, label);
            generate_expression(&*iteration, label);
            label.jmp(lbegin);
            label.l_label(lend);
        }

        Block { statements } => {
            for statement in statements {
                generate_statement(statement, label);
                label.pop("rax")
            }
        }
    }
}

fn generate_expression(node: &Expression, label: &mut Label) -> () {
    use Expression::{BinaryOperator, FunctionCall, LocalVariable, Num, UnaryOperator};
    match node {
        Num(i) => label.push(i),

        LocalVariable(variable) => match variable.var_type() {
            Array(_, _) => gen_local_variable(node, label),

            _ => {
                gen_local_variable(node, label);
                label.pop("rax");
                label.mov("rax", "[rax]");
                label.push("rax");
            }
        },

        BinaryOperator { kind, left, right } => generate_binary(label, kind, left, right),
        UnaryOperator { kind, expression } => generate_unary(label, kind, expression),

        FunctionCall { name, args } => {
            for arg in args {
                generate_expression(arg, label);
            }
            for i in (0..args.len()).rev() {
                label.pop(register::ARGS_REGISTER[i][0]);
            }
            label.call(name);
            label.push("rax");
        }
    }
}

fn generate_binary(label: &mut Label, kind: &BinaryKind, left: &Expression, right: &Expression) {
    use BinaryKind::{Add, Assign, Compare, Divide, Multiply, Subtract};
    match kind {
        Assign => {
            gen_local_variable(left, label);
            generate_expression(right, label);

            label.pop("rdi");
            label.pop("rax");
            match left.kind() {
                Ok(t) if t.size() == 4 => label.mov("[rax]", "edi"),
                Ok(_) => label.mov("[rax]", "rdi"),
                Err(e) => eprintln!("{}", e),
            }
            label.push("rdi");
        }

        kind => {
            generate_expression(left, label);
            generate_expression(right, label);

            label.pop("rdi");
            label.pop("rax");
            let lk = left.kind();
            let rk = right.kind();
            let (rax, rdi) = match (lk, rk) {
                (Ok(l), Ok(r)) if l.size() == 4 && r.size() == 4 => ("eax", "edi"),
                _ => ("rax", "rdi"),
            };
            match kind {
                Add => label.add(rax, rdi),
                Subtract => label.sub(rax, rdi),
                Multiply => label.imul(rax, rdi),
                Divide => {
                    label.cqo();
                    label.idiv(rdi);
                }
                Compare(cmp) => {
                    label.cmp(rax, rdi);
                    use CompareKind::{Equal, LessEqual, LessThan, NotEqual};
                    match cmp {
                        Equal => label.sete("al"),
                        NotEqual => label.setne("al"),
                        LessThan => label.setl("al"),
                        LessEqual => label.setle("al"),
                    }
                    label.movzx("rax", "al");
                }
                Assign => unreachable!(),
            }
            label.push("rax");
        }
    }
}

fn generate_unary(label: &mut Label, kind: &UnaryKind, expression: &Expression) {
    match kind {
        UnaryKind::Address => gen_local_variable(expression, label),
        UnaryKind::Deref => {
            generate_expression(expression, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }
    }
}

fn gen_local_variable(node: &Expression, label: &mut Label) {
    match node {
        Expression::LocalVariable(var) => generate_variable(var, label),

        Expression::UnaryOperator {
            kind: UnaryKind::Deref,
            expression,
        } => {
            generate_expression(expression, label);
        }

        _ => eprintln!("左辺値が代入可能ではありません。"),
    }
}

fn generate_variable(node: &Variable, label: &mut Label) {
    label.mov("rax", "rbp");
    label.sub("rax", node.offset());
    label.push("rax");
}

fn gen_condition(condition: &Expression, label: &mut Label) {
    generate_expression(&*condition, label);
    label.pop("rax");
    let s = match condition.kind() {
        Ok(k) if k.size() == 4 => "eax",
        _ => "rax",
    };
    label.cmp(s, "0");
}

fn gen_parameter(node: &Variable, label: &mut Label, parameter_index: usize) {
    generate_variable(node, label);
    label.pop("rax");

    match node.var_type().size() {
        4 => label.mov("[rax]", register::ARGS_REGISTER[parameter_index][1]),
        _ => label.mov("[rax]", register::ARGS_REGISTER[parameter_index][0]),
    }
}
