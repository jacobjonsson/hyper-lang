mod expr;
mod func;
mod item;
mod param;
mod view;

use syntax::SyntaxKind;

use crate::parser::Parser;

pub(crate) fn source_file(parser: &mut Parser) {
    let marker = parser.start();
    item::parse_items(parser);
    marker.complete(parser, SyntaxKind::SourceFile);
}

pub(super) fn name(parser: &mut Parser) {
    if !parser.at(SyntaxKind::Identifier) {
        parser.error("expected an identifier");
        return;
    }

    let marker = parser.start();
    parser.bump(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::Name);
}

#[allow(dead_code)]
pub(super) fn name_ref(parser: &mut Parser) {
    if !parser.at(SyntaxKind::Identifier) {
        parser.error("expected an identifier");
        return;
    }

    let marker = parser.start();
    parser.bump(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::NameRef);
}
