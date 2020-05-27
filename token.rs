use parse::ParseError;

pub fn tokenize(code: &String) -> Result<TokenList, TokenizeError> {
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
                vect.push(Token::RESERVED(codev[cur].to_string()));
                cur += 1;
            }
            '0' ..= '9' => {
                let (lo, c) = str_to_long(code, cur);
                vect.push(Token::NUMBER(lo));
                cur = c;
            }
            '=' | '!' | '<' | '>' => {
                if codev[cur + 1] == '=' {
                    vect.push(Token::RESERVED(format!("{}{}", codev[cur], codev[cur + 1])));
                    cur += 2;
                } else {
                    vect.push(Token::RESERVED(codev[cur].to_string()));
                    cur += 1;
                }
            }
            'a' ..= 'z' | 'A' ..= 'Z' => {
                let (identify, c) = get_identify(code, cur);
                vect.push(keyword_or_identify(identify));
                cur = c;
            }
            _ => {
                return Err(error_at(code, cur, "トークナイズ出来ません。"));
            }
        }
    }
    Ok(TokenList{ list: vect, })
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
    for now in cursor + 1 .. len {
        match codev[now] {
            'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' => continue,
            _ => return (&code[cursor..now], now),
        }
    }
    (&code[cursor..len], len)
}

fn keyword_or_identify(name: &str) -> Token {
    match name {
        "return" | "if" | "else" | "while" | "for" | "int" => Token::RESERVED(name.to_string()),
        _ => Token::IDENTIFY(name.to_string()),
    }
}

fn error_at(code: &str, pos: usize, error: &str) -> TokenizeError {
    eprintln!("{}", error);
    eprintln!("{}", code);
    eprintln!("{}^", " ".repeat(pos));
    TokenizeError
}

#[derive(Debug)]
enum Token {
    RESERVED(String),
    IDENTIFY(String),
    NUMBER(i64),
}

#[derive(Debug)]
pub struct TokenList {
    list : Vec<Token>,
}

pub struct TokenizeError;


impl TokenList {
    fn get(&self) -> Option<&Token> {
        if !self.at_eof() {
            Some(&self.list[0])
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<Token> {
        if !self.at_eof() {
            Some(self.list.remove(0))
        } else {
            None
        }
    }

    pub fn at_eof(&self) -> bool {
        self.list.len() == 0
    }
}

impl TokenList {
    pub fn consume(&mut self, stri: &str) -> bool {
        if let Some(Token::RESERVED(ref s)) = self.get() {
            if s == stri {
                self.pop();
                return true;
            }
        }
        false
    }

    pub fn next_reserved(&self, stri: &str) -> bool {
        matches!(self.get(), Some(Token::RESERVED(s)) if s == stri)
    }

    pub fn next_identify(&self) -> bool {
        matches!(self.get(), Some(Token::IDENTIFY(_)))
    }

    pub fn expect_num(&mut self) -> Result<i64, ParseError> {
        match self.pop() {
            Some(Token::NUMBER(i)) => Ok(i),
            _ => Err(ParseError::new("数ではありません。".to_string())),
        }
    }

    pub fn expect_reserved(&mut self, t: &str) -> Result<String, ParseError> {
        match self.pop() {
            Some(Token::RESERVED(s)) if s == t => Ok(s),
            _ => Err(ParseError::new(format!("{} がありません。", t))),
        }
    }

    pub fn expect_identify(&mut self) -> Option<String> {
        match self.pop() {
            Some(Token::IDENTIFY(s)) => Some(s),
            _ => None,
        }
    }
}

impl std::fmt::Debug for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TokenizeError")
    }
}