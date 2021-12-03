use super::{
    expr::{expr, literal},
    name_ref,
};
use crate::parser::Parser;
use syntax::{SyntaxKind, T};

pub(super) fn xml_element(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(T![<]);
    let tag_name = parser.text().unwrap_or("");
    name_ref(parser);
    xml_attribute_list(parser);

    if parser.at(T![/>]) {
        parser.bump(T![/>]);
        marker.complete(parser, SyntaxKind::XML_ELEMENT);
        return;
    }

    parser.expect(T![>]);

    loop {
        if parser.at(SyntaxKind::EOF) {
            parser.error("unterminated xml element");
            marker.complete(parser, SyntaxKind::XML_ELEMENT);
            return;
        }

        if parser.at(T!['{']) {
            expr(parser);
        }

        // Closing tag
        if parser.at(T![</]) {
            parser.bump(T![</]);
            if !parser.text_matches(tag_name) {
                parser.error(format!("expected closing tag to match {}", tag_name));
            }
            name_ref(parser);
            parser.expect(T![>]);
            marker.complete(parser, SyntaxKind::XML_ELEMENT);
            return;
        }

        // A child xml element
        if parser.at(T![<]) {
            xml_element(parser);
            continue;
        }

        // Anything is valid html text
        parser.bump_any();
    }
}

fn xml_attribute_list(parser: &mut Parser) {
    let marker = parser.start();

    while !parser.at(SyntaxKind::EOF) && !parser.at(T![/>]) && !parser.at(T![>]) {
        xml_attribute(parser);
    }

    marker.complete(parser, SyntaxKind::XML_ATTRIBUTE_LIST);
}

fn xml_attribute(parser: &mut Parser) {
    let marker = parser.start();
    name_ref(parser);
    parser.expect(T![=]);

    if parser.at(SyntaxKind::STRING) {
        literal(parser);
    } else if parser.at(T!['{']) {
        parser.bump(T!['{']);
        expr(parser);
        parser.expect(T!['}']);
    } else {
        parser.error("expected string or an expression wrapped in {}");
    }

    marker.complete(parser, SyntaxKind::XML_ATTRIBUTE);
}
