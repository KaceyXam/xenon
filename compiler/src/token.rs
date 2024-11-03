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
    In,
    Struct,
    Enum,
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
    Dot,
    Comma,
    Range,
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

                ',' => tokens.push(Token::Comma),
                '.' => tokens.push(Token::Dot),

                '+' => tokens.push(Token::Plus),
                '-' => {
                    if src.peek() == Some(&'>') {
                        tokens.push(Token::Range);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '*' => tokens.push(Token::Multiply),
                '/' => {
                    if src.peek() == Some(&'/') {
                        while let Some(l) = src.next() {
                            if l == '\n' {
                                break;
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

                '\'' => {
                    let Some(char) = src.next() else {
                        panic!("Unterminated Char");
                    };

                    if src.peek() == Some(&'\'') {
                        src.next();
                        tokens.push(Token::CharLiteral(char));
                    } else {
                        panic!("Unterminated Char");
                    }
                }

                '"' => {
                    let mut str: Vec<char> = vec![];

                    while src.peek().is_some_and(|&c| c != '"') {
                        str.push(src.next().unwrap());
                    }

                    if src.peek().is_none() {
                        panic!("Unterminated String");
                    } else {
                        src.next();
                        tokens.push(Token::StringLiteral(str.iter().collect::<String>().into()))
                    }
                }

                ';' => tokens.push(Token::SemiColon),

                c if c.is_numeric() => {
                    let mut str = vec![c];
                    let mut float = false;

                    while let Some(l) = src.peek() {
                        if !l.is_numeric() && *l != '.' {
                            break;
                        }
                        if *l == '.' && !float {
                            float = true;
                        }

                        str.push(src.next().unwrap());
                    }

                    if float {
                        tokens.push(Token::FloatLiteral(
                            str.iter().collect::<String>().parse().unwrap(),
                        ))
                    } else {
                        tokens.push(Token::IntLiteral(
                            str.iter().collect::<String>().parse().unwrap(),
                        ))
                    }
                }

                c if c.is_alphabetic() || c == '_' => {
                    let mut str = vec![c];

                    while let Some(l) = src.peek() {
                        if !l.is_alphanumeric() && *l != '_' {
                            break;
                        }
                        str.push(src.next().unwrap());
                    }

                    tokens.push(Token::keyword(str.iter().collect()));
                }
                c if c.is_ascii_whitespace() => continue,
                c => {
                    eprintln!("Unexpected Character: {c}");
                    unimplemented!()
                }
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
            "in" => Token::In,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            x => Token::Keyword(x.into()),
        }
    }
}
