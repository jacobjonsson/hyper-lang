//! Generated file, do not edit by hand

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    L_BRACKET,
    R_BRACKET,
    SEMICOLON,
    COLON,
    COMMA,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    DOT,
    EQ,
    EQ_EQ,
    BANG_EQ,
    L_ANGLE,
    R_ANGLE,
    BANG,
    L_ANGLE_SLASH,
    SLASH_R_ANGLE,
    PLUS_EQ,
    MINUS_EQ,
    STAR_EQ,
    SLASH_EQ,
    FUNC_KW,
    VIEW_KW,
    STATE_KW,
    LET_KW,
    MUT_KW,
    RETURN_KW,
    TRUE_KW,
    FALSE_KW,
    INT,
    STRING,
    IDENT,
    ERROR,
    COMMENT,
    WHITESPACE,
    SOURCE_FILE,
    NAME,
    NAME_REF,
    PARAM_LIST,
    PARAM,
    FUNC_BODY,
    VIEW_BODY,
    RETURN_STMT,
    LITERAL,
    BINARY_EXPR,
    UNARY_EXPR,
    LET_STMT,
    STATE_STMT,
    XML_ELEMENT,
    XML_ATTRIBUTE_LIST,
    XML_ATTRIBUTE,
    IDENT_PATTERN,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            FUNC_KW | VIEW_KW | STATE_KW | LET_KW | MUT_KW | RETURN_KW | TRUE_KW | FALSE_KW => true,
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool {
        match self {
            L_PAREN | R_PAREN | L_BRACE | R_BRACE | L_BRACKET | R_BRACKET | SEMICOLON | COLON | COMMA | PLUS
            | MINUS | STAR | SLASH | PERCENT | DOT | EQ | EQ_EQ | BANG_EQ | L_ANGLE | R_ANGLE | BANG
            | L_ANGLE_SLASH | SLASH_R_ANGLE | PLUS_EQ | MINUS_EQ | STAR_EQ | SLASH_EQ => true,
            _ => false,
        }
    }
    pub fn is_literal(self) -> bool {
        match self {
            INT | STRING => true,
            _ => false,
        }
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_BRACE,
            '}' => R_BRACE,
            '[' => L_BRACKET,
            ']' => R_BRACKET,
            ';' => SEMICOLON,
            ':' => COLON,
            ',' => COMMA,
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '/' => SLASH,
            '%' => PERCENT,
            '.' => DOT,
            '=' => EQ,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '!' => BANG,
            _ => return None,
        };
        Some(tok)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "func" => FUNC_KW,
            "view" => VIEW_KW,
            "state" => STATE_KW,
            "let" => LET_KW,
            "mut" => MUT_KW,
            "return" => RETURN_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn to_string(&self) -> Option<&str> {
        let tok = match self {
            L_PAREN => "'('",
            R_PAREN => "')'",
            L_BRACE => "'{'",
            R_BRACE => "'}'",
            L_BRACKET => "'['",
            R_BRACKET => "']'",
            SEMICOLON => ";",
            COLON => ":",
            COMMA => ",",
            PLUS => "+",
            MINUS => "-",
            STAR => "*",
            SLASH => "/",
            PERCENT => "%",
            DOT => ".",
            EQ => "=",
            EQ_EQ => "==",
            BANG_EQ => "!=",
            L_ANGLE => "<",
            R_ANGLE => ">",
            BANG => "!",
            L_ANGLE_SLASH => "</",
            SLASH_R_ANGLE => "/>",
            PLUS_EQ => "+=",
            MINUS_EQ => "-=",
            STAR_EQ => "*=",
            SLASH_EQ => "/=",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: SyntaxKind :: L_BRACE } ; ['}'] => { $ crate :: SyntaxKind :: R_BRACE } ; ['['] => { $ crate :: SyntaxKind :: L_BRACKET } ; [']'] => { $ crate :: SyntaxKind :: R_BRACKET } ; [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [==] => { $ crate :: SyntaxKind :: EQ_EQ } ; [!=] => { $ crate :: SyntaxKind :: BANG_EQ } ; [<] => { $ crate :: SyntaxKind :: L_ANGLE } ; [>] => { $ crate :: SyntaxKind :: R_ANGLE } ; [!] => { $ crate :: SyntaxKind :: BANG } ; [</] => { $ crate :: SyntaxKind :: L_ANGLE_SLASH } ; [/>] => { $ crate :: SyntaxKind :: SLASH_R_ANGLE } ; [+=] => { $ crate :: SyntaxKind :: PLUS_EQ } ; [-=] => { $ crate :: SyntaxKind :: MINUS_EQ } ; [*=] => { $ crate :: SyntaxKind :: STAR_EQ } ; [/=] => { $ crate :: SyntaxKind :: SLASH_EQ } ; [func] => { $ crate :: SyntaxKind :: FUNC_KW } ; [view] => { $ crate :: SyntaxKind :: VIEW_KW } ; [state] => { $ crate :: SyntaxKind :: STATE_KW } ; [let] => { $ crate :: SyntaxKind :: LET_KW } ; [mut] => { $ crate :: SyntaxKind :: MUT_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; [#] => { $ crate :: SyntaxKind :: HASH } ; }
