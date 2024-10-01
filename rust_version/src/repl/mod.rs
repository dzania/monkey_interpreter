use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &str = ">> ";

pub fn start(input: &mut dyn std::io::BufRead, output: &mut dyn std::io::Write) {
    loop {
        write!(output, "{}", PROMPT).expect("Failed to write to output");
        output.flush().expect("Failed to flush output");
        let mut line = String::new();
        input.read_line(&mut line).expect("Failed to read line");

        if line.is_empty() {
            break;
        }

        let mut lexer = Lexer::new(line);
        loop {
            let tok = lexer.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            }

            println!("{:?}", tok);
        }
    }
}
