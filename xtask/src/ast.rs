#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node { name: String, ty: String, cardinality: Cardinality },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) contextual_keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_BRACE"),
        ("}", "R_BRACE"),
        ("[", "L_BRACKET"),
        ("]", "R_BRACKET"),
        (";", "SEMICOLON"),
        (":", "COLON"),
        (",", "COMMA"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("%", "PERCENT"),
        (".", "DOT"),
        ("=", "EQ"),
        ("==", "EQ_EQ"),
        ("!=", "BANG_EQ"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("!", "BANG"),
        ("</", "L_ANGLE_SLASH"),
        ("/>", "SLASH_R_ANGLE"),
        ("+=", "PLUS_EQ"),
        ("-=", "MINUS_EQ"),
        ("*=", "STAR_EQ"),
        ("/=", "SLASH_EQ"),
    ],
    keywords: &["func", "view", "state", "let", "mut", "return", "true", "false"],
    contextual_keywords: &[],
    literals: &["INT", "STRING"],
    tokens: &["IDENT", "ERROR", "COMMENT", "WHITESPACE"],
    nodes: &[
        "SOURCE_FILE",
        "NAME",
        "NAME_REF",
        "PARAM_LIST",
        "PARAM",
        "FUNC_BODY",
        "VIEW_BODY",
        "RETURN_STMT",
        "LITERAL",
        "BINARY_EXPR",
        "UNARY_EXPR",
        "LET_STMT",
        "STATE_STMT",
        "XML_ELEMENT",
        "XML_ATTRIBUTE_LIST",
        "XML_ATTRIBUTE",
        "IDENT_PATTERN",
    ],
};
