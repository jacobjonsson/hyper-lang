use super::{token::Token, tokenize};
use syntax::T;

macro_rules! test_token {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let tokens = tokenize($input);
            assert!(tokens.len() == 1);
            let actual = format!("{:?}", tokens[0]);
            assert_eq!(actual, $expected);
        }
    };
}

test_token!(whitespace, "   ", "WHITESPACE@3");
test_token!(single_line_comment, "// abc", "COMMENT@6");

test_token!(string, "\"hello\"", "STRING@7");
test_token!(identifier, "hello", "IDENT@5");
test_token!(integer, "123", "INT@3");
test_token!(r#true, "true", "TRUE_KW@4");
test_token!(r#false, "false", "FALSE_KW@5");

test_token!(left_paren, "(", "L_PAREN@1");
test_token!(right_paren, ")", "R_PAREN@1");
test_token!(left_brace, "{", "L_BRACE@1");
test_token!(right_brace, "}", "R_BRACE@1");
test_token!(left_bracket, "[", "L_BRACKET@1");
test_token!(right_bracket, "]", "R_BRACKET@1");
test_token!(plus, "+", "PLUS@1");
test_token!(minus, "-", "MINUS@1");
test_token!(star, "*", "STAR@1");
test_token!(slash, "/", "SLASH@1");
test_token!(semicolon, ";", "SEMICOLON@1");
test_token!(comma, ",", "COMMA@1");
test_token!(dot, ".", "DOT@1");
test_token!(equals, "=", "EQ@1");
test_token!(less_than, "<", "L_ANGLE@1");
test_token!(greater_than, ">", "R_ANGLE@1");
test_token!(percent, "%", "PERCENT@1");
test_token!(bang, "!", "BANG@1");

test_token!(func, "func", "FUNC_KW@4");
test_token!(view, "view", "VIEW_KW@4");
test_token!(r#let, "let", "LET_KW@3");
test_token!(r#mut, "mut", "MUT_KW@3");
test_token!(state, "state", "STATE_KW@5");
test_token!(r#return, "return", "RETURN_KW@6");

#[test]
fn smoke_test() {
    use syntax::SyntaxKind::*;

    let source = "(){}[]+-/*123func";
    let expected = &[
        Token { kind: T!['('], len: 1.into() },
        Token { kind: T![')'], len: 1.into() },
        Token { kind: T!['{'], len: 1.into() },
        Token { kind: T!['}'], len: 1.into() },
        Token { kind: T!['['], len: 1.into() },
        Token { kind: T![']'], len: 1.into() },
        Token { kind: T![+], len: 1.into() },
        Token { kind: T![-], len: 1.into() },
        Token { kind: T![/], len: 1.into() },
        Token { kind: T![*], len: 1.into() },
        Token { kind: INT, len: 3.into() },
        Token { kind: FUNC_KW, len: 4.into() },
    ];

    let actual = tokenize(source);
    assert_eq!(actual, expected);
}
