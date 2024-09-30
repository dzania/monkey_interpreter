use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Lexer {
    input: String,
    position: usize,     // Current char
    read_postion: usize, // after current
    ch: u8,              // Current char under examination
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn byte_char_to_string(ch: u8) -> String {
    (ch as char).to_string()
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            input,
            ..Self::default()
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_postion >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.position]
        }
    }

    // Sets current char
    fn read_char(&mut self) {
        if self.read_postion >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.position]
        }
        self.position = self.read_postion;
        self.read_postion += 1
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char()
        }
        self.input[position..self.position].into()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch.clone();
                    self.read_char();
                    Token::new(
                        TokenType::Eq,
                        String::from_utf8_lossy(&[ch, self.ch]).into(),
                    )
                } else {
                    Token::new(TokenType::Assign, byte_char_to_string(self.ch))
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch.clone();
                    self.read_char();
                    Token::new(
                        TokenType::NotEq,
                        String::from_utf8_lossy(&[ch, self.ch]).to_string(),
                    )
                } else {
                    Token::new(TokenType::Bang, byte_char_to_string(self.ch))
                }
            }
            _ => todo!(),
        };

        self.read_char();
        token
    }
}
