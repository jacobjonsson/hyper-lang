use crate::SyntaxKind;
use text_size::TextSize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: TextSize,
}

impl Token {
    pub fn new(kind: SyntaxKind, len: TextSize) -> Token {
        Token { kind, len }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}@{:?}", self.kind, self.len)
    }
}
