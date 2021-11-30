use crate::parser::Marker;
use crate::parser::Parser;
use syntax::SyntaxKind;

use super::param::*;

pub(super) fn parse_items(parser: &mut Parser) {
    while !parser.at(SyntaxKind::Eof) {
        let marker = parser.start();

        if parser.at(SyntaxKind::Func) {
            parse_func(parser, marker);
            continue;
        }

        if parser.at(SyntaxKind::View) {
            parse_view(parser, marker);
            continue;
        }

        parser.err_and_bump("expected an item");
        marker.abandon(parser);
    }
}

fn parse_func(parser: &mut Parser, marker: Marker) {
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
    let marker = parser.start();
    parser.bump(SyntaxKind::LeftBrace);

    while !parser.at(SyntaxKind::Eof) && !parser.at(SyntaxKind::RightBrace) {
        func_body_statement(parser);
        if !parser.at(SyntaxKind::RightBrace) {
            parser.expect(SyntaxKind::Semicolon);
        }
    }

    parser.expect(SyntaxKind::RightBrace);

    marker.complete(parser, SyntaxKind::FuncBody);
}

fn func_body_statement(parser: &mut Parser) {
    todo!()
}

fn parse_view(parser: &mut Parser, marker: Marker) {
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
    let marker = parser.start();
    parser.bump(SyntaxKind::LeftBrace);

    while !parser.at(SyntaxKind::Eof) && !parser.at(SyntaxKind::RightBrace) {
        view_body_statement(parser);
        if !parser.at(SyntaxKind::RightBrace) {
            parser.expect(SyntaxKind::Semicolon);
        }
    }

    parser.expect(SyntaxKind::RightBrace);

    marker.complete(parser, SyntaxKind::ViewBody);
}

fn view_body_statement(parser: &mut Parser) {
    todo!()
}

fn name(parser: &mut Parser) {
    let marker = parser.start();
    parser.bump(SyntaxKind::Identifier);
    marker.complete(parser, SyntaxKind::Name);
}
