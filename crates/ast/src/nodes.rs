use crate::{func::Func, support, view::View, AstChildren, AstNode};
use syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

pub struct SourceFile(SyntaxNode);

impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SourceFile
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self(syntax))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl SourceFile {
    pub fn items(&self) -> AstChildren<Item> {
        support::children(self.syntax())
    }
}

#[derive(Debug)]
pub enum Item {
    Func(Func),
    View(View),
}

impl AstNode for Item {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Func => true,
            SyntaxKind::View => true,
            _ => false,
        }
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            let item = match syntax.kind() {
                SyntaxKind::Func => Item::Func(Func { syntax }),
                SyntaxKind::View => Item::View(View { syntax }),
                _ => return None,
            };

            Some(item)
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Func(func) => func.syntax(),
            Item::View(view) => view.syntax(),
        }
    }
}

#[derive(Debug)]
pub struct Name(SyntaxNode);

impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Name
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self(syntax))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl Name {
    pub fn identifier(&self) -> Option<SyntaxToken> {
        support::token(&self.0, SyntaxKind::Identifier)
    }
}

#[derive(Debug)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NameRef
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

impl NameRef {
    pub fn identifier(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax(), SyntaxKind::Identifier)
    }
}
