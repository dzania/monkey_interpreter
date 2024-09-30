#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    Int,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

// Define keywords as a hash map
use std::collections::HashMap;

fn keywords() -> HashMap<String, TokenType> {
    let mut map = HashMap::new();
    map.insert("fn".to_string(), TokenType::Function);
    map.insert("let".to_string(), TokenType::Let);
    map.insert("true".to_string(), TokenType::True);
    map.insert("false".to_string(), TokenType::False);
    map.insert("if".to_string(), TokenType::If);
    map.insert("else".to_string(), TokenType::Else);
    map.insert("return".to_string(), TokenType::Return);
    map
}

// Function to look up an identifier and return the correct TokenType
pub fn lookup_ident(ident: &str) -> TokenType {
    let keywords_map = keywords();
    if let Some(tok) = keywords_map.get(ident) {
        tok.clone()
    } else {
        TokenType::Ident
    }
}

// Define constants
impl TokenType {
    pub const ASSIGN: &'static str = "=";
    pub const PLUS: &'static str = "+";
    pub const MINUS: &'static str = "-";
    pub const BANG: &'static str = "!";
    pub const ASTERISK: &'static str = "*";
    pub const SLASH: &'static str = "/";

    pub const LT: &'static str = "<";
    pub const GT: &'static str = ">";

    pub const COMMA: &'static str = ",";
    pub const SEMICOLON: &'static str = ";";

    pub const LPAREN: &'static str = "(";
    pub const RPAREN: &'static str = ")";
    pub const LBRACE: &'static str = "{";
    pub const RBRACE: &'static str = "}";

    pub const EQ: &'static str = "==";
    pub const NOT_EQ: &'static str = "!=";
}
