use crate::ccc::error::CompileError;

pub fn tokenize(code: &String) -> Result<TokenList, CompileError> {
    let mut vect = Vec::new();
    let codev = code.chars().collect::<Vec<char>>();
    let len = codev.len();
    let mut cur = 0;

    while cur < len {
        match codev[cur] {
            ' ' | '\n' | '\r' | '\t' => {
                cur += 1;
            }
            '+' | '-' | '*' | '/' | '(' | ')' | ';' | '{' | '}' | ',' | '&' => {
                vect.push(Token::RESERVED(codev[cur].to_string(), cur));
                cur += 1;
            }
            '0'..='9' => {
                let (lo, c) = str_to_long(code, cur);
                vect.push(Token::NUMBER(lo, cur));
                cur = c;
            }
            '=' | '!' | '<' | '>' => {
                if codev[cur + 1] == '=' {
                    vect.push(Token::RESERVED(
                        format!("{}{}", codev[cur], codev[cur + 1]),
                        cur,
                    ));
                    cur += 2;
                } else {
                    vect.push(Token::RESERVED(codev[cur].to_string(), cur));
                    cur += 1;
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let (identify, c) = get_identify(code, cur);
                vect.push(keyword_or_identify(identify, cur));
                cur = c;
            }
            _ => {
                return Err(CompileError::new(code, cur, "トークナイズ出来ません。"));
            }
        }
    }
    Ok(TokenList {
        code: code.to_string(),
        list: vect,
        pos: 0,
    })
}

fn str_to_long(code: &String, cursor: usize) -> (i64, usize) {
    let mut len = cursor;
    while len + 1 <= code.len() && code[cursor..len + 1].parse::<i64>().is_ok() {
        len += 1
    }
    (code[cursor..len].parse().unwrap(), len)
}

fn get_identify(code: &String, cursor: usize) -> (&str, usize) {
    let codev = code.chars().collect::<Vec<char>>();
    let len = codev.len();
    for now in cursor + 1..len {
        match codev[now] {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => continue,
            _ => return (&code[cursor..now], now),
        }
    }
    (&code[cursor..len], len)
}

fn keyword_or_identify(name: &str, cur: usize) -> Token {
    match name {
        "return" | "if" | "else" | "while" | "for" | "int" => {
            Token::RESERVED(name.to_string(), cur)
        }
        _ => Token::IDENTIFY(name.to_string(), cur),
    }
}

#[derive(Debug)]
enum Token {
    RESERVED(String, usize),
    IDENTIFY(String, usize),
    NUMBER(i64, usize),
}

impl Token {
    fn pos(&self) -> usize {
        match self {
            Token::RESERVED(_, p) => *p,
            Token::IDENTIFY(_, p) => *p,
            Token::NUMBER(_, p) => *p,
        }
    }
}

#[derive(Debug)]
pub struct TokenList {
    pos: usize,
    code: String,
    list: Vec<Token>,
}

impl TokenList {
    fn get(&mut self) -> Option<&Token> {
        if !self.at_eof() {
            let itm = &self.list[0];
            self.pos = itm.pos();
            Some(itm)
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<Token> {
        if !self.at_eof() {
            let itm = self.list.remove(0);
            self.pos = itm.pos();
            Some(itm)
        } else {
            None
        }
    }

    pub fn at_eof(&self) -> bool {
        self.list.len() == 0
    }
}

impl TokenList {
    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn position(&self) -> usize {
        self.pos
    }
}

impl TokenList {
    pub fn consume(&mut self, stri: &str) -> bool {
        if let Some(Token::RESERVED(ref s, _)) = self.get() {
            if s == stri {
                self.pop();
                return true;
            }
        }
        false
    }

    pub fn next_identify(&mut self) -> bool {
        matches!(self.get(), Some(Token::IDENTIFY(_, _)))
    }

    pub fn expect_num(&mut self) -> Result<i64, CompileError> {
        match self.pop() {
            Some(Token::NUMBER(i, _)) => Ok(i),
            _ => Err(CompileError::new(
                "数ではありません。",
                self.pos,
                &self.code,
            )),
        }
    }

    pub fn expect_reserved(&mut self, t: &str) -> Result<String, CompileError> {
        match self.pop() {
            Some(Token::RESERVED(s, _)) if s == t => Ok(s),
            _ => Err(CompileError::new(
                &format!("{} がありません。", t),
                self.pos,
                &self.code,
            )),
        }
    }

    pub fn expect_identify(&mut self) -> Option<String> {
        match self.pop() {
            Some(Token::IDENTIFY(s, _)) => Some(s),
            _ => None,
        }
    }
}
