use super::param::param_list;
use super::stmt::state_stmt;
use super::xml;
use super::{name, stmt::let_stmt};
use crate::parser::{Marker, Parser};
use syntax::{SyntaxKind, T};

pub(super) fn view(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::VIEW_KW);

    name(parser);

    if parser.at(T!['(']) {
        param_list(parser);
    } else {
        parser.error("expected view arguments");
    }

    if parser.at(T!['{']) {
        view_body(parser);
    } else {
        parser.error("expected view body");
    }

    marker.complete(parser, SyntaxKind::VIEW_KW);
}

fn view_body(parser: &mut Parser) {
    parser.expect(T!['{']);

    while !parser.at(SyntaxKind::EOF) && !parser.at(T!['}']) {
        if parser.at(T![,]) {
            parser.bump(T![,]);
            continue;
        }

        view_body_stmt(parser);
    }

    parser.expect(T!['}']);
}

fn view_body_stmt(parser: &mut Parser) {
    let marker = parser.start();

    if parser.at(SyntaxKind::LET_KW) {
        let_stmt(parser, marker);
        return;
    }

    if parser.at(SyntaxKind::STATE_KW) {
        state_stmt(parser, marker);
        return;
    }

    if parser.at(SyntaxKind::RETURN_KW) {
        view_return_stmt(parser, marker);
        return;
    }

    parser.err_and_bump("expected a valid view body stmt");
    marker.abandon(parser);
}

fn view_return_stmt(parser: &mut Parser, marker: Marker) {
    parser.bump(SyntaxKind::RETURN_KW);

    if parser.at(T!['(']) {
        parser.bump(T!['(']);
    }

    xml::xml_element(parser);

    if parser.at(T![')']) {
        parser.bump(T![')']);
    }

    if parser.at(T![;]) {
        parser.bump(T![;]);
    }

    marker.complete(parser, SyntaxKind::RETURN_KW);
}
