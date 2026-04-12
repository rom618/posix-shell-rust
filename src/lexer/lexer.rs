use std::io::{BufRead, BufReader};
use super::token;
use token::Token;
use token::TokenType;
use token::OPS;
use token::RED;
use super::utils;
use utils::get_word_type;
use utils::is_prefix;
use utils::type_is_reserved;
use utils::is_digit_string;

use crate::io_backend::io_backend;
use io_backend::InputSource;


pub enum LexerMode
{
    Normal,
    HeredocMode
}

pub enum LexerError
{
    UnexpectedEOF,
    InvalidToken,
    MissingError,
}

pub struct Lexer
{
    pub source: InputSource,

    pub curr: Option<Token>,
    pub line: Option<String>,
    pub index: usize,

    pub mode: LexerMode,

    pub s_quote: char, // either 0 or '
    pub d_quote: char, // either 0 or "
    pub dollar: char, // either 0 or { or (

    pub error: Option<LexerError>
}

impl Lexer {

    fn get_lexer_line(&mut self) {
        // 1. if current line still has unread chars → do nothing
        if let Some(ref line) = self.line {
            if self.index < line.len() {
                return;
            }
        }

        // 2. reset state
        self.line = None;
        self.index = 0;

        // 3. ask input source for next line
        let line = match self.source.read_line() {
            Some(l) => l,
            None => return, // EOF
        };

        self.line = Some(line);
        self.index = 0;
    }
    fn peek_chr(&self) -> Option<char> {
        let line = self.line.as_ref()?;
        line[self.index..].chars().next()
    }
    fn pop_chr(&mut self) -> Option<char> {
        let line = self.line.as_ref()?;

        let ch = line[self.index..].chars().next()?;
        self.index += ch.len_utf8();

        Some(ch)
    }
    fn skip_delim(&mut self) {
        while matches!(self.peek_chr(), Some(' ' | '\t')) {
            self.pop_chr();
        }
    }
    fn lex_heredoc_line(&mut self) -> Option<Token> {
        self.get_lexer_line();

        let line = match self.line.take() {
            Some(l) => l,
            None => return Some(Token::new(TokenType::Teof, String::new())),
        };

        if line.is_empty() {
            return Some(Token::new(TokenType::Teof, String::new()));
        }

        // consume entire line (like your C: lexer->index = size)
        self.index = line.len();

        Some(Token::new(TokenType::Heredoc, line))
    }
    pub fn get_lexer_token(&mut self) -> Option<Token> {
        // heredoc mode
        if let LexerMode::HeredocMode = self.mode {
            return self.lex_heredoc_line();
        }

        self.get_lexer_line();

        let ch = self.peek_chr()?;

        // skip whitespace
        if matches!(ch, ' ' | '\t') {
            self.skip_delim();
            return self.get_lexer_token();
        }

        // comments
        if ch == '#' && self.s_quote == '\0' && self.d_quote == '\0' {
            while let Some(c) = self.peek_chr() {
                if c == '\n' { break; }
                self.pop_chr();
            }
            return self.get_lexer_token();
        }

        // newline token
        if ch == '\n' && self.s_quote == '\0' && self.d_quote == '\0' {
            self.pop_chr();
            return Some(Token::new(TokenType::Newline, "\n".parse().unwrap()));
        }

        // operators
        let mut buf = String::new();
        buf.push(ch);

        if is_prefix(OPS, &buf) {
            self.pop_chr();

            while let Some(next) = self.peek_chr() {
                buf.push(next);

                if !is_prefix(OPS, &buf) {
                    buf.pop();
                    break;
                }

                self.pop_chr();

                if buf.len() >= 3 {
                    break;
                }
            }

            let ttype = get_word_type(&buf);

            return Some(Token::new(ttype, buf));
        }

        // words
        let mut token = String::new();

        while let Some(c) = self.peek_chr() {
            // stop conditions
            if self.s_quote == '\0'
                && self.d_quote == '\0'
                && matches!(c, ' ' | '\t' | '\n')
            {
                break;
            }

            // operator boundary
            if self.s_quote == '\0'
                && self.d_quote == '\0'
                && is_prefix(OPS, &c.to_string())
            {
                break;
            }

            // quote handling
            if c == '\'' && self.d_quote == '\0' {
                self.s_quote = if self.s_quote == '\0' { '\'' } else { '\0' };
                self.pop_chr();
                continue;
            }

            if c == '"' && self.s_quote == '\0' {
                self.d_quote = if self.d_quote == '\0' { '"' } else { '\0' };
                self.pop_chr();
                continue;
            }

            // dollar handling (simplified version of your C logic)
            if c == '$' && self.s_quote == '\0' {
                self.dollar = '$';
            }

            // append char
            token.push(c);
            self.pop_chr();
        }

        if token.is_empty() {
            return None;
        }

        self.dollar = '\0';

        let mut ttype = get_word_type(&token);

        if is_digit_string(&token) {
            if let Some(next) = self.peek_chr() {
                if next == '<' || next == '>' {
                    ttype = TokenType::IoNumber;
                }
            }
        }

        if type_is_reserved(ttype) {
            ttype = TokenType::Word;
        }

        Some(Token::new(ttype, token))
    }
    fn lex_one_token(&mut self) {
        match self.get_lexer_token() {
            Some(token) => {
                self.curr = Some(token);
            }
            None => {
                self.error = Some(LexerError::UnexpectedEOF);
                self.curr = None;
            }
        }
    }
    pub fn peek_token(&mut self) -> &Token {
        if self.curr.is_none() {
            self.lex_one_token();
        }

        self.curr.as_ref().unwrap()
    }

    pub fn pop_token(&mut self) -> Token {
        self.peek_token();
        self.curr.take().unwrap()
    }
}