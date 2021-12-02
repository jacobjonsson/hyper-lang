use crate::{AstNode, HasName};
use syntax::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct Func {
    pub(crate) syntax: SyntaxNode,
}

impl HasName for Func {}

impl AstNode for Func {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Func
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
