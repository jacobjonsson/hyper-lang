use syntax::{SyntaxKind, SyntaxToken};

pub struct Identifier(SyntaxToken);

impl Identifier {
    pub fn can_cast(token: &SyntaxToken) -> bool {
        token.kind() == SyntaxKind::Identifier
    }

    pub fn cast(token: SyntaxToken) -> Option<Self> {
        if Self::can_cast(&token) {
            Some(Self(token))
        } else {
            None
        }
    }

    pub fn value(&self) -> &str {
        self.0.text()
    }

    pub fn syntax(&self) -> &SyntaxToken {
        &self.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax(), f)
    }
}
