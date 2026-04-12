use std::fs::File;
use std::io::BufReader;
use std::io;
use crate::lexer;
use lexer::lexer::Lexer;
use lexer::lexer::LexerMode;
use crate::io_backend::io_backend;
use io_backend::InputSource;

pub struct Variable
{
    pub name: String,
    pub value: String,
    pub read_only: bool
}

pub struct Shell
{
    pub pretty_print: bool,
    pub arguments: Vec<String>,
    pub lexer: Lexer,
    pub vars: Vec<Variable>,
    pub encountered_exit: bool,
    pub status_code: u8,
    pub encountered_continue: bool,
    pub encountered_break: bool,
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
        line: None,
        index: 0,
        mode: LexerMode::Normal,
        s_quote: '\0',
        d_quote: '\0',
        dollar: '\0',
        error: None,
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