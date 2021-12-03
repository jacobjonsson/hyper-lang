#[cfg(test)]
mod test;

mod cursor;
mod token;

use self::cursor::Cursor;
use syntax::{SyntaxKind, T};
pub use token::Token;

pub fn tokenize(mut input: &str) -> Vec<Token> {
    if input.is_empty() {
        return Default::default();
    }

    let mut tokens = Vec::new();

    while !input.is_empty() {
        let token = first_token(input);
        input = &input[token.len.into()..];
        tokens.push(token);
    }

    tokens
}

fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

impl<'a> Cursor<'a> {
    pub fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();

        let kind = match first_char {
            '(' => T!['('],
            ')' => T![')'],
            '{' => T!['{'],
            '}' => T!['}'],
            '[' => T!['['],
            ']' => T![']'],
            '%' => T![%],
            '+' => T![+],
            '-' => T![-],
            '*' => T![*],
            ';' => T![;],
            '=' => T![=],
            '.' => T![.],
            ',' => T![,],
            '<' => T![<],
            '>' => T![>],
            '!' => T![!],
            '/' => {
                if self.first() == '/' {
                    self.bump();
                    self.single_line_comment()
                } else {
                    T![/]
                }
            }
            '"' => self.string(),
            '0'..='9' => self.integer(),
            ch if is_whitespace(ch) => self.whitespace(),
            ch if is_identifier_star(ch) => self.identifier_or_keyword(),
            _ => SyntaxKind::ERROR,
        };

        Token::new(kind, (self.len_consumed() as u32).into())
    }

    // Assumes first and second / is consumed.
    fn single_line_comment(&mut self) -> SyntaxKind {
        loop {
            match self.first() {
                '\0' => {
                    break;
                }
                '\n' => {
                    self.bump();
                    break;
                }
                _ => {
                    self.bump();
                }
            }
        }

        SyntaxKind::COMMENT
    }

    fn whitespace(&mut self) -> SyntaxKind {
        while is_whitespace(self.first()) {
            self.bump();
        }

        SyntaxKind::WHITESPACE
    }

    fn integer(&mut self) -> SyntaxKind {
        while self.first().is_digit(10) {
            self.bump();
        }

        SyntaxKind::INT
    }

    fn string(&mut self) -> SyntaxKind {
        loop {
            match self.first() {
                '"' => {
                    self.bump();
                    return SyntaxKind::STRING;
                }
                '\\' => {
                    self.bump();
                    self.bump();
                }
                '\0' => return SyntaxKind::ERROR,
                _ => {
                    self.bump();
                }
            }
        }
    }

    fn identifier_or_keyword(&mut self) -> SyntaxKind {
        while is_identifier_cont(self.first()) {
            self.bump();
        }

        let text = self.text();

        SyntaxKind::from_keyword(text).unwrap_or(SyntaxKind::IDENT)
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

fn is_identifier_star(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || ch == '$'
}

fn is_identifier_cont(ch: char) -> bool {
    is_identifier_star(ch) || ch.is_digit(10)
}
