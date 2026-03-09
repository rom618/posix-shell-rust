use std::fs::File;
use crate::lexer::token::Token;

pub mod token;

pub enum LexerMode
{
    Normal,
    HeredocMode
}

pub struct Lexer
{
    pub file: Option<File>,
    pub input_string: Option<String>,
    pub curr: Option<Token>,
    pub line: Option<String>,
    pub index: usize,
    pub mode: LexerMode,
    pub s_quote: char, // either 0 or '
    pub d_quote: char, // either 0 or "
    pub dollar: char // either 0 or { or (
}
