use super::{token::Token, tokenize};

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

test_token!(whitespace, "   ", "Whitespace@3");
test_token!(single_line_comment, "// abc", "Comment@6");

test_token!(string, "\"hello\"", "String@7");
test_token!(identifier, "hello", "Identifier@5");
test_token!(integer, "123", "Integer@3");

test_token!(left_paren, "(", "LeftParen@1");
test_token!(right_paren, ")", "RightParen@1");
test_token!(left_brace, "{", "LeftBrace@1");
test_token!(right_brace, "}", "RightBrace@1");
test_token!(left_bracket, "[", "LeftBracket@1");
test_token!(right_bracket, "]", "RightBracket@1");
test_token!(plus, "+", "Plus@1");
test_token!(minus, "-", "Minus@1");
test_token!(star, "*", "Star@1");
test_token!(slash, "/", "Slash@1");
test_token!(semicolon, ";", "Semicolon@1");
test_token!(comma, ",", "Comma@1");
test_token!(dot, ".", "Dot@1");
test_token!(equals, "=", "Equals@1");
test_token!(less_than, "<", "LessThan@1");
test_token!(greater_than, ">", "GreaterThan@1");

test_token!(func, "func", "Func@4");
test_token!(view, "view", "View@4");
test_token!(r#let, "let", "Let@3");
test_token!(r#mut, "mut", "Mut@3");
test_token!(state, "state", "State@5");

#[test]
fn smoke_test() {
    use syntax::SyntaxKind::*;

    let source = "(){}[]+-/*123func";
    let expected = &[
        Token {
            kind: LeftParen,
            len: 1.into(),
        },
        Token {
            kind: RightParen,
            len: 1.into(),
        },
        Token {
            kind: LeftBrace,
            len: 1.into(),
        },
        Token {
            kind: RightBrace,
            len: 1.into(),
        },
        Token {
            kind: LeftBracket,
            len: 1.into(),
        },
        Token {
            kind: RightBracket,
            len: 1.into(),
        },
        Token {
            kind: Plus,
            len: 1.into(),
        },
        Token {
            kind: Minus,
            len: 1.into(),
        },
        Token {
            kind: Slash,
            len: 1.into(),
        },
        Token {
            kind: Star,
            len: 1.into(),
        },
        Token {
            kind: Integer,
            len: 3.into(),
        },
        Token {
            kind: Func,
            len: 4.into(),
        },
    ];

    let actual = tokenize(source);
    assert_eq!(actual, expected);
}
