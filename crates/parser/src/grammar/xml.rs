use super::{
    expr::{expr, literal},
    name_ref,
};
use crate::parser::Parser;
use syntax::SyntaxKind;

pub(super) fn xml_element(parser: &mut Parser) {
    let marker = parser.start();
    parser.expect(SyntaxKind::LessThan);
    let tag_name = parser.text().unwrap_or("");
    name_ref(parser);
    xml_attribute_list(parser);

    if parser.at(SyntaxKind::SlashGreaterThan) {
        parser.bump(SyntaxKind::SlashGreaterThan);
        marker.complete(parser, SyntaxKind::XmlElement);
        return;
    }

    parser.expect(SyntaxKind::GreaterThan);

    loop {
        if parser.at(SyntaxKind::Eof) {
            parser.error("unterminated xml element");
            marker.complete(parser, SyntaxKind::XmlElement);
            return;
        }

        if parser.at(SyntaxKind::LeftBrace) {
            expr(parser);
        }

        // Closing tag
        if parser.at(SyntaxKind::LessThanSlash) {
            parser.bump(SyntaxKind::LessThanSlash);
            if !parser.text_matches(tag_name) {
                parser.error(format!("expected closing tag to match {}", tag_name));
            }
            name_ref(parser);
            parser.expect(SyntaxKind::GreaterThan);
            marker.complete(parser, SyntaxKind::XmlElement);
            return;
        }

        // A child xml element
        if parser.at(SyntaxKind::LessThan) {
            xml_element(parser);
            continue;
        }

        // Anything is valid html text
        parser.bump_any();
    }
}

fn xml_attribute_list(parser: &mut Parser) {
    let marker = parser.start();

    while !parser.at(SyntaxKind::Eof) && !parser.at(SyntaxKind::SlashGreaterThan) && !parser.at(SyntaxKind::GreaterThan)
    {
        xml_attribute(parser);
    }

    marker.complete(parser, SyntaxKind::XmlAttributeList);
}

fn xml_attribute(parser: &mut Parser) {
    let marker = parser.start();
    name_ref(parser);
    parser.expect(SyntaxKind::Equals);

    if parser.at(SyntaxKind::String) {
        literal(parser);
    } else if parser.at(SyntaxKind::LeftBrace) {
        parser.bump(SyntaxKind::LeftBrace);
        expr(parser);
        parser.expect(SyntaxKind::RightBrace);
    } else {
        parser.error("expected string or an expression wrapped in {}");
    }

    marker.complete(parser, SyntaxKind::XmlAttribute);
}
