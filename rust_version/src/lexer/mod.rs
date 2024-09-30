#[derive(Default)]
pub struct Lexer {
    input: String,
    position: usize,     // Current char
    read_postion: usize, // after current
    ch: u8,              // Current char under examination
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
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

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}
