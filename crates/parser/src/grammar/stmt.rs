use super::expr::expr;
use super::name;
use crate::parser::Marker;
use crate::parser::Parser;
use syntax::SyntaxKind;
use syntax::T;

pub(super) fn let_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::LET_KW);

    if parser.at(SyntaxKind::MUT_KW) {
        parser.bump(SyntaxKind::MUT_KW);
    }

    name(parser);

    if parser.at(T![;]) {
        parser.bump(T![;]);
        marker.complete(parser, SyntaxKind::LET_STMT);
        return;
    }

    parser.expect(T![=]);

    expr(parser);

    if parser.at(T![;]) {
        parser.bump(T![;]);
    }

    marker.complete(parser, SyntaxKind::LET_STMT);
}

pub(super) fn state_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::STATE_KW);

    name(parser);

    parser.expect(T![=]);

    expr(parser);

    if parser.at(T![;]) {
        parser.bump(T![;]);
    }

    marker.complete(parser, SyntaxKind::STATE_STMT);
}
