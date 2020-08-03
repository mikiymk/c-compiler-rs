pub mod code_generator;
pub mod error;
pub mod parse;

pub fn compile(program: &str) -> Result<(), error::CompileError> {
    use code_generator::code_generate;
    use parse::parse;
    let parsed = parse(program)?;
    eprintln!("{:?}", parsed);
    code_generate(&parsed);
    Ok(())
}
