use super::expr::expr;
use super::name;
use crate::parser::Marker;
use crate::parser::Parser;
use syntax::SyntaxKind;

pub(super) fn let_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::Let);

    if parser.at(SyntaxKind::Mut) {
        parser.bump(SyntaxKind::Mut);
    }

    name(parser);

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump(SyntaxKind::Semicolon);
        marker.complete(parser, SyntaxKind::LetStmt);
        return;
    }

    parser.expect(SyntaxKind::Equals);

    expr(parser);

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump(SyntaxKind::Semicolon);
    }

    marker.complete(parser, SyntaxKind::LetStmt);
}

pub(super) fn state_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::State);

    name(parser);

    parser.expect(SyntaxKind::Equals);

    expr(parser);

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump(SyntaxKind::Semicolon);
    }

    marker.complete(parser, SyntaxKind::StateStmt);
}
