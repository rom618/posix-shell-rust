use std::fmt;
use crate::lexer::structs::*;
use crate::lexer::token::*;
#[derive(Debug, Clone)]
pub enum ParsingError {
    // token-level
    UnexpectedToken { expected: Option<TokenType>, got: TokenType },
    UnexpectedEof,

    // missing reserved keywords
    ExpectedWord,
    ExpectedThen,
    ExpectedDo,
    ExpectedFi,
    ExpectedDone,
    ExpectedEsac,
    ExpectedIn,
    ExpectedRparen,
    ExpectedRbrace,
    ExpectedFunctionBody,

    // structural
    EmptyCompoundList,
    EmptyPipeline,
    EmptyCase,
    InvalidFunctionName(String),
    InvalidAssignment(String),

    // redirect-specific
    ExpectedRedirectTarget,
    InvalidFileDescriptor(String),
    HereDocMissing { label: String },

    // limits
    NestingTooDeep,

    // wrap lexing errors
    LexingError(LexingError),
}
impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnexpectedToken { expected, got } => match expected {
                Some(exp) => write!(f, "unexpected token: expected '{exp:?}', got '{got:?}'"),
                None      => write!(f, "unexpected token: got '{got:?}'"),
            },
            ParsingError::UnexpectedEof                  => write!(f, "unexpected end of input"),
            ParsingError::ExpectedWord                   => write!(f, "expected a word"),
            ParsingError::ExpectedThen                   => write!(f, "expected 'then' after condition"),
            ParsingError::ExpectedDo                     => write!(f, "expected 'do' after condition"),
            ParsingError::ExpectedFi                     => write!(f, "expected 'fi' to close 'if'"),
            ParsingError::ExpectedDone                   => write!(f, "expected 'done' to close loop"),
            ParsingError::ExpectedEsac                   => write!(f, "expected 'esac' to close 'case'"),
            ParsingError::ExpectedIn                     => write!(f, "expected 'in' in for-clause"),
            ParsingError::ExpectedRparen                 => write!(f, "expected ')' to close subshell"),
            ParsingError::ExpectedRbrace                 => write!(f, "expected '}}' to close brace group"),
            ParsingError::ExpectedFunctionBody           => write!(f, "expected function body after '()'"),
            ParsingError::EmptyCompoundList              => write!(f, "empty compound list"),
            ParsingError::EmptyPipeline                  => write!(f, "empty pipeline"),
            ParsingError::EmptyCase                      => write!(f, "empty case clause"),
            ParsingError::InvalidFunctionName(name)      => write!(f, "invalid function name: '{name}'"),
            ParsingError::InvalidAssignment(s)           => write!(f, "invalid assignment: '{s}'"),
            ParsingError::ExpectedRedirectTarget         => write!(f, "expected target after redirect operator"),
            ParsingError::InvalidFileDescriptor(s)       => write!(f, "invalid file descriptor: '{s}'"),
            ParsingError::HereDocMissing { label }       => write!(f, "heredoc body missing for label '{label}'"),
            ParsingError::NestingTooDeep                 => write!(f, "nesting too deep"),
            ParsingError::LexingError(e) => write!(f,"lexing error: '{}'",e),
        }
    }
}
pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub errors: Vec<ParsingError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            errors: Vec::new(),
        }
    }
}

impl From<LexingError> for ParsingError {
    fn from(err: LexingError) -> Self {
        ParsingError::LexingError(err)
    }
}