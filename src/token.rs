use std::fmt;
use std::rc::Rc;

use crate::callables::{
    lax_object::LaxObject,
    Callable,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
    //One char tokens
    OpenParen, CloseParen,
    OpenBrace, CloseBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    //Comparison Tokens
    Equal, EqualEqual,
    Bang, BangEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
       
    //Literals
    Identifier, String, Number,

    //Keywords
    If, Else, And, Or, True, False,
    For, While, Let, Fn, Class, Return,
    Nil, Print, Super, This,

    Eof,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Rc<Value>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String,
        literal: Rc<Value>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq)]
pub enum Value {
    String(String),
    Num(f64),
    Bool(bool),
    Callable(Callable),
    LaxObject(LaxObject),
    None
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(value) => write!(f, "{}", value),
            Value::Num(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Callable(value) => write!(f, "{}", value),
            Value::LaxObject(value) => write!(f, "{}", value),
            Value::None => write!(f, "nil"),
        }
    }
}

