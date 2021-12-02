use rowan::TextRange;
use syntax::SyntaxKind;
use text_size::TextSize;

pub struct TokenSource<'a> {
    source: &'a str,
    token_offset_pairs: Vec<(lexer::Token, TextSize)>,
    current: (crate::Token, usize),
}

impl<'a> TokenSource<'a> {
    pub(crate) fn new(source: &'a str, raw_tokens: &[lexer::Token]) -> Self {
        let token_offset_pairs: Vec<_> = raw_tokens
            .iter()
            .filter_map({
                let mut len = 0.into();
                move |token| {
                    let pair = if token.kind.is_trivia() { None } else { Some((*token, len)) };
                    len += token.len;
                    pair
                }
            })
            .collect();

        let first = mk_token(0, &token_offset_pairs);
        TokenSource { source, token_offset_pairs, current: (first, 0) }
    }

    pub(crate) fn current(&self) -> crate::Token {
        self.current.0
    }

    pub(crate) fn lookahead_nth(&self, n: usize) -> crate::Token {
        mk_token(self.current.1 + n, &self.token_offset_pairs)
    }

    pub(crate) fn bump(&mut self) {
        if self.current.0.kind == SyntaxKind::Eof {
            return;
        }

        let pos = self.current.1 + 1;
        self.current = (mk_token(pos, &self.token_offset_pairs), pos);
    }

    /// Returns the raw slice of the current token
    pub(crate) fn text(&self) -> Option<&'a str> {
        self.token_offset_pairs
            .get(self.current.1)
            .map(|(token, offset)| &self.source[TextRange::at(*offset, token.len)])
    }

    pub(crate) fn text_matches(&self, text: &str) -> bool {
        self.text().map_or(false, |s| s == text)
    }
}

fn mk_token(pos: usize, token_offset_pairs: &[(lexer::Token, TextSize)]) -> crate::Token {
    let (kind, is_jointed_to_next) = match token_offset_pairs.get(pos) {
        Some((token, offset)) => (
            token.kind,
            token_offset_pairs.get(pos + 1).map_or(false, |(_, next_offset)| offset + token.len == *next_offset),
        ),
        None => (SyntaxKind::Eof, false),
    };
    crate::Token { kind, is_jointed_to_next }
}
