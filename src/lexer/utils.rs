use super::token;
use token::TokenType;
pub fn is_prefix(set: &[&str], value: &str) -> bool {
    set.iter().any(|op| op.starts_with(value))
}

pub fn is_redirection(t: TokenType) -> bool {
    matches!(
        t,
        TokenType::Less
            | TokenType::Great
            | TokenType::Dless
            | TokenType::Dgreat
            | TokenType::LessAnd
            | TokenType::GreatAnd
            | TokenType::LessGreat
            | TokenType::DlessDash
            | TokenType::Clobber
    )
}

pub fn is_reserved_word(word: &str) -> bool {
    match word {
        "if"
        | "then"
        | "else"
        | "elif"
        | "fi"
        | "do"
        | "done"
        | "case"
        | "esac"
        | "while"
        | "until"
        | "for"
        | "in" => true,
        _ => false
    }
}
pub fn type_is_reserved(t: TokenType) -> bool {
    matches!(
        t,
        TokenType::Bang
            | TokenType::DoToken
            | TokenType::EsacToken
            | TokenType::InToken
            | TokenType::Rbrace
            | TokenType::DoneToken
            | TokenType::FiToken
            | TokenType::ThenToken
            | TokenType::Lbrace
            | TokenType::ElifToken
            | TokenType::ForToken
            | TokenType::UntilToken
            | TokenType::CaseToken
            | TokenType::ElseToken
            | TokenType::IfToken
            | TokenType::WhileToken
    )
}
fn match_direct_token(value: &str) -> Option<TokenType> {
    match value {
        // operators
        "&&" => Some(TokenType::AndIf),
        "||" => Some(TokenType::Dpipe),
        ";;" => Some(TokenType::Dsemi),
        "<<" => Some(TokenType::Dless),
        ">>" => Some(TokenType::Dgreat),
        "<&" => Some(TokenType::LessAnd),
        ">&" => Some(TokenType::GreatAnd),
        "<>" => Some(TokenType::LessGreat),
        "<<-" => Some(TokenType::DlessDash),
        ">|" => Some(TokenType::Clobber),

        "&" => Some(TokenType::Amp),
        "|" => Some(TokenType::Pipe),
        "!" => Some(TokenType::Bang),
        ";" => Some(TokenType::Semi),
        "(" => Some(TokenType::Lpar),
        ")" => Some(TokenType::Rpar),
        "{" => Some(TokenType::Lbrace),
        "}" => Some(TokenType::Rbrace),
        "<" => Some(TokenType::Less),
        ">" => Some(TokenType::Great),

        // keywords
        "if" => Some(TokenType::IfToken),
        "then" => Some(TokenType::ThenToken),
        "else" => Some(TokenType::ElseToken),
        "elif" => Some(TokenType::ElifToken),
        "fi" => Some(TokenType::FiToken),
        "do" => Some(TokenType::DoToken),
        "done" => Some(TokenType::DoneToken),
        "case" => Some(TokenType::CaseToken),
        "esac" => Some(TokenType::EsacToken),
        "while" => Some(TokenType::WhileToken),
        "until" => Some(TokenType::UntilToken),
        "for" => Some(TokenType::ForToken),
        "in" => Some(TokenType::InToken),

        _ => None,
    }
}
fn is_assignment_word(value: &str) -> bool {
    let eq_pos = match value.find('=') {
        Some(p) if p > 0 => p,
        _ => return false,
    };

    let (left, _) = value.split_at(eq_pos);

    if left.is_empty() {
        return false;
    }

    left.chars().all(|c| c.is_alphanumeric() || c == '_')
}
pub fn is_digit_string(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}
pub fn get_word_type(value: &str) -> TokenType {
    if value.is_empty() {
        return TokenType::Error;
    }

    if let Some(t) = match_direct_token(value) {
        return t;
    }

    if is_assignment_word(value) {
        return TokenType::AssignmentWord;
    }

    TokenType::Word
}