use std::fs::File;
use std::io::BufReader;
use std::io;

use super::structs::*;
use crate::lexer;
use lexer::structs::*;

pub fn build_shell(args: Vec<String>) -> Shell {
    let mut arguments: Vec<String> = Vec::with_capacity(args.len());
    let mut iter = args.into_iter().peekable();
    let mut pretty_print = false;
    let mut input_string: Option<String> = None;
    let mut script_path: Option<String> = None;
    iter.next(); // skip first argument which is binary path

    while let Some(arg) = iter.next() {
        if input_string.is_some() || script_path.is_some() {
            arguments.push(arg);
            arguments.extend(iter);
            break;
        }

        match arg.as_str() {
            "-p" | "--pretty-print" => pretty_print = true,
            "-c" => input_string = iter.next(),
            _ => script_path = Some(arg),
        }
    }

    let source = if let Some(path) = script_path {
        let file = File::open(path).ok().map(BufReader::new);
        match file {
            Some(f) => InputSource::File(f),
            None => InputSource::Stdin(io::stdin().lock()),
        }
    } else if let Some(input) = input_string {
        InputSource::String(input)
    } else {
        InputSource::Stdin(io::stdin().lock())
    };

    let lexer = Lexer {
        source,
        curr: None,
        next: None,
        line: None,
        index: 0,
        mode: LexerMode::Normal,
        s_quote: '\0',
        d_quote: '\0',
        dollar: '\0',
        backtick: '\0',
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
