use std::fmt;

use crate::io_backend::structs::InputSource;
use super::token;
use token::Token;
#[derive(Clone, PartialEq)]
pub enum LexerMode
{
    Normal,
    HeredocMode(String),
}

#[derive(Debug, Clone)]
pub enum LexingError {
    // quotes & escapes
    UnterminatedSingleQuote,        // no closing '
    UnterminatedDoubleQuote,        // no closing "
    UnterminatedBacktick,           // no closing `
    InvalidEscapeSequence(char),    // \x where x is not meaningful

    // substitutions
    UnterminatedCommandSubstitution, // $( with no closing )
    UnterminatedArithmetic,          // $(( with no closing ))
    UnterminatedParameterExpansion,  // ${ with no closing }

    // heredoc
    UnterminatedHereDoc { label: String }, // << LABEL but body never terminated

    // characters & tokens
    InvalidCharacter(char),              // character not valid in this context
    UnexpectedEof,                   // input ended mid-token
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexingError::UnterminatedSingleQuote         => write!(f, "unterminated single quote"),
            LexingError::UnterminatedDoubleQuote         => write!(f, "unterminated double quote"),
            LexingError::UnterminatedBacktick            => write!(f, "unterminated backtick substitution"),
            LexingError::InvalidEscapeSequence(c)        => write!(f, "invalid escape sequence '\\{c}'"),
            LexingError::UnterminatedCommandSubstitution => write!(f, "unterminated command substitution '$('"),
            LexingError::UnterminatedArithmetic          => write!(f, "unterminated arithmetic expansion '$(('"),
            LexingError::UnterminatedParameterExpansion  => write!(f, "unterminated parameter expansion '${{'"),
            LexingError::UnterminatedHereDoc { label }   => write!(f, "unterminated heredoc: missing '{label}'"),
            LexingError::InvalidCharacter(c)                 => write!(f, "unexpected character '{c}'"),
            LexingError::UnexpectedEof                   => write!(f, "unexpected end of input"),
        }
    }
}

pub struct Lexer
{
    pub source: InputSource,

    pub curr: Option<Token>,
    pub next: Option<Token>,

    pub line: Option<String>,
    pub index: usize,

    pub mode: LexerMode,

    pub s_quote: char, // either 0 or '
    pub d_quote: char, // either 0 or "
    pub dollar: char, // either 0 or { or (
    pub backtick: char, // either 0 or `
}
impl Lexer {
    pub fn enter_heredoc_mode(&mut self, label: &str) {
        self.mode = LexerMode::HeredocMode(label.to_string());
    }

    pub fn exit_heredoc_mode(&mut self) {
        self.mode = LexerMode::Normal;
    }
}