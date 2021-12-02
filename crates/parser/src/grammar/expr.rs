use crate::parser::CompletedMarker;
use crate::parser::Parser;
use crate::token_set::TokenSet;
use syntax::SyntaxKind;

pub(super) fn expr(parser: &mut Parser) {
    expr_bp(parser, 1);
}

/// Operator precedence of the current token
/// Follows the operator precedence of javascript
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table
fn current_op(parser: &mut Parser) -> (u8, SyntaxKind) {
    match parser.current() {
        SyntaxKind::Star => (13, SyntaxKind::Star),
        SyntaxKind::Slash => (13, SyntaxKind::Slash),
        SyntaxKind::Percent => (13, SyntaxKind::Percent),
        SyntaxKind::Plus => (12, SyntaxKind::Plus),
        SyntaxKind::Minus => (12, SyntaxKind::Plus),
        SyntaxKind::LessThan => (11, SyntaxKind::LessThan),
        SyntaxKind::GreaterThan => (11, SyntaxKind::GreaterThan),
        SyntaxKind::Equals => (2, SyntaxKind::Equals),
        SyntaxKind::Comma => (1, SyntaxKind::Comma),

        _ => (0, parser.current()),
    }
}

/// Parses an expression with the given binding power
fn expr_bp(parser: &mut Parser, bp: u8) {
    let marker = parser.start();

    let mut lhs = match lhs(parser) {
        Some(lhs) => lhs.extend_to(parser, marker),
        None => {
            marker.abandon(parser);
            return;
        }
    };

    loop {
        let (op_bp, op) = current_op(parser);
        if op_bp < bp {
            break;
        }

        let marker = lhs.precede(parser);
        parser.bump(op);

        expr_bp(parser, op_bp + 1);
        lhs = marker.complete(parser, SyntaxKind::BinaryExpr);
    }
}

pub(crate) const UNARY_FIRST: TokenSet = TokenSet::new(&[SyntaxKind::Bang, SyntaxKind::Plus, SyntaxKind::Minus]);

/// Parses the left hand side of an expression
fn lhs(parser: &mut Parser) -> Option<CompletedMarker> {
    if let Some(m) = literal(parser) {
        return Some(m);
    }

    if parser.at_ts(UNARY_FIRST) {
        let marker = parser.start();
        parser.bump_any();
        expr_bp(parser, 255);
        let done = marker.complete(parser, SyntaxKind::UnaryExpr);
        return Some(done);
    }

    let done = match parser.current() {
        SyntaxKind::Identifier => {
            let marker = parser.start();
            parser.bump(SyntaxKind::Identifier);
            marker.complete(parser, SyntaxKind::NameRef)
        }
        _ => return None,
    };

    Some(done)
}

pub(crate) const LITERAL_FIRST: TokenSet =
    TokenSet::new(&[SyntaxKind::True, SyntaxKind::False, SyntaxKind::String, SyntaxKind::Integer]);

pub(super) fn literal(parser: &mut Parser) -> Option<CompletedMarker> {
    if !parser.at_ts(LITERAL_FIRST) {
        return None;
    }
    let marker = parser.start();
    parser.bump_any();
    Some(marker.complete(parser, SyntaxKind::Literal))
}
