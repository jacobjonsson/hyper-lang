use crate::{support, AstChildren, AstNode, HasName, NameRef};
use syntax::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct View {
    pub(crate) syntax: SyntaxNode,
}

impl View {
    pub fn body(&self) -> Option<ViewBody> {
        support::child(self.syntax())
    }
}

impl HasName for View {}

impl AstNode for View {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::View
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

#[derive(Debug)]
pub struct ViewBody {
    pub(crate) syntax: SyntaxNode,
}

impl ViewBody {
    pub fn statements(&self) -> AstChildren<ViewStatement> {
        support::children(self.syntax())
    }
}

impl AstNode for ViewBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ViewBody
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

#[derive(Debug)]
pub enum ViewStatement {
    Return(ViewReturn),
}

impl AstNode for ViewStatement {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::Return => true,
            _ => false,
        }
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::Return => Some(Self::Return(ViewReturn { syntax })),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}

#[derive(Debug)]
pub struct ViewReturn {
    pub(crate) syntax: SyntaxNode,
}

impl AstNode for ViewReturn {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Return
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

#[derive(Debug)]
pub struct XmlElement {
    pub(crate) syntax: SyntaxNode,
}

impl XmlElement {
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(self.syntax())
    }
}

impl AstNode for XmlElement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::XmlElement
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
