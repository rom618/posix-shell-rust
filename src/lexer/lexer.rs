use super::token;
use token::Token;
use token::TokenType;
use token::OPS;
use super::utils;
use utils::get_word_type;
use utils::is_prefix;
use utils::type_is_reserved;
use utils::is_digit_string;
use super::structs::*;
impl Lexer {

    fn get_lexer_line(&mut self) {
        // if current line still has unread chars, do nothing
        if let Some(ref line) = self.line {
            if self.index < line.len() {
                return;
            }
        }

        // reset state
        self.line = None;
        self.index = 0;

        // ask input source for next line
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

    fn lex_one_token(&mut self) -> Result<Token, LexingError> {
        self.get_lexer_line();
        if let LexerMode::HeredocMode(label) = &self.mode {
            return self.lex_heredoc_line(&*label.clone());
        }

        let ch = match self.peek_chr() {
            Some(c) => c,
            None => return Ok(Token::new(TokenType::Teof, String::new())),
        };

        // skip whitespaces
        if matches!(ch, ' ' | '\t') {
            self.skip_delim();
            return self.lex_one_token();
        }

        // comments
        if ch == '#' && self.s_quote == '\0' && self.d_quote == '\0' && self.backtick == '\0' {
            while let Some(c) = self.peek_chr() {
                if c == '\n' {
                    break;
                }
                self.pop_chr();
            }
            return self.lex_one_token();
        }

        // newline
        if ch == '\n' && self.s_quote == '\0' && self.d_quote == '\0' && self.backtick == '\0' {
            self.pop_chr();
            return Ok(Token::new(TokenType::Newline, "\n".to_string()));
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
            return Ok(Token::new(ttype, buf));
        }

        // words
        let mut token = String::new();

        while let Some(c) = self.peek_chr() {
            // stop conditions
            if self.s_quote == '\0' && self.d_quote == '\0' && self.backtick == '\0' {
                if matches!(c, ' ' | '\t' | '\n') {
                    break;
                }
                if is_prefix(OPS, &c.to_string()) {
                    break;
                }
            }

            // backticks
            if c == '`' && self.s_quote == '\0' && self.d_quote == '\0' {
                self.backtick = if self.backtick == '\0' { '`' } else { '\0' };
                self.pop_chr();
                token.push(c);
                continue;
            }

            // escapes
            if c == '\\' && self.s_quote == '\0' {
                self.pop_chr();

                let next = match self.peek_chr() {
                    Some(nc) => nc,
                    None => return Err(LexingError::UnexpectedEof),
                };

                if next == '\n' {
                    self.pop_chr(); // consume the newline
                    continue;
                }

                token.push('\\');
                token.push(next);
                self.pop_chr();
                continue;
            }

            if c == '\'' && self.d_quote == '\0' && self.backtick == '\0' {
                self.s_quote = if self.s_quote == '\0' { '\'' } else { '\0' };
                self.pop_chr();
                token.push(c);
                continue;
            }

            if c == '"' && self.s_quote == '\0' && self.backtick == '\0' {
                self.d_quote = if self.d_quote == '\0' { '"' } else { '\0' };
                self.pop_chr();
                token.push(c);
                continue;
            }

            if c == '$' && self.s_quote == '\0' && self.d_quote == '\0' && self.backtick == '\0' {
                self.dollar = '$';
            }

            // normal character
            token.push(c);
            self.pop_chr();
        }

        // unterminated constructs checks
        if self.s_quote != '\0' {
            return Err(LexingError::UnterminatedSingleQuote);
        }
        if self.d_quote != '\0' {
            return Err(LexingError::UnterminatedDoubleQuote);
        }
        if self.backtick != '\0' {
            return Err(LexingError::UnterminatedBacktick);
        }

        if token.is_empty() {
            return Err(LexingError::UnexpectedEof);
        }

        self.dollar = '\0';

        let mut ttype = get_word_type(&token);

        // io number detection
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

        Ok(Token::new(ttype, token))
    }
    pub fn peek_token(&mut self) -> Result<&Token,LexingError> {
        if self.curr.is_none() {
            self.curr = Some(self.lex_one_token()?);
        }

        Ok(self.curr.as_ref().unwrap())
    }

    pub fn pop_token(&mut self) -> Result<Token,LexingError> {
        if let Some(tok) = self.curr.take() {
            // println!("{:#?}",tok);
            self.curr = if let Some(tok) = self.next.take() {
                Some(tok)
            } else {
                None
            };
            return Ok(tok);
        }

        self.lex_one_token()
    }

    pub fn peek_next_token(&mut self) -> Result<&Token, LexingError> {
        let _ = self.peek_token();

        if self.next.is_none() {
            self.next = Some(self.lex_one_token()?);
        }
        Ok(self.next.as_ref().unwrap())
    }

    pub fn advance(&mut self) -> Result<Token, LexingError> {
        let current = self.pop_token()?;

        if let Some(n) = self.next.take() {
            self.curr = Some(n);
        }

        Ok(current)
    }

    fn lex_heredoc_line(&mut self, label: &str) -> Result<Token, LexingError> {
        let line = match self.line.take() {
            Some(l) => l,
            None => return Ok(Token::new(TokenType::Teof, String::new())),
        };

        let trimmed = line.trim_end_matches('\n');
        if trimmed == label {
            self.exit_heredoc_mode();
            return Ok(Token::new(TokenType::HeredocEof, String::new()));
        }

        self.index = line.len();
        Ok(Token::new(TokenType::Heredoc, line))
    }

    pub fn collect_heredoc_body(&mut self) -> Result<String, LexingError> {
        let mut body = String::new();

        loop {
            let token = self.lex_one_token()?;

            match token.token_type {
                TokenType::Heredoc => {
                    body.push_str(&token.value);
                }
                TokenType::HeredocEof => {
                    if matches!(self.mode, LexerMode::Normal) {
                        break;
                    } else {
                        if let LexerMode::HeredocMode(label) = &self.mode {
                            return Err(LexingError::UnterminatedHereDoc {
                                label: label.clone(),
                            });
                        }
                        break;
                    }
                }
                _ => {
                    return Err(LexingError::UnterminatedHereDoc {
                        label: ("None").parse().unwrap(),
                    });
                }
            }

            if self.mode == LexerMode::Normal {
                break;
            }
        }

        Ok(body)
    }
}