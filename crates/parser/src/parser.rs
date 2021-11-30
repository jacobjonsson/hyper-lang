mod marker;

use crate::{event::Event, token_source::TokenSource, ParseError};
use std::cell::Cell;
use syntax::SyntaxKind;

pub(crate) use self::marker::CompletedMarker;
pub(crate) use self::marker::Marker;

pub(crate) struct Parser {
    token_source: TokenSource,
    events: Vec<Event>,
    steps: Cell<u32>,
}

impl Parser {
    pub fn new(token_source: TokenSource) -> Parser {
        Parser {
            token_source,
            events: Vec::new(),
            steps: Cell::new(0),
        }
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    pub(crate) fn current(&self) -> SyntaxKind {
        self.token_source.current().kind
    }

    /// Lookahead operation: returns the kind of the next nth
    /// token.
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n <= 3);

        let steps = self.steps.get();
        assert!(steps <= 10_000_000, "the parser seems stuck");
        self.steps.set(steps + 1);

        self.token_source.lookahead_nth(n).kind
    }

    /// Checks if the current token is `kind`.
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        match kind {
            _ => self.token_source.lookahead_nth(n).kind == kind,
        }
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        let n_raw_tokens = match kind {
            _ => 1,
        };
        self.do_bump(kind, n_raw_tokens);
        true
    }

    #[allow(dead_code)]
    fn at_composite2(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind) -> bool {
        let t1 = self.token_source.lookahead_nth(n);
        if t1.kind != k1 || !t1.is_jointed_to_next {
            return false;
        }
        let t2 = self.token_source.lookahead_nth(n + 1);
        t2.kind == k2
    }

    #[allow(dead_code)]
    fn at_composite3(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind, k3: SyntaxKind) -> bool {
        let t1 = self.token_source.lookahead_nth(n);
        if t1.kind != k1 || !t1.is_jointed_to_next {
            return false;
        }
        let t2 = self.token_source.lookahead_nth(n + 1);
        if t2.kind != k2 || !t2.is_jointed_to_next {
            return false;
        }
        let t3 = self.token_source.lookahead_nth(n + 2);
        t3.kind == k3
    }

    /// Checks if the current token is in `kinds`.
    pub(crate) fn at_ts(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.contains(&self.current())
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    /// Advances the parser by one token
    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);
        if kind == SyntaxKind::Eof {
            return;
        }
        self.do_bump(kind, 1);
    }

    /// Emit error with the `message`
    /// FIXME: this should be much more fancy and support
    /// structured errors with spans and notes, like rustc
    /// does.
    pub(crate) fn error<T: Into<String>>(&mut self, message: T) {
        let msg = ParseError(message.into());
        self.push_event(Event::Error { msg });
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}", kind));
        false
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_and_bump(&mut self, message: &str) {
        self.err_recover(message, Default::default());
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_recover(&mut self, message: &str, recovery: &[SyntaxKind]) {
        if self.at_ts(recovery) {
            self.error(message);
            return;
        }

        let marker = self.start();
        self.error(message);
        self.bump_any();
        marker.complete(self, SyntaxKind::Error);
    }

    pub(crate) fn do_bump(&mut self, kind: SyntaxKind, n_raw_tokens: u8) {
        for _ in 0..n_raw_tokens {
            self.token_source.bump();
        }

        self.push_event(Event::Token { kind, n_raw_tokens });
    }

    pub(crate) fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }
}
