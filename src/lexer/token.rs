#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Word,
    AssignmentWord,
    Heredoc,
    HeredocEof,
    Name, // useless
    Newline,
    IoNumber,
    AndIf, // &&
    OrIf, // ||
    Dsemi, // ;;
    Less, // <
    Great, // >
    Dless, // <<
    Dgreat, // >>
    LessAnd, // <&
    GreatAnd, // >&
    LessGreat, // <`>
    DlessDash, // <<-
    Clobber, // >|
    IfToken,
    ThenToken,
    ElseToken,
    ElifToken,
    FiToken,
    DoToken,
    DoneToken,
    CaseToken,
    EsacToken,
    WhileToken,
    UntilToken,
    ForToken,
    Lbrace, // {
    Rbrace, // }
    Lpar, // (
    Rpar, // )
    Bang, // !
    Semi, // ;
    InToken,
    Amp, // &
    Pipe, // |
    Dpipe, // ||
    Teof,
    Error
}

pub const OPS: &[&str] = &[
    "&", "&&", "|", "||", ";;", "<<",
    ">>", "<&", ">&", "<>", "<<-", ">|",
    "{", "}", "(", ")", ";",
];

#[derive(Clone, Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Self {
            token_type,
            value,
        }
    }
}