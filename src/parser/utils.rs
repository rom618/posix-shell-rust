use crate::lexer::token::{Token, TokenType};
use crate::lexer::utils::{get_word_type, is_reserved_word};
use crate::parser::structs::{Parser, ParsingError};

impl<'a> Parser<'a> {
    pub fn expect(&mut self, expected: TokenType) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        if tok.token_type == expected {
            Ok(tok)
        } else {
            Err(ParsingError::UnexpectedToken {
                expected: Some(expected),
                got: tok.token_type,
            })
        }
    }

    pub fn matches(&mut self, t: TokenType) -> bool {
        self.lexer.peek_token().map(|tok| tok.token_type == t).unwrap_or(false)
    }

    pub fn matches_next(&mut self, t: TokenType) -> bool {
        self.lexer.peek_next_token().map(|tok| tok.token_type == t).unwrap_or(false)
    }

    pub fn parse_linebreak(&mut self) {
        while self.matches(TokenType::Newline) {
            let _ = self.lexer.pop_token();
        }
    }

    pub fn parse_newline_list(&mut self) -> bool {
        let mut saw_one = false;
        while self.matches(TokenType::Newline) {
            let _ = self.lexer.pop_token();
            saw_one = true;
        }
        saw_one
    }

    pub fn parse_sequential_sep(&mut self) {
        self.parse_linebreak();
        if self.matches(TokenType::Semi) {
            let _ = self.lexer.pop_token();
            self.parse_linebreak();
        } else {
            self.parse_newline_list();
        }
    }

    pub fn matches_reserved(&mut self, ttype: TokenType) -> bool {
        match self.lexer.peek_token() {
            Ok(tok) => get_word_type(&tok.value) == ttype,
            Err(_) => false,
        }
    }
    pub fn matches_reserved_any(&mut self) -> bool {
        match self.lexer.peek_token() {
            Ok(tok) => is_reserved_word(&tok.value),
            Err(_) => false,
        }
    }

    pub fn expect_reserved(&mut self, ttype: TokenType) -> Result<Token, ParsingError> {
        if self.matches_reserved(ttype) {
            Ok(self.lexer.pop_token()?)
        } else {
            let got = self.lexer.peek_token()
                .map(|t| get_word_type(&t.value))
                .unwrap_or(TokenType::Error);
            Err(ParsingError::UnexpectedToken { expected: Some(ttype), got })
        }
    }

    pub fn at_compound_list_end(&mut self) -> bool {
        match self.lexer.peek_token() {
            Ok(tok) => matches!(
            get_word_type(&tok.value),
            TokenType::ThenToken
            | TokenType::DoToken
            | TokenType::DoneToken
            | TokenType::FiToken
            | TokenType::ElifToken
            | TokenType::ElseToken
            | TokenType::EsacToken
        ),
            Err(_) => true,
        }
    }
}