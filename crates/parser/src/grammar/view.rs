use super::param::param_list;
use super::stmt::state_stmt;
use super::xml;
use super::{name, stmt::let_stmt};
use crate::parser::{Marker, Parser};
use syntax::SyntaxKind;

pub(super) fn view(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::View);

    name(parser);

    if parser.at(SyntaxKind::LeftParen) {
        param_list(parser);
    } else {
        parser.error("expected view arguments");
    }

    if parser.at(SyntaxKind::LeftBrace) {
        view_body(parser);
    } else {
        parser.error("expected view body");
    }

    marker.complete(parser, SyntaxKind::View);
}

fn view_body(parser: &mut Parser) {
    parser.expect(SyntaxKind::LeftBrace);

    while !parser.at(SyntaxKind::Eof) && !parser.at(SyntaxKind::RightBrace) {
        if parser.at(SyntaxKind::Semicolon) {
            parser.bump(SyntaxKind::Semicolon);
            continue;
        }

        view_body_stmt(parser);
    }

    parser.expect(SyntaxKind::RightBrace);
}

fn view_body_stmt(parser: &mut Parser) {
    let marker = parser.start();

    if parser.at(SyntaxKind::Let) {
        let_stmt(parser, marker);
        return;
    }

    if parser.at(SyntaxKind::State) {
        state_stmt(parser, marker);
        return;
    }

    if parser.at(SyntaxKind::Return) {
        view_return_stmt(parser, marker);
        return;
    }

    parser.err_and_bump("expected a valid view body stmt");
    marker.abandon(parser);
}

fn view_return_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::Return);

    if parser.at(SyntaxKind::LeftParen) {
        parser.bump(SyntaxKind::LeftParen);
    }

    xml::xml_element(parser);

    if parser.at(SyntaxKind::RightParen) {
        parser.bump(SyntaxKind::RightParen);
    }

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump(SyntaxKind::Semicolon);
    }

    marker.complete(parser, SyntaxKind::ReturnStmt);
}
