#[derive(Debug)]
pub struct CompleteCommand {
    pub list: List,
}

#[derive(Debug)]
pub struct List {
    pub first: AndOr,
    pub rest: Vec<ListOp>,
}

#[derive(Debug)]
pub struct ListOp {
    pub op: SeparatorOp,
    pub node: AndOr,
}

#[derive(Debug)]
pub enum SeparatorOp {
    Seq,        // ;
    Background, // &
}

#[derive(Debug)]
pub struct AndOr {
    pub first: Pipeline,
    pub rest: Vec<AndOrOpNode>,
}

#[derive(Debug)]
pub struct AndOrOpNode {
    pub op: AndOrOp,
    pub rhs: Pipeline,
}

#[derive(Debug)]
pub enum AndOrOp {
    AndIf, // &&
    OrIf,  // ||
}

#[derive(Debug)]
pub struct Pipeline {
    pub negated: bool,
    pub commands: Vec<Command>, // pipe_sequence
}

#[derive(Debug)]
pub enum Command {
    Simple(SimpleCommand),
    Compound {
        command: CompoundCommand,
        redirects: Vec<Redirect>,
    },
    Function(FunctionDef),
}

#[derive(Debug)]
pub struct SimpleCommand {
    pub assignments: Vec<Assignment>,
    pub name: Option<String>,
    pub words: Vec<String>,
    pub redirects: Vec<Redirect>,
}

#[derive(Debug)]
pub struct Assignment {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Redirect {
    pub fd: Option<u32>,
    pub kind: RedirectKind,
    pub target: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum CmdPrefixItem {
    Assignment(Assignment),
    Redirect(Redirect),
}

#[derive(Debug)]
pub enum CmdSuffixItem {
    Word(String),
    Redirect(Redirect),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectKind {
    Input,      // <
    Output,     // >
    Append,     // >>
    HereDoc,    // <<
    HereDocDash,// <<-
    DupInput,   // <&
    DupOutput,  // >&
    Clobber,    // >|
    Open,       // <>
}

#[derive(Debug)]
pub enum CompoundCommand {
    If(IfClause),
    While(WhileClause),
    Until(UntilClause),
    For(ForClause),
    Case(CaseClause),
    Subshell(Vec<AndOr>),
    BraceGroup(Vec<AndOr>),
}

#[derive(Debug)]
pub struct IfClause {
    pub condition: Vec<AndOr>,
    pub then_branch: Vec<AndOr>,
    pub elif_parts: Vec<ElifPart>,
    pub else_branch: Option<Vec<AndOr>>,
}

#[derive(Debug)]
pub struct ElifPart {
    pub condition: Vec<AndOr>,
    pub then_branch: Vec<AndOr>,
}

#[derive(Debug)]
pub struct WhileClause {
    pub condition: Vec<AndOr>,
    pub body: Vec<AndOr>,
}

#[derive(Debug)]
pub struct UntilClause {
    pub condition: Vec<AndOr>,
    pub body: Vec<AndOr>,
}

#[derive(Debug)]
pub struct ForClause {
    pub var: String,
    pub words: Option<Vec<String>>,
    pub body: Vec<AndOr>,
}

#[derive(Debug)]
pub struct CaseClause {
    pub word: String,
    pub items: Vec<CaseItem>,
}

#[derive(Debug)]
pub struct CaseItem {
    pub patterns: Vec<String>,
    pub body: Vec<AndOr>,
}

#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub body: CompoundCommand,
    pub redirects: Vec<Redirect>,
}

impl Pipeline {
    pub fn is_simple(&self) -> bool {
        self.commands.len() == 1 && !self.negated
    }
}
