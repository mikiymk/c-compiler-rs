use super::token::{Token, TokenList};
use crate::ccc::error::CompileError;

pub fn tokenize(code: &str) -> Result<TokenList, CompileError> {
    let mut vect = Vec::new();
    let codev = code.chars().collect::<Vec<char>>();
    let len = codev.len();
    let mut cur = 0;

    while cur < len {
        match codev[cur] {
            ' ' | '\n' | '\r' | '\t' => {
                cur += 1;
            }
            '/' => {
                if codev[cur + 1] == '/' {
                    let mut c = cur + 2;
                    while codev[c] != '\n' {
                        c = c + 1;
                    }
                    cur = c;
                } else if codev[cur + 1] == '*' {
                    let mut c = cur + 3;
                    while codev[c - 1] != '*' && codev[c] != '/' {
                        c = c + 1;
                    }
                    cur = c;
                } else {
                    vect.push(Token::new_reserved(codev[cur], cur));
                    cur += 1;
                }
            }

            '+' | '-' | '*' | '(' | ')' | ';' | '{' | '}' | ',' | '&' | '[' | ']' => {
                vect.push(Token::new_reserved(codev[cur], cur));
                cur += 1;
            }
            '0'..='9' => {
                let (lo, c) = str_to_long(code, cur);
                vect.push(Token::new_number(lo, cur));
                cur = c;
            }
            '=' | '!' | '<' | '>' => {
                if codev[cur + 1] == '=' {
                    vect.push(Token::new_reserved(
                        format!("{}{}", codev[cur], codev[cur + 1]),
                        cur,
                    ));
                    cur += 2;
                } else {
                    vect.push(Token::new_reserved(codev[cur], cur));
                    cur += 1;
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let (identify, c) = get_identify(code, cur);
                vect.push(keyword_or_identify(identify, cur));
                cur = c;
            }
            _ => {
                return Err(CompileError::new("トークナイズ出来ません。", cur, code));
            }
        }
    }
    Ok(TokenList::new(code, vect))
}

fn str_to_long(code: &str, cursor: usize) -> (i64, usize) {
    let mut len = cursor;
    while len + 1 <= code.len() && code[cursor..len + 1].parse::<i64>().is_ok() {
        len += 1
    }
    (code[cursor..len].parse().unwrap(), len)
}

fn get_identify(code: &str, cursor: usize) -> (&str, usize) {
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
        "return" | "if" | "else" | "while" | "for" | "int" | "sizeof" => {
            Token::new_reserved(name, cur)
        }
        _ => Token::new_identify(name, cur),
    }
}
