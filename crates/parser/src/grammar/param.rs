use super::{Parser, SyntaxKind};

/// Assumes that current token is a left paren.
pub(super) fn param_list(parser: &mut Parser) {
    let marker = parser.start();

    parser.bump(SyntaxKind::LeftParen);

    while !parser.at(SyntaxKind::Eof) && parser.at(SyntaxKind::LeftParen) {
        param(parser);

        if !parser.at(SyntaxKind::LeftParen) {
            parser.expect(SyntaxKind::Comma);
        }
    }

    parser.expect(SyntaxKind::RightParen);

    marker.complete(parser, SyntaxKind::ParamList);
}

pub(super) fn param(parser: &mut Parser) {
    todo!()
}
