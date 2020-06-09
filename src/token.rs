use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    GE, // >=
    GT, // >
    EQ, // ==
    LE, // <=
    LT, // <

    SemiColon,  // ;
    LeftParen,  // (
    RightParen, // )

    Assignment, // =

    If,
    Else,
    Int,

    Identifier, //标识符

    IntLiteral,    //整型字面量
    StringLiteral, //字符串字面量
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Slash => write!(f, "Slash"),

            TokenType::GE => write!(f, "GE"),
            TokenType::GT => write!(f, "GT"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::LE => write!(f, "LE"),
            TokenType::LT => write!(f, "LT"),

            TokenType::SemiColon => write!(f, "SemiColon"),
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),

            TokenType::Assignment => write!(f, "Assignment"),

            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::Int => write!(f, "Int"),

            TokenType::Identifier => write!(f, "Identifier"),

            TokenType::IntLiteral => write!(f, "IntLiteral"),
            TokenType::StringLiteral => write!(f, "StringLiteral"),
        }
    }
}

pub trait Token {
    fn get_type(&self) -> &TokenType;
    fn get_text(&self) -> &String;
}

pub struct SimpleToken {
    pub token_type: TokenType,
    pub text: String,
}

impl SimpleToken {
    pub fn new(t: TokenType, text: String) -> Self {
        SimpleToken {
            token_type: t,
            text: text,
        }
    }

    pub fn clone(&self) -> Self {
        SimpleToken {
            token_type: *self.get_type(),
            text: self.text.clone(),
        }
    }
}

impl Token for SimpleToken {
    fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    fn get_text(&self) -> &String {
        &self.text
    }
}



pub trait TokenReader {
    fn read(&mut self) -> Option<&Box<dyn Token>>;
    fn peek(&self) -> Option<&Box<dyn Token>>;
    fn unread(&mut self);
    fn get_position(&self) -> usize;
    fn set_position(&mut self, position: usize);
    fn dump(&mut self);
}

pub struct SimpleTokenReader<'a> {
    tokens: &'a Vec<Box<dyn Token>>,
    pos: usize,
}

impl<'a> SimpleTokenReader<'a> {
    pub fn new(tokens: &'a Vec<Box<dyn Token>>) -> Self {
        SimpleTokenReader {
            tokens: tokens,
            pos: 0,
        }
    }
}

impl<'a> TokenReader for SimpleTokenReader<'a> {
    fn read(&mut self) -> Option<&Box<dyn Token>> {
        if self.pos < self.tokens.len() {
            let token = self.tokens.get(self.pos);
            self.pos = self.pos + 1;
            return token;
        }
        return Option::None;
    }
    fn peek(&self) -> Option<&Box<dyn Token>> {
        if self.pos < self.tokens.len() {
            return self.tokens.get(self.pos);
        }
        return Option::None;
    }

    fn unread(&mut self) {
        if self.pos > 0 {
            self.pos = self.pos - 1;
        }
    }
    fn get_position(&self) -> usize {
        self.pos
    }
    fn set_position(&mut self, position: usize) {
        if position < self.tokens.len() {
            self.pos = position;
        }
    }
    fn dump(&mut self) {
        println!("text\t\ttype");
        loop {
            let token = self.read();
            match token {
                Option::None => break,
                Option::Some(x) => println!("{}\t\t{}", x.get_text(), x.get_type()),
            }
        }
    }
}
