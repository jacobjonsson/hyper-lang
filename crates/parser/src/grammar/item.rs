use crate::parser::Parser;
use syntax::SyntaxKind;

use super::func::func;
use super::view::view;

pub(super) fn parse_items(parser: &mut Parser) {
    while !parser.at(SyntaxKind::EOF) {
        let marker = parser.start();

        if parser.at(SyntaxKind::FUNC_KW) {
            func(parser, marker);
            continue;
        }

        if parser.at(SyntaxKind::VIEW_KW) {
            view(parser, marker);
            continue;
        }

        parser.err_and_bump("expected an item");
        marker.abandon(parser);
    }
}
