use parse::Node;
use parse::NodeKind;
use parse::CompareKind;

pub fn assemble(node: &Node) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    gen(node);

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

fn gen(node: &Node) {
    match node {
        Node::Num(i) => {
            println!("  push {}", i);
        },
        Node::LocalVariable(_) => {
            gen_local_variable(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        },
        Node::BinaryOperator{kind:NodeKind::Assign, left, right} => {
            gen_local_variable(&*left);
            gen(&*right);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        },
        Node::BinaryOperator{kind, left, right} => {
            gen(&*left);
            gen(&*right);
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
        },

        Node::Statements(vec) => {
            for node in vec {
                gen(node);
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