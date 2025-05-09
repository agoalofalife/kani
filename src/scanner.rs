use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;
use crate::token::*;
use crate::token::TokenType::*;

#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<Token>,
    // source: String,
    current: usize, // think about when len of chars greater them limit of i32
    start: usize,
    line: usize,
}
impl Scanner{
    pub fn new() -> Scanner {
        Scanner {
            tokens: Vec::new(),
            // source: content,
            start: 0,
            current: 0,
            line: 1,
            // char_positions,
        }
    }
    pub fn scan_tokens(&mut self, source: &String) {
        let mut iter = source.char_indices().peekable();
        let mut keywords:HashMap<String, TokenType> = HashMap::new();

        keywords.insert("and".to_string(), And);
        keywords.insert("class".to_string(), Class);
        keywords.insert("else".to_string(), Else);
        keywords.insert("false".to_string(), False);
        keywords.insert("for".to_string(), For);
        keywords.insert("function".to_string(), Function);
        keywords.insert("if".to_string(), If);
        keywords.insert("null".to_string(), Null);
        keywords.insert("or".to_string(), Or);
        keywords.insert("print".to_string(), Print);
        keywords.insert("return".to_string(), Return);
        keywords.insert("parent".to_string(), Parent);
        keywords.insert("this".to_string(), This);
        keywords.insert("true".to_string(), True);
        keywords.insert("var".to_string(), Var);
        keywords.insert("while".to_string(), While);



        while let Some((index, char)) = iter.next() {
            match char {
                '(' => self.add_token(LeftParen),
                ')' => self.add_token(RightParen),
                '{' => self.add_token(LeftBrace),
                '}' => self.add_token(RightBrace),
                ',' => self.add_token(Comma),
                '.' => self.add_token(Dot),
                '-' => self.add_token(Minus),
                '+' => self.add_token(Plus),
                ';' => self.add_token(Semicolon),
                '*' => self.add_token(Star),
                '!' => {
                    // matching with !=
                    let token_type = if self.match_with_next_char('=', &mut iter) {
                        BangEqual
                    } else {
                        Bang
                    };
                    self.add_token(token_type)
                }
                '=' => {
                    // matching with ==
                    let token_type = if self.match_with_next_char('=', &mut iter) {
                        EqualEqual
                    } else {
                        Equal
                    };
                    self.add_token(token_type)
                }
                '<' => {
                    // matching with <=
                    let token_type = if self.match_with_next_char('=', &mut iter) {
                        LessEqual
                    } else {
                        Less
                    };
                    self.add_token(token_type)
                }
                '>' => {
                    // matching with <=
                    let token_type = if self.match_with_next_char('=', &mut iter) {
                        GreaterEqual
                    } else {
                        Greater
                    };
                    self.add_token(token_type)
                }
                '/' => {
                    // handle comments and slash char
                    if self.match_with_next_char('/', &mut iter) {
                        // Ha ha this is comment
                        while let Some((_, char)) = iter.peek() {
                            if *char != '\n' {
                                iter.next();
                            } else {
                                break;
                            }
                        }
                        // while iter.peek() != '\n' && !self.is_end() {
                        //     self.next();
                        // }
                    } else {
                        self.add_token(Slash);
                    }
                }
                '"' => { // string literal
                    let byte_start = index;
                    let mut byte_end = byte_start;

                    while let Some((i, char)) = iter.peek() {
                        // println!("{} {} {}", index, i, char);
                        byte_end = *i;
                        if *char != '"' {
                            if *char == '\n' {self.line += 1}
                            iter.next();
                        } else {
                            break;
                        }
                    }

                    if iter.peek().is_none() {
                        panic!("Unterminated string")
                    }
                    // if is at end => it means unterminated string
                    iter.next(); // move from closing '"'

                    self.add_token_literal(Str, source[byte_start..=byte_end].to_string(), source[byte_start + 1..byte_end].to_string());
                },
                ' ' | '\r' | '\t' => {} // just ignore
                '\n' => {
                    self.line += 1;
                }
                _ => {
                    if Self::is_digit(char) { // for numbers
                        let byte_start = index;
                        let mut byte_end = byte_start;

                        // capture part of number until "."
                        while let Some((i, char)) = iter.peek() {
                            byte_end = *i;
                            if Self::is_digit(*char) {
                                iter.next();
                            } else {
                                break;
                            }
                        }

                        if !iter.peek().is_none() && iter.peek().unwrap().1 == '.' {
                            iter.next();
                            while let Some((i, char)) = iter.peek() {
                                byte_end = *i;
                                if Self::is_digit(*char) {
                                    iter.next();
                                } else {
                                    break;
                                }
                            }
                        }

                        self.add_token_number(Number, source[byte_start..=byte_end].to_string(), source[byte_start..byte_end].parse::<f64>().unwrap())
                    } else if char.is_alphabetic() {
                        let byte_start = index;
                        let mut byte_end = byte_start;

                        // capture part of number until "."
                        while let Some((i, char)) = iter.peek() {
                            byte_end = *i;
                            if char.is_alphanumeric() {
                                iter.next();
                            } else {
                                break;
                            }
                        }
                        let text = source[byte_start..byte_end].to_string();

                        match keywords.get(&text) {
                            Some(token_type) => self.add_token(token_type.clone()),
                            None => self.add_token(Identifier)
                        }
                    } else {
                        panic!("Unexpected character in line:{}", self.line)
                    }
                }
            };
        }
        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: TokenVal::Str("".to_string()),
            line: self.line,
        });
    }

    fn match_with_next_char(&self, candidate: char, iter: &mut Peekable<CharIndices>) -> bool {
        match iter.peek() {
            Some((_, char)) => {
                if *char == candidate {
                    iter.next();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: "".to_string(),
            literal: TokenVal::Str("".to_string()),
            line: self.line,
        })
    }

    fn add_token_literal(&mut self, token_type: TokenType, lexeme: String, literal: String) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal: TokenVal::Str(literal),
            line: self.line
        })
    }
    fn add_token_number(&mut self, token_type: TokenType, lexeme: String, literal: f64) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal: TokenVal::Float(literal),
            line: self.line
        })
    }
    fn is_digit(c:char) -> bool {
        c >= '0' && c <= '9'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_scanning_simple_function_structure() {
        let mut scanner = Scanner::new();
        scanner.scan_tokens(&"\
        function test() {\
            return 2 + 3;\
        }\
        ".to_string());
        let mut iter = scanner.tokens.iter();


        assert_eq!(iter.next().unwrap().token_type, Function);
        assert_eq!(iter.next().unwrap().token_type, Identifier);
        assert_eq!(iter.next().unwrap().token_type, LeftParen);
        assert_eq!(iter.next().unwrap().token_type, RightParen);
        assert_eq!(iter.next().unwrap().token_type, LeftBrace);
        assert_eq!(iter.next().unwrap().token_type, Return);
        assert_eq!(iter.next().unwrap().token_type, Number);
        assert_eq!(iter.next().unwrap().token_type, Plus);
        assert_eq!(iter.next().unwrap().token_type, Number);
        assert_eq!(iter.next().unwrap().token_type, Semicolon);
        assert_eq!(iter.next().unwrap().token_type, RightBrace);
    }
}