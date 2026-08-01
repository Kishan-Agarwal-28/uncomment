#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Code,
    LineComment,
    BlockComment,
    StringLiteral,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

impl Token {
    pub fn new(kind: TokenKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}
