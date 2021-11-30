use crate::ParseError;
use lexer::Token;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::mem;
use syntax::{HyperLanguage, SyntaxKind};
use text_size::{TextRange, TextSize};

enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

pub struct TreeSink<'a> {
    text: &'a str,
    tokens: &'a [Token],
    text_pos: TextSize,
    token_pos: usize,
    state: State,
    errors: Vec<ParseError>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> TreeSink<'a> {
    pub(crate) fn new(text: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            text,
            text_pos: 0.into(),
            token_pos: 0,
            tokens,
            state: State::PendingStart,
            errors: Vec::new(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub(crate) fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.builder.finish_node(),
            State::Normal => (),
        };
        self.eat_trivias();
        let n_tokens = n_tokens as usize;
        let len = self.tokens[self.token_pos..self.token_pos + n_tokens]
            .iter()
            .map(|it| it.len)
            .sum::<TextSize>();
        self.do_token(kind, len, n_tokens);
    }

    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => {
                self.builder.start_node(HyperLanguage::kind_to_raw(kind));
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingFinish => self.builder.finish_node(),
            State::Normal => (),
        }

        self.eat_trivias();
        self.builder.start_node(HyperLanguage::kind_to_raw(kind));
    }

    pub(crate) fn finish_node(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.builder.finish_node(),
            State::Normal => (),
        }
    }

    pub(crate) fn finish(mut self) -> (GreenNode, Vec<ParseError>) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingFinish => {
                self.builder.finish_node();
            }
            State::PendingStart | State::Normal => unreachable!(),
        };

        (self.builder.finish(), self.errors)
    }

    pub(crate) fn error(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    fn eat_trivias(&mut self) {
        while let Some(token) = self.tokens.get(self.token_pos) {
            if !token.kind.is_trivia() {
                break;
            }
            self.do_token(token.kind, token.len, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, len: TextSize, n_tokens: usize) {
        let range = TextRange::at(self.text_pos, len);
        let text = &self.text[range];
        self.text_pos += len;
        self.token_pos += n_tokens;
        self.builder.token(HyperLanguage::kind_to_raw(kind), text);
    }
}
