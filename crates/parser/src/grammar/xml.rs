use super::{
    expr::{expr, literal},
    name_ref,
};
use crate::{parser::Parser, Parse};
use syntax::{SyntaxKind, T};

pub(super) fn xml_element(parser: &mut Parser) {
    let marker = parser.start();

    let (tag_name, self_closing) = xml_opening_element(parser);
    if self_closing {
        marker.complete(parser, SyntaxKind::XML_ELEMENT);
        return;
    }

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
            if !parser.text_matches(tag_name.unwrap_or("")) {
                parser.error(format!("expected closing tag to match {}", tag_name.unwrap_or("")));
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

fn xml_opening_element<'a>(parser: &'a mut Parser) -> (Option<&'a str>, bool) {
    let marker = parser.start();

    parser.expect(T![<]);
    let tag_name = parser.text();
    name_ref(parser);
    xml_attribute_list(parser);

    if parser.at(T![/>]) {
        parser.bump(T![/>]);
        marker.complete(parser, SyntaxKind::XML_SELF_CLOSING_ELEMENT);
        return (Some(""), true);
    }

    parser.expect(T![>]);
    marker.complete(parser, SyntaxKind::XML_OPENING_ELEMENT);
    (tag_name, false)
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
