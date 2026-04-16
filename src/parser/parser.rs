use log::error;
use crate::ast::ast::*;
use crate::lexer::structs::LexingError;
use crate::lexer::utils::is_reserved_word;
use crate::lexer::token::*;
use super::structs::*;

impl<'a> Parser<'a> {
    pub fn parse_complete_command(&mut self) -> Result<CompleteCommand, ParsingError> {
        let list = self.parse_list()?;
        let _ = self.parse_separator();
        Ok(CompleteCommand { list })
    }

    pub fn parse_list(&mut self) -> Result<List, ParsingError> {
        let first = self.parse_and_or()?;
        let mut rest: Vec<ListOp> = Vec::new();
        loop {
            let sep = match self.parse_separator() {
                Ok(Some(op)) => op,
                _ => break,
            };
            let and_or = self.parse_and_or()?;
            rest.push(ListOp { op: sep, node: and_or });
        }
        Ok(List { first, rest })
    }

    pub fn parse_and_or(&mut self) -> Result<AndOr, ParsingError> {
        let first = self.parse_pipeline()?;
        let mut rest: Vec<AndOrOpNode> = Vec::new();
        loop {
            let op = if self.matches(TokenType::AndIf) {
                let _ = self.lexer.pop_token();
                AndOrOp::AndIf
            } else if self.matches(TokenType::OrIf) {
                let _ = self.lexer.pop_token();
                AndOrOp::OrIf
            } else {
                break;
            };
            self.parse_linebreak();
            let rhs = self.parse_pipeline()?;
            rest.push(AndOrOpNode { op, rhs });
        }
        Ok(AndOr { first, rest })
    }

    pub fn parse_pipeline(&mut self) -> Result<Pipeline, ParsingError> {
        let negated = if self.matches_reserved(TokenType::Bang) {
            let _ = self.lexer.pop_token();
            true
        } else {
            false
        };
        let pipe_sequence = self.parse_pipe_sequence()?;
        let mut pipeline = pipe_sequence;
        pipeline.negated = negated;
        Ok(pipeline)
    }

    pub fn parse_pipe_sequence(&mut self) -> Result<Pipeline, ParsingError> {
        let mut commands = vec![self.parse_command()?];
        loop {
            if !self.matches(TokenType::Pipe) {
                break;
            }
            let _ = self.lexer.pop_token();
            self.parse_linebreak();
            commands.push(self.parse_command()?);
        }
        if commands.is_empty() {
            return Err(ParsingError::EmptyPipeline);
        }
        Ok(Pipeline {
            negated: false,
            commands,
        })
    }

    pub fn parse_command(&mut self) -> Result<Command, ParsingError> {
        if (self.matches(TokenType::Name) || self.matches(TokenType::Word))
            && !self.matches_reserved_any()
            && self.matches_next(TokenType::Lpar)
        {
            let func = self.parse_function_definition()?;
            return Ok(Command::Function(func));
        }
        if let Ok(comp) = self.parse_compound_command() {
            let redirects = self.parse_redirect_list()?;
            return Ok(Command::Compound {
                command: comp,
                redirects,
            });
        }
        let simple = self.parse_simple_command()?;
        Ok(Command::Simple(simple))
    }

    pub fn parse_compound_command(&mut self) -> Result<CompoundCommand, ParsingError> {
        if self.matches(TokenType::Lbrace) {
            self.parse_brace_group()
        } else if self.matches(TokenType::Lpar) {
            self.parse_subshell()
        } else if self.matches_reserved(TokenType::ForToken) {
            let for_c = self.parse_for_clause()?;
            Ok(CompoundCommand::For(for_c))
        } else if self.matches_reserved(TokenType::CaseToken) {
            let case_c = self.parse_case_clause()?;
            Ok(CompoundCommand::Case(case_c))
        } else if self.matches_reserved(TokenType::IfToken) {
            let if_c = self.parse_if_clause()?;
            Ok(CompoundCommand::If(if_c))
        } else if self.matches_reserved(TokenType::WhileToken) {
            let w = self.parse_while_clause()?;
            Ok(CompoundCommand::While(w))
        } else if self.matches_reserved(TokenType::UntilToken) {
            let u = self.parse_until_clause()?;
            Ok(CompoundCommand::Until(u))
        } else {
            Err(ParsingError::UnexpectedToken { expected: None, got: TokenType::Error })
        }
    }

