pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    #[allow(dead_code)]
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
        "FUNC",
        "FUNC_BODY",
        "VIEW",
        "VIEW_BODY",
        "RETURN_STMT",
        "VIEW_RETURN_STMT",
        "LITERAL",
        "BINARY_EXPR",
        "UNARY_EXPR",
        "LET_STMT",
        "STATE_STMT",
        "XML_ELEMENT",
        "XML_OPENING_ELEMENT",
        "XML_SELF_CLOSING_ELEMENT",
        "XML_CLOSING_ELEMENT",
        "XML_ATTRIBUTE_LIST",
        "XML_ATTRIBUTE",
        "IDENT_PATTERN",
    ],
};

pub(crate) struct AstSrc<'a> {
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [AstNodeSrc<'a>],
    pub(crate) enums: &'a [AstEnumSrc<'a>],
}

pub(crate) struct AstNodeSrc<'a> {
    pub(crate) name: &'a str,
    pub(crate) fields: &'a [Field<'a>],
    pub(crate) doc: &'a str,
}

pub(crate) enum Field<'a> {
    Token(&'a str),
    Node { name: &'a str, src: FieldSrc<'a> },
}

pub(crate) enum FieldSrc<'a> {
    #[allow(dead_code)]
    Shorthand,
    Optional(&'a str),
    Many(&'a str),
}

pub(crate) struct AstEnumSrc<'a> {
    pub(crate) name: &'a str,
    pub(crate) variants: &'a [&'a str],
    pub(crate) doc: &'a str,
}

macro_rules! ast_nodes {
    ($(
        $(#[doc = $doc:literal])*
        struct $name:ident {
            $($field_name:ident $(![$($token:tt)*])? $(: $ty:tt)?),*$(,)?
        }
    )*) => {
        [$(
            AstNodeSrc {
                name: stringify!($name),
                fields: &[
                    $(field!($(T![$($token)*])? $field_name $($ty)?)),*
                ],
                doc: concat!($($doc, "\n"),*)
            }
        ),*]
    };
}

macro_rules! field {
    (T![$($token:tt)*] T) => {
        Field::Token(stringify!($($token)*))
    };
    ($field_name:ident) => {
        Field::Node { name: stringify!($field_name), src: FieldSrc::Shorthand }
    };
    ($field_name:ident [$ty:ident]) => {
        Field::Node { name: stringify!($field_name), src: FieldSrc::Many(stringify!($ty)) }
    };
    ($field_name:ident $ty:ident) => {
        Field::Node { name: stringify!($field_name), src: FieldSrc::Optional(stringify!($ty)) }
    };
}

macro_rules! ast_enums {
    ($(
        $(#[doc = $doc:literal])*
        enum $name:ident {
            $($variant:ident),*$(,)?
        }
    )*) => {
        [$(
            AstEnumSrc {
                name: stringify!($name),
                variants: &[$(stringify!($variant)),*],
                doc: concat!($($doc, "\n"),*)
            }
        ),*]
    };
}

pub(crate) const AST_SRC: AstSrc<'static> = AstSrc {
    tokens: &["Whitespace", "Comment", "String"],

    nodes: &ast_nodes! {
        struct LetStmt {
            T![let],
            T![mut],
            pattern: Pattern
        }

        struct IdentPattern {
            name: Name
        }

        struct Name {
            T![ident]
        }

        struct NameRef {
            T![ident]
        }

        struct ParamList {
            T!['('],
            params: [Pattern],
            T![')']
        }

        struct BinaryExpr {
            // Manually created
        }

        struct UnaryExpr {
            // Manually created
        }

        struct Literal {
            // Manually created
        }

        struct Func {
            T![func],
            name: Name,
            params: ParamList,
            body: FuncBody
        }

        struct FuncBody {
            T!['{'],
            body: [FuncStmt],
            T!['}']
        }

        struct View {
            T![view],
            name: Name,
            params: [ParamList],
            body: ViewBody
        }

        struct ViewBody {
            T!['{'],
            body: [ViewStmt],
            T!['}']
        }

        struct StateStmt {
            T![state],
            pattern: Pattern,
            T![=],
            value: Expr
        }

        struct ViewReturnStmt {
            T![return],
            T!['('],
            value: Expr,
            T![')']
        }

        struct XmlElement {
            opening_element: XmlOpeningElementKind,
            children: [XmlElement],
            closing_element: XmlSelfClosingElement
        }

        struct XmlOpeningElement {
            T![<],
            name: Name,
            attributes: [XmlAttribute],
            T![>]
        }

        struct XmlSelfClosingElement {
            T![<],
            name: Name,
            attributes: [XmlAttribute],
            T![/],
            T![>]
        }

        struct XmlClosingElement {
            T![<],
            T![/],
            name: Name,
            T![>]
        }

        struct XmlAttribute {
            name: Name,
            T![=],
            T!['{'],
            value: Expr,
            T!['}']
        }
    },

    enums: &ast_enums! {
        enum Pattern {
            IdentPattern
        }

        enum Expr {
            Literal
        }

        enum FuncStmt {
            LetStmt
        }

        enum XmlOpeningElementKind {
            XmlOpeningElement,
            XmlSelfClosingElement
        }

        enum ViewStmt {
            LetStmt,
            StateStmt,
            ViewReturnStmt,
        }
    },
};
