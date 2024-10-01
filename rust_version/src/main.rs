pub mod lexer;
pub mod repl;
pub mod token;
use std::io;

fn main() {
    println!("Hello this is the Monkey programming language!\n");
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut handle = stdin.lock();
    let mut out_handle = stdout.lock();

    // Call the function with the standard input and output
    repl::start(&mut handle, &mut out_handle);
}