    pub fn parse_subshell(&mut self) -> Result<CompoundCommand, ParsingError> {
        self.expect(TokenType::Lpar)?;
        let list = self.parse_compound_list()?;
        self.expect(TokenType::Rpar)?;
        Ok(CompoundCommand::Subshell(list))
    }

    pub fn parse_compound_list(&mut self) -> Result<Vec<AndOr>, ParsingError> {
        self.parse_linebreak();
        if self.at_compound_list_end() {
            return Err(ParsingError::EmptyCompoundList);
        }
        let terms = self.parse_term()?;
        if terms.is_empty() {
            return Err(ParsingError::EmptyCompoundList);
        }
        let _ = self.parse_separator();
        Ok(terms)
    }

    pub fn parse_term(&mut self) -> Result<Vec<AndOr>, ParsingError> {
        let mut and_ors = vec![self.parse_and_or()?];
        loop {
            if self.at_compound_list_end() {
                break;
            }
            if let Ok(_op) = self.parse_separator_op() {
                self.parse_linebreak();
                if self.at_compound_list_end() {
                    break;
                }
                let next = self.parse_and_or()?;
                and_ors.push(next);
                continue;
            }
            if self.parse_newline_list() {
                if self.at_compound_list_end() {
                    break;
                }
                let next = self.parse_and_or()?;
                and_ors.push(next);
                continue;
            }
            break;
        }
        Ok(and_ors)
    }

    pub fn parse_for_clause(&mut self) -> Result<ForClause, ParsingError> {
        self.expect_reserved(TokenType::ForToken)?;
        let name = self.parse_name()?.value; // rule 5
        self.parse_linebreak();
        let words = if self.matches_reserved(TokenType::InToken) {
            self.expect_reserved(TokenType::InToken)?; // rule 6
            let wordlist = self.parse_wordlist()?;
            self.parse_sequential_sep();
            Some(wordlist.into_iter().map(|t| t.value).collect())
        } else {
            None
        };
        let body = self.parse_do_group()?;
        Ok(ForClause { var: name, words, body })
    }

