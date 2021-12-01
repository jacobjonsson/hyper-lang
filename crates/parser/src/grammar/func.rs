use super::param::param_list;
use super::stmt::let_stmt;
use super::{expr::expr, name};
use crate::parser::{Marker, Parser};
use syntax::SyntaxKind;

pub(super) fn func(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::Func);

    name(parser);

    if parser.at(SyntaxKind::LeftParen) {
        param_list(parser);
    } else {
        parser.error("expected function arguments");
    }

    if parser.at(SyntaxKind::LeftBrace) {
        func_body(parser);
    } else {
        parser.error("expected function body");
    }

    marker.complete(parser, SyntaxKind::Func);
}

fn func_body(parser: &mut Parser) {
    parser.expect(SyntaxKind::LeftBrace);

    while !parser.at(SyntaxKind::Eof) && !parser.at(SyntaxKind::RightBrace) {
        if parser.at(SyntaxKind::Semicolon) {
            parser.bump(SyntaxKind::Semicolon);
            continue;
        }

        func_body_stmt(parser);
    }

    parser.expect(SyntaxKind::RightBrace);
}

fn func_body_stmt(parser: &mut Parser) {
    let marker = parser.start();

    if parser.at(SyntaxKind::Let) {
        let_stmt(parser, marker);
        return;
    }

    if parser.at(SyntaxKind::State) {
        parser.err_and_bump("functions cannot have state");
        marker.abandon(parser);
        return;
    }

    if parser.at(SyntaxKind::Return) {
        func_return_stmt(parser, marker);
        return;
    }

    parser.err_and_bump("expected a valid func body stmt");
    marker.abandon(parser);
}

fn func_return_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::Return);

    if parser.at(SyntaxKind::Semicolon) {
        parser.bump(SyntaxKind::Semicolon);
        marker.complete(parser, SyntaxKind::ReturnStmt);
        return;
    }

    expr(parser);

    parser.expect(SyntaxKind::Semicolon);
    marker.complete(parser, SyntaxKind::ReturnStmt);
}
