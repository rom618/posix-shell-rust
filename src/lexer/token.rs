pub enum TokenType {
    Word,
    AssignmentWord,
    Heredoc,
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
    LessGreat, // <>
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
pub struct Token {
    pub value: String,
    pub token_type: TokenType
}

impl Token {

}