    pub fn parse_name(&mut self) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        if tok.token_type == TokenType::Name || tok.token_type == TokenType::Word {
            Ok(tok)
        } else {
            Err(ParsingError::ExpectedWord)
        }
    }

    pub fn parse_wordlist(&mut self) -> Result<Vec<Token>, ParsingError> {
        let mut words = vec![];
        while self.matches(TokenType::Word) {
            let w = self.lexer.pop_token()?;
            words.push(w);
        }
        Ok(words)
    }

    pub fn parse_case_clause(&mut self) -> Result<CaseClause, ParsingError> {
        self.expect_reserved(TokenType::CaseToken)?;
        let word = self.lexer.pop_token()?.value; // WORD
        self.parse_linebreak();
        self.expect_reserved(TokenType::InToken)?;
        self.parse_linebreak();
        let items = if self.matches_reserved(TokenType::EsacToken) {
            vec![]
        } else {
            self.parse_case_list()?
        };
        self.expect_reserved(TokenType::EsacToken)?;
        Ok(CaseClause { word, items })
    }

    pub fn parse_case_list(&mut self) -> Result<Vec<CaseItem>, ParsingError> {
        let mut items = vec![];
        while !self.matches_reserved(TokenType::EsacToken) {
            items.push(self.parse_case_item()?);
        }
        Ok(items)
    }

    pub fn parse_case_item(&mut self) -> Result<CaseItem, ParsingError> {
        if self.matches(TokenType::Lpar) {
            let _ = self.lexer.pop_token();
        }
        let patterns = self.parse_pattern()?;
        self.expect(TokenType::Rpar)?;
        self.parse_linebreak();
        let body = if self.matches(TokenType::Dsemi) || self.matches_reserved(TokenType::EsacToken) {
            vec![]
        } else {
            self.parse_compound_list()?
        };
        if self.matches(TokenType::Dsemi) {
            let _ = self.lexer.pop_token();
            self.parse_linebreak();
        }
        self.parse_linebreak();
        Ok(CaseItem {
            patterns: patterns.into_iter().map(|t| t.value).collect(),
            body,
        })
    }

    pub fn parse_pattern(&mut self) -> Result<Vec<Token>, ParsingError> {
        let mut pats = vec![self.lexer.pop_token()?]; // first WORD
        while self.matches(TokenType::Pipe) {
            let _ = self.lexer.pop_token();
            pats.push(self.lexer.pop_token()?); // more WORD
        }
        Ok(pats)
    }

    pub fn parse_if_clause(&mut self) -> Result<IfClause, ParsingError> {
        self.expect_reserved(TokenType::IfToken)?;
        let condition = self.parse_compound_list()?;
        self.expect_reserved(TokenType::ThenToken)?;
        let then_branch = self.parse_compound_list()?;
        let mut elif_parts = vec![];
        let mut else_branch = None;
        while self.matches_reserved(TokenType::ElifToken) {
            let _ = self.lexer.pop_token();
            let elif_cond = self.parse_compound_list()?;
            self.expect_reserved(TokenType::ThenToken)?;
            let elif_then = self.parse_compound_list()?;
            elif_parts.push(ElifPart {
                condition: elif_cond,
                then_branch: elif_then,
            });
        }
        if self.matches_reserved(TokenType::ElseToken) {
            let _ = self.lexer.pop_token();
            else_branch = Some(self.parse_compound_list()?);
        }
        self.expect_reserved(TokenType::FiToken)?;
        Ok(IfClause{
            condition,
            then_branch,
            elif_parts,
            else_branch,
        })
    }

    pub fn parse_while_clause(&mut self) -> Result<WhileClause, ParsingError> {
        self.expect_reserved(TokenType::WhileToken)?;
        let condition = self.parse_compound_list()?;
        let body = self.parse_do_group()?;
        Ok(WhileClause { condition, body })
    }

    pub fn parse_until_clause(&mut self) -> Result<UntilClause, ParsingError> {
        self.expect_reserved(TokenType::UntilToken)?;
        let condition = self.parse_compound_list()?;
        let body = self.parse_do_group()?;
        Ok(UntilClause { condition, body })
    }

    pub fn parse_function_definition(&mut self) -> Result<FunctionDef, ParsingError> {
        let name_tok = self.parse_fname()?;
        self.expect(TokenType::Lpar)?;
        self.expect(TokenType::Rpar)?;
        self.parse_linebreak();
        let mut func = FunctionDef {
            name: name_tok.value,
            body: CompoundCommand::BraceGroup(vec![]), // temp
            redirects: vec![],
        };
        self.parse_function_body(&mut func)?;
        Ok(func)
    }

    pub fn parse_function_body(&mut self, func: &mut FunctionDef) -> Result<(), ParsingError> {
        func.body = self.parse_compound_command()?;
        func.redirects = self.parse_redirect_list()?;
        Ok(())
    }

    pub fn parse_fname(&mut self) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        // must not be a reserved word
        if tok.token_type == TokenType::Name
            || (tok.token_type == TokenType::Word && !is_reserved_word(&tok.value))
        {
            Ok(tok)
        } else {
            Err(ParsingError::InvalidFunctionName(tok.value))
        }
    }
    pub fn parse_brace_group(&mut self) -> Result<CompoundCommand, ParsingError> {
        self.expect(TokenType::Lbrace)?;
        let list = self.parse_compound_list()?;
        self.expect(TokenType::Rbrace)?;
        Ok(CompoundCommand::BraceGroup(list))
    }

    pub fn parse_do_group(&mut self) -> Result<Vec<AndOr>, ParsingError> {
        self.expect_reserved(TokenType::DoToken)?;
        let body = self.parse_compound_list()?;
        self.expect_reserved(TokenType::DoneToken)?;
        Ok(body)
    }

    pub fn parse_simple_command(&mut self) -> Result<SimpleCommand, ParsingError> {
        let prefix_items = self.parse_cmd_prefix()?;

        let mut assignments = vec![];
        let mut redirects = vec![];

        for item in prefix_items {
            match item {
                CmdPrefixItem::Assignment(a) => assignments.push(a),
                CmdPrefixItem::Redirect(r) => redirects.push(r),
            }
        }

        let name = if let Ok(cmd_name_tok) = self.parse_cmd_name() {
            Some(cmd_name_tok.value)
        } else {
            None
        };

        let suffix_items = self.parse_cmd_suffix()?;

        let mut words = vec![];
        for item in suffix_items {
            match item {
                CmdSuffixItem::Word(w) => words.push(w),
                CmdSuffixItem::Redirect(r) => redirects.push(r),
            }
        }

        Ok(SimpleCommand {
            assignments,
            name,
            words,
            redirects,
        })
    }

    pub fn parse_cmd_name(&mut self) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        if tok.token_type == TokenType::Word {
            Ok(tok) // rule 7a
        } else {
            Err(ParsingError::ExpectedWord)
        }
    }

    pub fn parse_cmd_word(&mut self) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        if tok.token_type == TokenType::Word {
            Ok(tok) // rule 7b
        } else {
            Err(ParsingError::ExpectedWord)
        }
    }

    pub fn parse_cmd_prefix(&mut self) -> Result<Vec<CmdPrefixItem>, ParsingError> {
        let mut items = vec![];

        loop {
            if self.matches(TokenType::AssignmentWord) {
                let tok = self.lexer.pop_token()?;
                let parts: Vec<&str> = tok.value.splitn(2, '=').collect();
                if parts.len() == 2 {
                    items.push(CmdPrefixItem::Assignment(Assignment {
                        key: parts[0].to_string(),
                        value: Some(parts[1].to_string()),
                    }));
                    continue;
                } else {
                    return Err(ParsingError::InvalidAssignment(tok.value));
                }
            } else if self.matches(TokenType::IoNumber)
                || self.matches(TokenType::Less)
                || self.matches(TokenType::Great)
                || self.matches(TokenType::Dgreat)
                || self.matches(TokenType::LessAnd)
                || self.matches(TokenType::GreatAnd)
                || self.matches(TokenType::LessGreat)
                || self.matches(TokenType::Clobber)
                || self.matches(TokenType::Dless)
                || self.matches(TokenType::DlessDash)
            {
                let redir = self.parse_io_redirect()?;
                items.push(CmdPrefixItem::Redirect(redir));
                continue;
            }

            break;
        }

        Ok(items)
    }

    pub fn parse_cmd_suffix(&mut self) -> Result<Vec<CmdSuffixItem>, ParsingError> {
        let mut items = vec![];
        loop {
            if self.matches(TokenType::Word) {
                let w = self.lexer.pop_token()?.value;
                items.push(CmdSuffixItem::Word(w));
            } else if self.matches(TokenType::IoNumber)
                || self.matches(TokenType::Less)
                || self.matches(TokenType::Great)
                || self.matches(TokenType::Dgreat)
                || self.matches(TokenType::LessAnd)
                || self.matches(TokenType::GreatAnd)
                || self.matches(TokenType::LessGreat)
                || self.matches(TokenType::Clobber)
                || self.matches(TokenType::Dless)
                || self.matches(TokenType::DlessDash)
            {
                let redir = self.parse_io_redirect()?;
                items.push(CmdSuffixItem::Redirect(redir));
            } else {
                break;
            }
        }
        Ok(items)
    }

    pub fn parse_redirect_list(&mut self) -> Result<Vec<Redirect>, ParsingError> {
        let mut reds = vec![];
        while let Ok(r) = self.parse_io_redirect() {
            reds.push(r);
        }
        Ok(reds)
    }

    pub fn parse_io_redirect(&mut self) -> Result<Redirect, ParsingError> {
        let mut redir = Redirect {
            fd: None,
            kind: RedirectKind::Input, // default
            target: None,
            body: None,
        };
        if self.matches(TokenType::IoNumber) {
            let num_tok = self.lexer.pop_token()?;
            redir.fd = Some(num_tok.value.parse().map_err(|_| ParsingError::InvalidFileDescriptor(num_tok.value))?);
        }
        if self.matches(TokenType::Dless) || self.matches(TokenType::DlessDash) {
            self.parse_io_here(&mut redir)?;
        } else {
            self.parse_io_file(&mut redir)?;
        }
        Ok(redir)
    }

    pub fn parse_io_file(&mut self, redir: &mut Redirect) -> Result<(), ParsingError> {
        let op = self.lexer.pop_token()?;
        match op.token_type {
            TokenType::Less => redir.kind = RedirectKind::Input,
            TokenType::Great => redir.kind = RedirectKind::Output,
            TokenType::Dgreat => redir.kind = RedirectKind::Append,
            TokenType::LessAnd => redir.kind = RedirectKind::DupInput,
            TokenType::GreatAnd => redir.kind = RedirectKind::DupOutput,
            TokenType::LessGreat => redir.kind = RedirectKind::Open,
            TokenType::Clobber => redir.kind = RedirectKind::Clobber,
            _ => return Err(ParsingError::UnexpectedToken { expected: None, got: op.token_type }),
        }
        let filename = self.parse_filename()?;
        redir.target = Some(filename.value);
        Ok(())
    }

    pub fn parse_io_here(&mut self, redir: &mut Redirect) -> Result<(), ParsingError> {
        let op = self.lexer.pop_token()?;
        redir.kind = if op.token_type == TokenType::Dless {
            RedirectKind::HereDoc
        } else {
            RedirectKind::HereDocDash
        };
        let end = self.parse_here_end()?;
        let label = end.value.clone();

        self.lexer.enter_heredoc_mode(&end.value);

        let body = self.lexer.collect_heredoc_body()
            .map_err(ParsingError::LexingError)?;

        redir.target = Some(label);   // delimiter word
        redir.body = Some(body);      // heredoc content

        Ok(())
    }

    pub fn parse_filename(&mut self) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        if tok.token_type == TokenType::Word {
            Ok(tok) // rule 2
        } else {
            Err(ParsingError::ExpectedRedirectTarget)
        }
    }

    pub fn parse_here_end(&mut self) -> Result<Token, ParsingError> {
        let tok = self.lexer.pop_token()?;
        if tok.token_type == TokenType::Word {
            Ok(tok) // rule 3
        } else {
            Err(ParsingError::ExpectedWord)
        }
    }

    pub fn parse_separator_op(&mut self) -> Result<SeparatorOp, ParsingError> {
        let tok = self.lexer.peek_token()?;
        match tok.token_type {
            TokenType::Semi => {
                let _ = self.lexer.pop_token();
                Ok(SeparatorOp::Seq)
            }
            TokenType::Amp => {
                let _ = self.lexer.pop_token();
                Ok(SeparatorOp::Background)
            }
            _ => Err(ParsingError::UnexpectedToken { expected: None, got: tok.token_type }),
        }
    }

    pub fn parse_separator(&mut self) -> Result<Option<SeparatorOp>, ParsingError> {
        if let Ok(op) = self.parse_separator_op() {
            self.parse_linebreak();
            Ok(Some(op))
        } else if self.matches(TokenType::Newline) {
            self.parse_newline_list();
            Ok(None) // newline acts as separator
        } else {
            Ok(None)
        }
    }

    pub fn at_eof(&mut self) -> Result<bool, LexingError> {
        match self.lexer.peek_token() {
            Ok(tok) => {
                if tok.token_type == TokenType::Teof {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                error!("at_eof: {}", e);
                Err(e)
            }
        }
    }
}

