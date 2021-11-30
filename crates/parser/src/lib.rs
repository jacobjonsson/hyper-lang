#[cfg(test)]
mod tests;

mod event;
mod grammar;
mod parser;
mod token_source;
mod tree_sink;

use std::sync::Arc;

use rowan::GreenNode;
use syntax::{SyntaxKind, SyntaxNode};
use token_source::TokenSource;
use tree_sink::TreeSink;

use crate::parser::Parser;

pub fn parse(source: &str) -> Parse {
    let raw_tokens = lexer::tokenize(source);
    let token_source = TokenSource::new(source, &raw_tokens);
    let mut parser = Parser::new(token_source);
    grammar::source_file(&mut parser);
    let events = parser.finish();
    let mut tree_sink = TreeSink::new(source, &raw_tokens);
    event::process(&mut tree_sink, events);
    let (green, errors) = tree_sink.finish();
    Parse::new(green, errors)
}

#[derive(Debug, PartialEq)]
pub struct ParseError(pub String);

/// `Token` abstracts the cursor of `TokenSource` operates on.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Token {
    /// What is the current token?
    pub kind: SyntaxKind,

    /// Is the current token joined to the next one (`> >` vs `>>`).
    pub is_jointed_to_next: bool,
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn new(green: GreenNode, errors: Vec<ParseError>) -> Self {
        Self {
            green_node: green,
            errors,
        }
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn debug_tree(&self) -> String {
        format!("{:#?}", self.syntax())
    }

    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn green_node(&self) -> &GreenNode {
        &self.green_node
    }

    pub fn ok(self) -> Result<SyntaxNode, Vec<ParseError>> {
        if self.errors.is_empty() {
            Ok(self.syntax())
        } else {
            Err(self.errors)
        }
    }
}
