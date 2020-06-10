use crate::ccc::parse::node::{BinaryKind, CompareKind, Node, StatementKind, UnaryKind};

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
    let mut label = Label::new();

    println!(".intel_syntax noprefix");
    println!(".global main");

    gen(node, &mut label);
}

fn gen(node: &Node, label: &mut Label) -> () {
    match node {
        Node::Num(i) => {
            label.push(i);
        }

        Node::LocalVariable(super::parse::node::VariableType::Array(_, _), _) => {
            label.comment("local var array start");
            gen_local_variable(node, label);
            label.comment("local var array end");
        }

        Node::LocalVariable(_, _) => {
            label.comment("local var start");
            gen_local_variable(node, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
            label.comment("local var end");
        }

        Node::BinaryOperator {
            kind: BinaryKind::Assign,
            left,
            right,
        } => {
            label.comment("assign left");
            gen_local_variable(&*left, label);
            label.comment("assign right");
            gen(&*right, label);

            label.comment("assign body");
            label.pop("rdi");
            label.pop("rax");
            match left.kind() {
                Ok(t) if t.size() == 4 => {
                    label.mov("[rax]", "edi");
                }
                Ok(_) => {
                    label.mov("[rax]", "rdi");
                }
                Err(e) => eprintln!("{}", e),
            }
            label.push("rdi");
            label.comment("assign end");
        }

        Node::BinaryOperator { kind, left, right } => {
            label.comment("binary left");
            gen(&*left, label);
            label.comment("binary right");
            gen(&*right, label);

            label.comment("binary body");
            label.pop("rdi");
            label.pop("rax");
            let lk = left.kind();
            let rk = right.kind();
            let (rax, rdi) = match (lk, rk) {
                (Ok(l), Ok(r)) if l.size() == 4 && r.size() == 4 => ("eax", "edi"),
                _ => ("rax", "rdi"),
            };
            match kind {
                BinaryKind::Add => label.add(rax, rdi),
                BinaryKind::Subtract => label.sub(rax, rdi),
                BinaryKind::Multiply => label.imul(rax, rdi),
                BinaryKind::Divide => {
                    label.cqo();
                    label.idiv(rdi);
                }
                BinaryKind::Compare(cmp) => {
                    label.cmp(rax, rdi);
                    match cmp {
                        CompareKind::Equal => label.sete("al"),
                        CompareKind::NotEqual => label.setne("al"),
                        CompareKind::LessThan => label.setl("al"),
                        CompareKind::LessEqual => label.setle("al"),
                    }
                    label.movzx("rax", "al");
                }
                BinaryKind::Assign => unreachable!(),
            }
            label.push("rax");
            label.comment("binary end");
        }
        Node::UnaryOperator {
            kind: UnaryKind::Address,
            expression,
        } => {
            label.comment("address");
            gen_local_variable(expression, label);
        }
        Node::UnaryOperator {
            kind: UnaryKind::Deref,
            expression,
        } => {
            label.comment("deref");
            gen(expression, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }

        Node::FunctionCall { name, args } => {
            label.comment("call function");
            for arg in args {
                gen(arg, label);
            }
            for i in (0..args.len()).rev() {
                label.pop(register::ARGS_REGISTER[i][0]);
            }
            label.call(name);
            label.push("rax");
        }

        Node::Statement(kind) => match kind {
            StatementKind::Return(expr) => {
                label.comment("return expression");
                gen(&*expr, label);
                label.comment("return body");
                label.pop("rax");
                label.mov("rsp", "rbp");
                label.pop("rbp");
                label.ret();
            }

            StatementKind::If {
                condition,
                t_statement,
            } => {
                label.comment("if statement");
                let l = label.get();
                gen_condition(&*condition, label);
                label.je(l);
                gen(&*t_statement, label);
                label.l_label(l);
            }

            StatementKind::IfElse {
                condition,
                t_statement,
                f_statement,
            } => {
                label.comment("if else statement");
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

            StatementKind::While {
                condition,
                statement,
            } => {
                label.comment("while statement");
                let lbegin = label.get();
                let lend = label.get();
                label.l_label(lbegin);
                gen_condition(&*condition, label);
                label.je(lend);
                gen(&*statement, label);
                label.jmp(lbegin);
                label.l_label(lend);
            }

            StatementKind::For {
                init,
                condition,
                iteration,
                statement,
            } => {
                label.comment("for statement");
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

            StatementKind::Block { statements } => {
                label.comment("block statements");
                for statement in statements {
                    label.comment("block statement");
                    gen(statement, label);
                    label.pop("rax")
                }
            }
        },

        Node::Program(vec) => {
            for node in vec {
                let mut label = Label::new();
                gen(node, &mut label);
            }
        }

        Node::Function {
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

mod register {
    type RegisterAlias = [&'static str; 4];
    // const RAX: RegisterAlias = ["RAX", "EAX", "AX", "AL"];
    const RDI: RegisterAlias = ["RDI", "EDI", "DI", "DIL"];
    const RSI: RegisterAlias = ["RSI", "ESI", "SI", "SIL"];
    const RDX: RegisterAlias = ["RDX", "EDX", "DX", "DL"];
    const RCX: RegisterAlias = ["RCX", "ECX", "CX", "CL"];
    // const RBP: RegisterAlias = ["RBP", "EBP", "BP", "BPL"];
    // const RSP: RegisterAlias = ["RSP", "ESP", "SP", "SPL"];
    // const RBX: RegisterAlias = ["RBX", "EBX", "BX", "BL"];
    const R8: RegisterAlias = ["R8", "R8D", "R8W", "R8B"];
    const R9: RegisterAlias = ["R9", "R9D", "R9W", "R9B"];
    // const R10: RegisterAlias = ["R10", "R10D", "R10W", "R10B"];
    // const R11: RegisterAlias = ["R11", "R11D", "R11W", "R11B"];
    // const R12: RegisterAlias = ["R12", "R12D", "R12W", "R12B"];
    // const R13: RegisterAlias = ["R13", "R13D", "R13W", "R13B"];
    // const R14: RegisterAlias = ["R14", "R14D", "R14W", "R14B"];
    // const R15: RegisterAlias = ["R15", "R15D", "R15W", "R15B"];

    /// 関数の引数に使うレジスタ。６個までの引数に対応。
    pub const ARGS_REGISTER: [RegisterAlias; 6] = [RDI, RSI, RDX, RCX, R8, R9];
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
