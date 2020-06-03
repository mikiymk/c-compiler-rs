use crate::ccc::parse::node::{CompareKind, Node, NodeKind, StatementKind, UnaryKind};

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

pub fn code_generate(node: &Node) {
    let mut label = Label::new();

    println!(".intel_syntax noprefix");
    println!(".global main");

    gen(node, &mut label);
}

fn gen(node: &Node, label: &mut Label) {
    match node {
        Node::Num(i) => {
            label.comment("number literal");
            label.push(i);
        }

        Node::LocalVariable(_, _) => {
            label.comment("local var until 'push rax'");
            gen_local_variable(node, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }

        Node::BinaryOperator {
            kind: NodeKind::Assign,
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
            label.mov("[rax]", "rdi");
            label.push("rdi");
        }

        Node::BinaryOperator { kind, left, right } => {
            gen(&*left, label);
            gen(&*right, label);
            label.pop("rdi");
            label.pop("rax");
            match kind {
                NodeKind::Add => label.add("rax", "rdi"),
                NodeKind::Subtract => label.sub("rax", "rdi"),
                NodeKind::Multiply => label.imul("rax", "rdi"),
                NodeKind::Divide => {
                    label.cqo();
                    label.idiv("rdi");
                }
                NodeKind::Compare(cmp) => {
                    label.cmp("rax", "rdi");
                    match cmp {
                        CompareKind::Equal => label.sete("al"),
                        CompareKind::NotEqual => label.setne("al"),
                        CompareKind::LessThan => label.setl("al"),
                        CompareKind::LessEqual => label.setle("al"),
                    }
                    label.movzx("rax", "al");
                }
                NodeKind::Assign => unreachable!(),
            }
            label.push("rax");
        }

        Node::UnaryOperator {
            kind: UnaryKind::Address,
            expression,
        } => {
            gen_local_variable(expression, label);
        }

        Node::UnaryOperator {
            kind: UnaryKind::Deref,
            expression,
        } => {
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
            let len = args.len();
            if len == 6 {
                label.pop("r9");
            }
            if len >= 5 {
                label.pop("r8");
            }
            if len >= 4 {
                label.pop("rcx");
            }
            if len >= 3 {
                label.pop("rdx");
            }
            if len >= 2 {
                label.pop("rsi");
            }
            if len >= 1 {
                label.pop("rdi");
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
                gen(&*condition, label);
                label.pop("rax");
                label.cmp("rax", "0");
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
                gen(&*condition, label);
                label.pop("rax");
                label.cmp("rax", "0");
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
                gen(&*condition, label);
                label.pop("rax");
                label.cmp("rax", "0");
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
                gen(&*condition, label);
                label.pop("rax");
                label.cmp("rax", "0");
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
                gen(node, label);
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

            let len = args.len();
            if len >= 1 {
                label.push("rdi");
            }
            if len >= 2 {
                label.push("rsi");
            }
            if len >= 3 {
                label.push("rdx");
            }
            if len >= 4 {
                label.push("rcx");
            }
            if len >= 5 {
                label.push("r8");
            }
            if len == 6 {
                label.push("r9");
            }

            label.sub("rsp", 208);

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
            gen_local_variable(expression, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }

        _ => eprintln!("左辺値が代入可能ではありません。"),
    }
}

/**
 * ローカルのユニークなラベル名のための構造体
 */
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
        self.push_count += 8;
    }

    fn pop<T>(&mut self, src: T)
    where
        T: std::fmt::Display,
    {
        println!("  pop {}", src);
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
