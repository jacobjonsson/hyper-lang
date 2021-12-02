#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    Tombstone,
    #[doc(hidden)]
    Eof,
    Error,

    // Tokens
    Whitespace,
    Comment,

    String,
    Identifier,
    Integer,
    True,
    False,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Semicolon,
    Comma,
    Dot,
    Equals,
    LessThan,
    GreaterThan,
    Bang,

    // Keywords
    Func,
    View,
    Let,
    Mut,
    State,
    Return,

    // Composite tokens
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    LessThanSlash,
    SlashGreaterThan,
    EqualsEquals,
    BangEquals,

    // Nodes
    SourceFile,
    Name,
    NameRef,
    ParamList,
    Param,
    FuncBody,
    ViewBody,
    ReturnStmt,
    Literal,
    BinaryExpr,
    UnaryExpr,
    LetStmt,
    StateStmt,
    XmlElement,
    XmlAttributeList,
    XmlAttribute,

    #[doc(hidden)]
    __LAST,
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}

impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::Whitespace | SyntaxKind::Comment)
    }
}
