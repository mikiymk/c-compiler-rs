use parse::Node;
use parse::NodeKind;
use parse::CompareKind;
use parse::StatementKind;

pub fn assemble(node: &Node) {
    let mut label = Label(0);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    gen(node, &mut label);

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

fn gen(node: &Node, label: &mut Label) {
    match node {
        Node::Num(i) => {
            println!("  push {}", i);
        }

        Node::LocalVariable(_) => {
            gen_local_variable(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        }

        Node::BinaryOperator{kind:NodeKind::Assign, left, right} => {
            gen_local_variable(&*left);
            gen(&*right, label);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }

        Node::BinaryOperator{kind, left, right} => {
            gen(&*left, label);
            gen(&*right, label);
            println!("  pop rdi");
            println!("  pop rax");
            match kind {
                NodeKind::Add => println!("  add rax, rdi"),
                NodeKind::Subtract => println!("  sub rax, rdi"),
                NodeKind::Multiply => println!("  imul rax, rdi"),
                NodeKind::Divide => {
                    println!("  cqo");
                    println!("  idiv rdi");
                },
                NodeKind::Compare(cmp) => {
                    println!("  cmp rax, rdi");
                    match cmp {
                        CompareKind::Equal => println!("  sete al"),
                        CompareKind::NotEqual => println!("  setne al"),
                        CompareKind::LessThan => println!("  setl al"),
                        CompareKind::LessEqual => println!("  setle al"),
                    }
                    println!("  movzb rax, al");
                },
                NodeKind::Assign => unreachable!(),
            }
            println!("  push rax");
        }

        Node::Statement(kind) => {
            match kind {
                StatementKind::Return(expr) => {
                    gen(&*expr, label);
                    println!("  pop rax");
                    println!("  mov rsp, rbp");
                    println!("  pop rbp");
                    println!("  ret");
                }

                StatementKind::If{condition, t_statement} => {
                    let l = label.get();
                    gen(&*condition, label);
                    println!("  pop rax");
                    println!("  cmp rax, 0");
                    println!("  je .Lend{}", l);
                    gen(&*t_statement, label);
                    println!(".Lend{}:", l);
                }

                StatementKind::IfElse{condition, t_statement, f_statement} => {
                    let lelse = label.get();
                    let lend = label.get();
                    gen(&*condition, label);
                    println!("  pop rax");
                    println!("  cmp rax, 0");
                    println!("  je .Lelse{}", lelse);
                    gen(&*t_statement, label);
                    println!("  jmp .Lend{}", lend);
                    println!(".Lelse{}:", lelse);
                    gen(&*f_statement, label);
                    println!(".Lend{}:", lend);
                }

                StatementKind::While{condition, statement} => {
                    let lbegin = label.get();
                    let lend = label.get();
                    println!(".Lbegin{}:", lbegin);
                    gen(&*condition, label);
                    println!("  pop rax");
                    println!("  cmp rax, 0");
                    println!("  je .Lend{}", lend);
                    gen(&*statement, label);
                    println!("  jmp .Lbegin{}", lbegin);
                    println!(".Lend{}:", lend);
                }

                StatementKind::For{init, condition, iteration, statement} => {
                    let lbegin = label.get();
                    let lend = label.get();
                    gen(&*init, label);
                    println!(".Lbegin{}:", lbegin);
                    gen(&*condition, label);
                    println!("  pop rax");
                    println!("  cmp rax, 0");
                    println!("  je .Lend{}", lend);
                    gen(&*statement, label);
                    gen(&*iteration, label);
                    println!("  jmp .Lbegin{}", lbegin);
                    println!(".Lend{}:", lend);
                }
            }
        }

        Node::Statements(vec) => {
            for node in vec {
                gen(node, label);
                println!("  pop rax");
            }
        }
    }
}

fn gen_local_variable(node: &Node) {
    if let Node::LocalVariable(offset) = node {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
    } else {
        eprintln!("代入の左辺値が変数ではありません。");
    }
}

struct Label(u64);

impl Label {
    fn get(&mut self) -> u64 {
        let a = self.0;
        self.0 += 1;
        a
    }
}