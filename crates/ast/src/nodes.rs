use crate::{support, traits::HasName, AstChildren, AstNode};
use syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

pub struct SourceFile(SyntaxNode);

impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::SourceFile
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
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
}

impl AstNode for Item {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Func => true,
            _ => false,
        }
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::can_cast(syntax.kind()) {
            let item = match syntax.kind() {
                SyntaxKind::Func => Item::Func(Func(syntax)),
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
        }
    }
}

#[derive(Debug)]
pub struct Func(SyntaxNode);

impl HasName for Func {}

impl AstNode for Func {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Func
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
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

#[derive(Debug)]
pub struct View(SyntaxNode);

impl HasName for View {}

impl AstNode for View {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::View
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
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

#[derive(Debug)]
pub struct Name(SyntaxNode);

impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::Name
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
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
