use crate::{support, Name, NameRef};
use syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode: Sized {
    fn can_cast(kind: SyntaxKind) -> bool;

    fn cast(syntax: SyntaxNode) -> Option<Self>;

    fn syntax(&self) -> &SyntaxNode;

    fn clone_for_update(&self) -> Self {
        Self::cast(self.syntax().clone_for_update()).unwrap()
    }

    fn clone_subtree(&self) -> Self {
        Self::cast(self.syntax().clone_subtree()).unwrap()
    }
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken: Sized {
    fn can_cast(token: SyntaxKind) -> bool;

    fn cast(syntax: SyntaxToken) -> Option<Self>;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

pub trait HasName: AstNode {
    fn name(&self) -> Option<Name> {
        support::child(self.syntax())
    }
}

pub trait HasNameRef: AstNode {
    fn name_ref(&self) -> Option<NameRef> {
        support::child(self.syntax())
    }
}
