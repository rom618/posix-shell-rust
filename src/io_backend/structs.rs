use std::fs::File;
use std::io::{self, Read};

use lexer::Lexer;
use crate::lexer;
use crate::lexer::LexerMode;

pub struct Variable
{
    name: String,
    value: String,
    read_only: bool
}

pub struct Shell
{
    pretty_print: bool,
    arguments: Vec<String>,
    lexer: Lexer,
    vars: Vec<Variable>,
    encountered_exit: bool,
    status_code: u8,
    encountered_continue: bool,
    encountered_break: bool,
}

pub fn build_shell(args: Vec<String>) -> Shell {
    let mut arguments: Vec<String> = Vec::with_capacity(args.len());
    let mut iter = args.into_iter().peekable();
    let mut pretty_print = false;
    let mut input_string: Option<String> = None;
    let mut script_path: Option<String> = None;

    while let Some(arg) = iter.next() {
        if input_string.is_some() || script_path.is_some() {
            arguments.push(arg);
            arguments.extend(iter);
            break;
        }

        match arg.as_str() {
            "-p" => pretty_print = true,
            "-c" => input_string = iter.next(),
            _ => script_path = Some(arg),
        }
    }

    let file = if let Some(path) = script_path {
        match File::open(path) {
            Ok(f) => Some(f),
            Err(_) => None
        }
    } else {
        None
    };

    let lexer = Lexer {
        file,
        input_string,
        curr: None,
        line: None,
        index: 0,
        mode: LexerMode::Normal,
        s_quote: '\0',
        d_quote: '\0',
        dollar: '\0',
    };

    Shell {
        pretty_print,
        arguments,
        lexer,
        vars: Vec::new(),
        encountered_exit: false,
        status_code: 0,
        encountered_continue: false,
        encountered_break: false
    }
}