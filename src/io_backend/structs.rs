use std::fs::File;
use std::io::{BufRead,BufReader};
use std::io::StdinLock;

use crate::lexer;
use lexer::structs::*;
use crate::variables::structs::*;

pub struct Shell
{
    pub pretty_print: bool,
    pub arguments: Vec<String>,
    pub lexer: Lexer,
    pub vars: Vec<Variable>,
    pub encountered_exit: bool,
    pub status_code: i32,
    pub encountered_continue: bool,
    pub encountered_break: bool,
}

pub enum InputSource {
    File(BufReader<File>),
    Stdin(StdinLock<'static>),
    String(String),
}

impl InputSource {
    pub fn read_line(&mut self) -> Option<String> {
        let mut buf = String::new();

        match self {
            InputSource::File(reader) => {
                match reader.read_line(&mut buf) {
                    Ok(0) | Err(_) => None,
                    Ok(_) => Some(buf),
                }
            }

            InputSource::String(s) => {
                if s.is_empty() {
                    None
                } else if let Some(pos) = s.find('\n') {
                    let line = s[..=pos].to_string();
                    *s = s[pos + 1..].to_string();
                    Some(line)
                } else {
                    let line = s.clone();
                    s.clear();
                    Some(line)
                }
            }

            InputSource::Stdin(stdin) => {
                match stdin.read_line(&mut buf) {
                    Ok(0) | Err(_) => None,
                    Ok(_) => Some(buf),
                }
            }
        }
    }
}