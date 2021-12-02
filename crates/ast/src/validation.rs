use std::fmt;

use syntax::SyntaxNode;
use text_size::TextRange;

pub struct ValidationError {
    kind: ValidationErrorKind,
    range: TextRange,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error at {}..{}: {}", u32::from(self.range.start()), u32::from(self.range.end()), self.kind)
    }
}

pub enum ValidationErrorKind {
    FuncMissingName,
    ViewMissingName,
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ValidationErrorKind::FuncMissingName => "function is missing name",
            ValidationErrorKind::ViewMissingName => "view is missing name",
        };

        write!(f, "{}", msg)
    }
}

pub fn validate(node: &SyntaxNode) -> Vec<ValidationError> {
    let errors = Vec::new();

    for _ in node.descendants() {}

    errors
}
