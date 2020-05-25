use parse::Node;
use parse::NodeKind;
use parse::CompareKind;
use parse::StatementKind;

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
    println!("main:");

    label.push("rbp");
    label.mov("rbp", "rsp");
    label.sub("rsp", 208);

    gen(node, &mut label);

    label.mov("rsp", "rbp");
    label.pop("rbp");
    label.ret();
}

fn gen(node: &Node, label: &mut Label) {
    match node {
        Node::Num(i) => {
            label.push(i);
        }

        Node::LocalVariable(_) => {
            gen_local_variable(node, label);
            label.pop("rax");
            label.mov("rax", "[rax]");
            label.push("rax");
        }

        Node::BinaryOperator{kind: NodeKind::Assign, left, right} => {
            gen_local_variable(&*left, label);
            gen(&*right, label);

            label.pop("rdi");
            label.pop("rax");
            label.mov("[rax]", "rdi");
            label.push("rdi");
        }

        Node::BinaryOperator{kind, left, right} => {
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
                },
                NodeKind::Compare(cmp) => {
                    label.cmp("rax", "rdi");
                    match cmp {
                        CompareKind::Equal => label.sete("al"),
                        CompareKind::NotEqual => label.setne("al"),
                        CompareKind::LessThan => label.setl("al"),
                        CompareKind::LessEqual => label.setle("al"),
                    }
                    label.movzx("rax", "al");
                },
                NodeKind::Assign => unreachable!(),
            }
            label.push("rax");
        }

        Node::FunctionCall{name} => {
            label.call(name);
            label.push("rax");
        }

        Node::Statement(kind) => {
            match kind {
                StatementKind::Return(expr) => {
                    gen(&*expr, label);
                    label.pop("rax");
                    label.mov("rsp", "rbp");
                    label.pop("rbp");
                    label.ret();
                }

                StatementKind::If{condition, t_statement} => {
                    let l = label.get();
                    gen(&*condition, label);
                    label.pop("rax");
                    label.cmp("rax", "0");
                    label.je(l);
                    gen(&*t_statement, label);
                    label.l_label(l);
                }

                StatementKind::IfElse{condition, t_statement, f_statement} => {
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

                StatementKind::While{condition, statement} => {
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

                StatementKind::For{init, condition, iteration, statement} => {
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

                StatementKind::Block{statements} => {
                    for statement in statements {
                        gen(statement, label);
                        label.pop("rax")
                    }
                }
            }
        }

        Node::Statements(vec) => {
            for node in vec {
                gen(node, label);
                label.pop("rax")
            }
        }
    }
}

fn gen_local_variable(node: &Node, label: &mut Label) {
    if let Node::LocalVariable(offset) = node {
        label.mov("rax", "rbp");
        label.sub("rax", offset);
        label.push("rax");
    } else {
        eprintln!("代入の左辺値が変数ではありません。");
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
    fn push<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  push {}", src);
        self.push_count += 1;
    }

    fn pop<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  pop {}", src);
        self.push_count -= 1;
    }

    fn mov<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display
    {
        println!("  mov {}, {}", dst, src);
    }

    fn movzx<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display
    {
        println!("  movzx {}, {}", dst, src);
    }

    fn add<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display
    {
        println!("  add {}, {}", dst, src);
    }

    fn sub<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display
    {
        println!("  sub {}, {}", dst, src);
    }

    fn imul<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display
    {
        println!("  imul {}, {}", dst, src);
    }

    fn cqo(&mut self) {
        println!("  cqo");
    }

    fn idiv<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  idiv {}", src);
    }

    fn cmp<T, U>(&mut self, dst: T, src: U)
    where
        T: std::fmt::Display,
        U: std::fmt::Display
    {
        println!("  cmp {}, {}", dst, src);
    }

    fn sete<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  sete {}", src);
    }

    fn setne<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  setne {}", src);
    }

    fn setl<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  setl {}", src);
    }

    fn setle<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  setle {}", src);
    }

    fn ret(&mut self) {
        println!("  ret");
    }

    fn jmp<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  jmp .L{}", src);
    }

    fn je<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!("  je .L{}", src);
    }

    fn call<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        // if self.push_count % 2 == 1 {
        //     self.sub("rsp", 8);
        // }
        println!("  call {}", src);
        // if self.push_count % 2 == 1 {
        //     self.add("rsp", 8);
        // }
    }

    fn l_label<T>(&mut self, src: T)
    where
        T: std::fmt::Display
    {
        println!(".L{}:", src);
    }
}