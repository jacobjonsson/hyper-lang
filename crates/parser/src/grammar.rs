mod expr;
mod func;
mod item;
mod param;
mod stmt;
mod view;
mod xml;

use crate::parser::Parser;
use syntax::SyntaxKind;

pub(crate) fn source_file(parser: &mut Parser) {
    let marker = parser.start();
    item::parse_items(parser);
    marker.complete(parser, SyntaxKind::SOURCE_FILE);
}

pub(super) fn name(parser: &mut Parser) {
    if !parser.at(SyntaxKind::IDENT) {
        parser.error("expected an identifier");
        return;
    }

    let marker = parser.start();
    parser.bump(SyntaxKind::IDENT);
    marker.complete(parser, SyntaxKind::NAME);
}

pub(super) fn name_ref(parser: &mut Parser) {
    if !parser.at(SyntaxKind::IDENT) {
        parser.error("expected an identifier");
        return;
    }

    let marker = parser.start();
    parser.bump(SyntaxKind::IDENT);
    marker.complete(parser, SyntaxKind::NAME_REF);
}
