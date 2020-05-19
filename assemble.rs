use parse::Node;
use parse::NodeKind;
use parse::CompareKind;

pub fn assemble(node: &Node) {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}

fn gen(node: &Node) {
    match node {
        Node::Num(i) => {
            println!("  push {}", i)
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
                }
            }
            println!("  push rax");
        },
    }
}