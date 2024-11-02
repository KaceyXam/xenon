use std::{iter::Peekable, str::Chars, sync::Arc};

#[derive(Debug)]
pub enum Token {
    Var,
    Keyword(Arc<str>),
    IntLiteral(i64),
    FloatLiteral(f32),
    StringLiteral(Arc<str>),
    CharLiteral(char),
    True,
    False,
    Func,
    For,
    If,
    Else,
    Switch,
    Case,
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterEqual,
    LessEqual,
    Bang,
    BangEqual,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Return,
    SemiColon,
}

impl Token {
    pub fn tokenize(src: String) -> Vec<Token> {
        let mut src = src.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = src.next() {
            match c {
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                '{' => tokens.push(Token::OpenBrace),
                '}' => tokens.push(Token::CloseBrace),
                '[' => tokens.push(Token::OpenBracket),
                ']' => tokens.push(Token::CloseBracket),

                ':' => tokens.push(Token::Colon),

                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => {
                    if src.peek() == Some(&'/') {
                        while let Some(l) = src.next() {
                            if l == '\n' {
                                continue;
                            }
                        }
                    } else {
                        tokens.push(Token::Divide)
                    }
                }

                '=' => {
                    if src.peek() == Some(&'=') {
                        src.next();
                        tokens.push(Token::EqualTo);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }

                '<' => {
                    if src.peek() == Some(&'=') {
                        src.next();
                        tokens.push(Token::LessEqual);
                    } else {
                        tokens.push(Token::LessThan);
                    }
                }

                '>' => {
                    if src.peek() == Some(&'=') {
                        src.next();
                        tokens.push(Token::GreaterEqual);
                    } else {
                        tokens.push(Token::GreaterThan);
                    }
                }

                '!' => {
                    if src.peek() == Some(&'=') {
                        src.next();
                        tokens.push(Token::BangEqual);
                    } else {
                        tokens.push(Token::Bang);
                    }
                }

                c if c.is_alphabetic() => {
                    let mut str = vec![c];

                    while let Some(l) = src.peek() {
                        if !l.is_alphanumeric() {
                            break;
                        }
                        str.push(src.next().unwrap());
                    }

                    tokens.push(Token::keyword(str.iter().collect()));
                }
                c if c.is_ascii_whitespace() => continue,
                _ => unimplemented!(),
            }
        }

        return tokens;
    }

    fn keyword(word: String) -> Token {
        match word.as_ref() {
            "true" => Token::True,
            "false" => Token::False,
            "func" => Token::Func,
            "var" => Token::Var,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "return" => Token::Return,
            x => Token::Keyword(x.into()),
        }
    }
}
