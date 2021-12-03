use super::{Parser, SyntaxKind};
use syntax::T;

/// Assumes that current token is a left paren.
pub(super) fn param_list(parser: &mut Parser) {
    let marker = parser.start();

    parser.expect(T!['(']);

    while !parser.at(SyntaxKind::EOF) && parser.at(T!['(']) {
        param(parser);

        if !parser.at(T!['(']) {
            parser.expect(T![,]);
        }
    }

    parser.expect(T![')']);

    marker.complete(parser, SyntaxKind::PARAM_LIST);
}

pub(super) fn param(_parser: &mut Parser) {
    todo!()
}
