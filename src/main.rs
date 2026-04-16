use std::env;
use std::process::exit;
use crate::io_backend::io_backend::build_shell;
use log::{error};
use crate::ast::pretty_print::pretty_print::PrettyPrinter;
use crate::parser::structs::Parser;

mod lexer;
mod io_backend;
pub mod ast;
pub mod parser;
pub mod exec;
pub mod variables;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut shell = build_shell(args);

    let mut parser = Parser::new(&mut shell.lexer);

    assert!(shell.pretty_print);

    loop {
        match parser.parse_complete_command() {
            Ok(ast) => {
                if shell.pretty_print {
                    // println!("{:#?}",ast);
                    let mut pp = PrettyPrinter::new(String::new());
                    pp.print_complete(&ast);
                    let output = pp.finish();
                    println!("{}", output.trim_end_matches('\n'));
                } else {
                    // execute(ast);
                    if shell.encountered_exit {
                        exit(shell.status_code);
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
                exit(2);
            }
        }

        match parser.at_eof() {
            Ok(eof) => {
                if eof {
                    break;
                }
            }
            Err(e) => {
                error!("{}",e);
                exit(2)
            }
        }
    }
    exit(0)
}
