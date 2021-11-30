use crate::{support, AstNode, Name};

pub trait HasName: AstNode {
    fn name(&self) -> Option<Name> {
        support::child(self.syntax())
    }
}
