mod code_generator;
mod error;
mod lexer;
mod parser;

pub fn compile(code: &str) -> Result<(), error::CompileError> {
    use code_generator::code_generate;
    let mut tokens = lexer::analyze(&code)?;
    let parsed = parser::analyze(&mut tokens)?;
    eprintln!("{:?}", parsed);
    code_generate(&parsed);
    Ok(())
}
