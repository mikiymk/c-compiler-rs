pub struct CompileError {
    code: String,
    pos: usize,
    error: String,
}

impl CompileError {
    pub fn new<S1, S2>(error: S1, pos: usize, code: S2) -> Self
    where
        S1: std::string::ToString,
        S2: std::string::ToString,
    {
        CompileError {
            code: code.to_string(),
            pos,
            error: error.to_string(),
        }
    }
}

impl std::fmt::Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Compile Error")?;
        writeln!(f, "{}", self.error)?;
        writeln!(f, "{}", self.code)?;
        writeln!(f, "{}^", " ".repeat(self.pos))
    }
}
