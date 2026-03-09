use std::env;
use crate::io_backend::structs::build_shell;

mod lexer;
mod io_backend;

fn main() {
    let args: Vec<String> = env::args().collect();

    let shell = build_shell(args);
}
