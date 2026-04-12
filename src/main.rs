use std::env;
use std::process::exit;
use crate::io_backend::structs::build_shell;

mod lexer;
mod io_backend;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut shell = build_shell(args);

    if shell.lexer.input_string.is_none() || shell.lexer.file.is_none() {
        exit(2);
    }

    let mut token = shell.lexer.peek_token();
}
