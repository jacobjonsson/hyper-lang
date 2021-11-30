mod item;
mod param;

use syntax::SyntaxKind;

use crate::parser::Parser;

pub(crate) fn source_file(parser: &mut Parser) {
    let marker = parser.start();
    item::parse_items(parser);
    marker.complete(parser, SyntaxKind::SourceFile);
}
