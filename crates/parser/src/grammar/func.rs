use super::param::param_list;
use super::stmt::let_stmt;
use super::{expr::expr, name};
use crate::parser::{Marker, Parser};
use syntax::{SyntaxKind, T};

pub(super) fn func(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::FUNC_KW);

    name(parser);

    if parser.at(T!['(']) {
        param_list(parser);
    } else {
        parser.error("expected function arguments");
    }

    if parser.at(T!['{']) {
        func_body(parser);
    } else {
        parser.error("expected function body");
    }

    marker.complete(parser, SyntaxKind::FUNC_KW);
}

fn func_body(parser: &mut Parser) {
    parser.expect(T!['{']);

    while !parser.at(SyntaxKind::EOF) && !parser.at(T!['}']) {
        if parser.at(T![;]) {
            parser.bump(T![;]);
            continue;
        }

        func_body_stmt(parser);
    }

    parser.expect(T!['}']);
}

fn func_body_stmt(parser: &mut Parser) {
    let marker = parser.start();

    if parser.at(SyntaxKind::LET_KW) {
        let_stmt(parser, marker);
        return;
    }

    if parser.at(SyntaxKind::STATE_KW) {
        parser.err_and_bump("functions cannot have state");
        marker.abandon(parser);
        return;
    }

    if parser.at(SyntaxKind::RETURN_KW) {
        func_return_stmt(parser, marker);
        return;
    }

    parser.err_and_bump("expected a valid func body stmt");
    marker.abandon(parser);
}

fn func_return_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::RETURN_KW);

    if parser.at(T![;]) {
        parser.bump(T![;]);
        marker.complete(parser, SyntaxKind::RETURN_STMT);
        return;
    }

    expr(parser);

    parser.expect(T![;]);
    marker.complete(parser, SyntaxKind::RETURN_STMT);
}
