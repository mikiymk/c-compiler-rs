use crate::ccc::error::CompileError;

#[derive(Debug)]
enum TokenKind {
    RESERVED(String),
    IDENTIFY(String),
    NUMBER(i64),
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    position: usize,
}

impl Token {
    pub fn new_reserved<S>(name: S, position: usize) -> Self
    where
        S: std::string::ToString,
    {
        Token {
            kind: TokenKind::RESERVED(name.to_string()),
            position,
        }
    }

    pub fn new_identify<S>(name: S, position: usize) -> Self
    where
        S: std::string::ToString,
    {
        Token {
            kind: TokenKind::IDENTIFY(name.to_string()),
            position,
        }
    }

    pub fn new_number(num: i64, position: usize) -> Self {
        Token {
            kind: TokenKind::NUMBER(num),
            position,
        }
    }
}

/// Array of tokens
#[derive(Debug)]
pub struct TokenList {
    pos: usize,
    code: String,
    list: Vec<Token>,
}

impl TokenList {
    pub fn new(code: &str, list: Vec<Token>) -> Self {
        TokenList {
            pos: 0,
            code: code.to_string(),
            list,
        }
    }

    fn get(&self) -> Option<&TokenKind> {
        if !self.at_eof() {
            let itm = &self.list[0];
            Some(&itm.kind)
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<TokenKind> {
        if !self.at_eof() {
            let itm = self.list.remove(0);
            self.pos = itm.position;
            Some(itm.kind)
        } else {
            None
        }
    }

    pub fn at_eof(&self) -> bool {
        self.list.len() == 0
    }

    pub fn error<S>(&self, err: S) -> CompileError
    where
        S: std::string::ToString,
    {
        CompileError::new(err, self.pos, &self.code)
    }

    pub fn next_reserved(&mut self, stri: &str) -> bool {
        matches!( self.get() ,Some(TokenKind::RESERVED(ref s)) if s == stri )
    }

    pub fn next_identify(&mut self) -> bool {
        matches!(self.get(), Some(TokenKind::IDENTIFY(_)))
    }

    pub fn consume_reserved(&mut self, stri: &str) -> bool {
        match self.get() {
            Some(TokenKind::RESERVED(ref s)) if s == stri => {
                self.pop();
                true
            }
            _ => false,
        }
    }

    pub fn expect_num(&mut self) -> Result<i64, CompileError> {
        match self.pop() {
            Some(TokenKind::NUMBER(i)) => Ok(i),
            _ => Err(self.error("数ではありません。")),
        }
    }

    pub fn expect_reserved(&mut self, t: &str) -> Result<String, CompileError> {
        match self.pop() {
            Some(TokenKind::RESERVED(s)) if s == t => Ok(s),
            _ => Err(self.error(format!("{} がありません。", t))),
        }
    }

    pub fn expect_identify(&mut self) -> Option<String> {
        match self.pop() {
            Some(TokenKind::IDENTIFY(s)) => Some(s),
            _ => None,
        }
    }
}
