use super::parse::node::VariableType::Array;
use super::parse::node::{BinaryKind, CompareKind, Node, StatementKind, UnaryKind};

mod register;

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

pub fn code_generate(node: &Node) -> () {
    println!(".intel_syntax noprefix");
    println!(".global main");

    gen(node, &mut Label::new());
}

fn gen(node: &Node, label: &mut Label) -> () {
    use Node::{
        BinaryOperator, Function, FunctionCall, LocalVariable, Num, Program, Statement,
        UnaryOperator,
    };
    match node {
        Num(i) => label.push(i),

        LocalVariable(Array(_, _), _) => gen_local_variable(node, label),

        LocalVariable(_, _) => {
            gen_local_variable(node, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }

        BinaryOperator { kind, left, right } => generate_binary(label, kind, left, right),
        UnaryOperator { kind, expression } => generate_unary(label, kind, expression),

        FunctionCall { name, args } => {
            for arg in args {
                gen(arg, label);
            }
            for i in (0..args.len()).rev() {
                label.pop(register::ARGS_REGISTER[i][0]);
            }
            label.call(name);
            label.push("rax");
        }

        Statement(kind) => generate_statement(label, kind),
        Program(vec) => {
            for node in vec {
                gen(node, &mut Label::new());
            }
        }

        Function {
            name,
            args,
            statement,
        } => {
            label.f_label(name);
            label.push("rbp");
            label.mov("rbp", "rsp");
            label.sub("rsp", 208);

            for i in 0..args.len() {
                gen_parameter(&args[i], label, i);
            }

            gen(statement, label);

            label.pop("rax");
            label.mov("rsp", "rbp");
            label.pop("rbp");
            label.ret();
        }
    }
}

fn generate_binary(label: &mut Label, kind: &BinaryKind, left: &Node, right: &Node) {
    use BinaryKind::{Add, Assign, Compare, Divide, Multiply, Subtract};
    match kind {
        Assign => {
            gen_local_variable(left, label);
            gen(right, label);

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
            gen(left, label);
            gen(right, label);

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

fn generate_unary(label: &mut Label, kind: &UnaryKind, expression: &Node) {
    match kind {
        UnaryKind::Address => gen_local_variable(expression, label),
        UnaryKind::Deref => {
            gen(expression, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }
    }
}

fn generate_statement(label: &mut Label, kind: &StatementKind) {
    use StatementKind::{Block, For, If, IfElse, Return, While};
    match kind {
        Return(expr) => {
            label.comment("return expression");
            gen(&*expr, label);
            label.comment("return body");
            label.pop("rax");
            label.mov("rsp", "rbp");
            label.pop("rbp");
            label.ret();
        }

        If {
            condition,
            t_statement,
        } => {
            let l = label.get();
            gen_condition(&*condition, label);
            label.je(l);
            gen(&*t_statement, label);
            label.l_label(l);
        }

        IfElse {
            condition,
            t_statement,
            f_statement,
        } => {
            let lelse = label.get();
            let lend = label.get();
            gen_condition(&*condition, label);
            label.je(lelse);
            gen(&*t_statement, label);
            label.jmp(lend);
            label.l_label(lelse);
            gen(&*f_statement, label);
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
            gen(&*statement, label);
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
            gen(&*init, label);
            label.l_label(lbegin);
            gen_condition(&*condition, label);
            label.je(lend);
            gen(&*statement, label);
            gen(&*iteration, label);
            label.jmp(lbegin);
            label.l_label(lend);
        }

        Block { statements } => {
            for statement in statements {
                gen(statement, label);
                label.pop("rax")
            }
        }
    }
}

fn gen_local_variable(node: &Node, label: &mut Label) {
    match node {
        Node::LocalVariable(_, offset) => {
            label.mov("rax", "rbp");
            label.sub("rax", offset);
            label.push("rax");
        }

        Node::UnaryOperator {
            kind: UnaryKind::Deref,
            expression,
        } => {
            gen(expression, label);
        }

        _ => eprintln!("左辺値が代入可能ではありません。"),
    }
}

fn gen_condition(condition: &Node, label: &mut Label) {
    gen(&*condition, label);
    label.pop("rax");
    let s = match condition.kind() {
        Ok(k) if k.size() == 4 => "eax",
        _ => "rax",
    };
    label.cmp(s, "0");
}

fn gen_parameter(node: &Node, label: &mut Label, parameter_index: usize) {
    gen_local_variable(&*node, label);
    label.pop("rax");

    if let Ok(t) = node.kind() {
        match t.size() {
            4 => label.mov("[rax]", register::ARGS_REGISTER[parameter_index][1]),
            _ => label.mov("[rax]", register::ARGS_REGISTER[parameter_index][0]),
        }
    }
}

/// ローカルのユニークなラベル名のための構造体
struct Label {
    label_count: u64,
    push_count: i64,
}

impl Label {
    fn new() -> Self {
        Label {
            label_count: 0,
            push_count: 0,
        }
    }

    fn get(&mut self) -> u64 {
        let a = self.label_count;
        self.label_count += 1;
        a
    }
}

impl Label {
    fn comment<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("# {}", src);
    }

    /// push src
    ///
    /// 現在のRSP位置にsrcを書き込み、RSPを8下げる。
    fn push<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  push {}", src);
        println!(
            "# push count {} + 8 => {}",
            self.push_count,
            self.push_count + 8
        );
        self.push_count += 8;
    }

    /// pop src
    ///
    /// 現在のRSP位置の64bitをsrcに読み込み、RSPを8上げる。
    fn pop<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  pop {}", src);
        println!(
            "# push count {} - 8 => {}",
            self.push_count,
            self.push_count - 8
        );
        self.push_count -= 8;
    }

    fn mov<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  mov {}, {}", dst, src);
    }

    fn movzx<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  movzx {}, {}", dst, src);
    }

    fn add<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  add {}, {}", dst, src);
    }

    fn sub<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  sub {}, {}", dst, src);
    }

    fn imul<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  imul {}, {}", dst, src);
    }

    fn cqo(&mut self) {
        println!("  cqo");
    }

    fn idiv<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  idiv {}", src);
    }

    fn cmp<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        println!("  cmp {}, {}", dst, src);
    }

    fn sete<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  sete {}", src);
    }

    fn setne<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  setne {}", src);
    }

    fn setl<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  setl {}", src);
    }

    fn setle<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  setle {}", src);
    }

    fn ret(&mut self) {
        println!("  ret");
    }

    fn jmp<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  jmp .L{}", src);
    }

    fn je<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  je .L{}", src);
    }

    fn call<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        let n = 16 - self.push_count % 16;
        if n != 16 {
            self.sub("rsp", n);
        }
        println!("  call {}", src);
        if n != 16 {
            self.add("rsp", n);
        }
    }

    fn f_label<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("{}:", src);
    }

    fn l_label<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!(".L{}:", src);
    }
}
