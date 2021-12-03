//! Generated file, do not edit by hand

use crate::*;
use syntax::{SyntaxKind::*, T};
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt {
    pub(crate) syntax: SyntaxNode,
}
impl LetStmt {
    pub fn let_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![let])
    }
    pub fn mut_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![mut])
    }
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentPattern {
    pub(crate) syntax: SyntaxNode,
}
impl IdentPattern {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl NameRef {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}
impl ParamList {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn params(&self) -> AstChildren<Pattern> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl BinaryExpr {}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl UnaryExpr {}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Func {
    pub(crate) syntax: SyntaxNode,
}
impl Func {
    pub fn func_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![func])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn params(&self) -> Option<ParamList> {
        support::child(&self.syntax)
    }
    pub fn body(&self) -> Option<FuncBody> {
        support::child(&self.syntax)
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncBody {
    pub(crate) syntax: SyntaxNode,
}
impl FuncBody {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn body(&self) -> AstChildren<FuncStmt> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct View {
    pub(crate) syntax: SyntaxNode,
}
impl View {
    pub fn view_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![view])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn params(&self) -> AstChildren<ParamList> {
        support::children(&self.syntax)
    }
    pub fn body(&self) -> Option<ViewBody> {
        support::child(&self.syntax)
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ViewBody {
    pub(crate) syntax: SyntaxNode,
}
impl ViewBody {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn body(&self) -> AstChildren<ViewStmt> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateStmt {
    pub(crate) syntax: SyntaxNode,
}
impl StateStmt {
    pub fn state_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![state])
    }
    pub fn pattern(&self) -> Option<Pattern> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [=])
    }
    pub fn value(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ViewReturnStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ViewReturnStmt {
    pub fn return_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![return])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn value(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmlElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlElement {
    pub fn opening_element(&self) -> Option<XmlOpeningElementKind> {
        support::child(&self.syntax)
    }
    pub fn children(&self) -> AstChildren<XmlElement> {
        support::children(&self.syntax)
    }
    pub fn closing_element(&self) -> Option<XmlSelfClosingElement> {
        support::child(&self.syntax)
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmlOpeningElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlOpeningElement {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [<])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn attributes(&self) -> AstChildren<XmlAttribute> {
        support::children(&self.syntax)
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [>])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmlSelfClosingElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlSelfClosingElement {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [<])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn attributes(&self) -> AstChildren<XmlAttribute> {
        support::children(&self.syntax)
    }
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [/])
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [>])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmlClosingElement {
    pub(crate) syntax: SyntaxNode,
}
impl XmlClosingElement {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [<])
    }
    pub fn slash_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [/])
    }
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [>])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XmlAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl XmlAttribute {
    pub fn name(&self) -> Option<Name> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T ! [=])
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn value(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    IdentPattern(IdentPattern),
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Literal(Literal),
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FuncStmt {
    LetStmt(LetStmt),
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XmlOpeningElementKind {
    XmlOpeningElement(XmlOpeningElement),
    XmlSelfClosingElement(XmlSelfClosingElement),
}
#[doc = ""]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ViewStmt {
    LetStmt(LetStmt),
    StateStmt(StateStmt),
    ViewReturnStmt(ViewReturnStmt),
}
impl AstNode for LetStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LET_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for IdentPattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == IDENT_PATTERN
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME_REF
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParamList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PARAM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for BinaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == BINARY_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UnaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == UNARY_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == LITERAL
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Func {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNC
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FuncBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FUNC_BODY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for View {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VIEW
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ViewBody {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VIEW_BODY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for StateStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == STATE_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ViewReturnStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VIEW_RETURN_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for XmlElement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for XmlOpeningElement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_OPENING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for XmlSelfClosingElement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_SELF_CLOSING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for XmlClosingElement {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_CLOSING_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for XmlAttribute {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == XML_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl From<IdentPattern> for Pattern {
    fn from(node: IdentPattern) -> Pattern {
        Pattern::IdentPattern(node)
    }
}
impl AstNode for Pattern {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, IDENT_PATTERN)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            IDENT_PATTERN => Pattern::IdentPattern(IdentPattern { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Pattern::IdentPattern(it) => &it.syntax,
        }
    }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr {
        Expr::Literal(node)
    }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, LITERAL)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LITERAL => Expr::Literal(Literal { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::Literal(it) => &it.syntax,
        }
    }
}
impl From<LetStmt> for FuncStmt {
    fn from(node: LetStmt) -> FuncStmt {
        FuncStmt::LetStmt(node)
    }
}
impl AstNode for FuncStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, LET_STMT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LET_STMT => FuncStmt::LetStmt(LetStmt { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FuncStmt::LetStmt(it) => &it.syntax,
        }
    }
}
impl From<XmlOpeningElement> for XmlOpeningElementKind {
    fn from(node: XmlOpeningElement) -> XmlOpeningElementKind {
        XmlOpeningElementKind::XmlOpeningElement(node)
    }
}
impl From<XmlSelfClosingElement> for XmlOpeningElementKind {
    fn from(node: XmlSelfClosingElement) -> XmlOpeningElementKind {
        XmlOpeningElementKind::XmlSelfClosingElement(node)
    }
}
impl AstNode for XmlOpeningElementKind {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, XML_OPENING_ELEMENT | XML_SELF_CLOSING_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            XML_OPENING_ELEMENT => XmlOpeningElementKind::XmlOpeningElement(XmlOpeningElement { syntax }),
            XML_SELF_CLOSING_ELEMENT => XmlOpeningElementKind::XmlSelfClosingElement(XmlSelfClosingElement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            XmlOpeningElementKind::XmlOpeningElement(it) => &it.syntax,
            XmlOpeningElementKind::XmlSelfClosingElement(it) => &it.syntax,
        }
    }
}
impl From<LetStmt> for ViewStmt {
    fn from(node: LetStmt) -> ViewStmt {
        ViewStmt::LetStmt(node)
    }
}
impl From<StateStmt> for ViewStmt {
    fn from(node: StateStmt) -> ViewStmt {
        ViewStmt::StateStmt(node)
    }
}
impl From<ViewReturnStmt> for ViewStmt {
    fn from(node: ViewReturnStmt) -> ViewStmt {
        ViewStmt::ViewReturnStmt(node)
    }
}
impl AstNode for ViewStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, LET_STMT | STATE_STMT | VIEW_RETURN_STMT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LET_STMT => ViewStmt::LetStmt(LetStmt { syntax }),
            STATE_STMT => ViewStmt::StateStmt(StateStmt { syntax }),
            VIEW_RETURN_STMT => ViewStmt::ViewReturnStmt(ViewReturnStmt { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ViewStmt::LetStmt(it) => &it.syntax,
            ViewStmt::StateStmt(it) => &it.syntax,
            ViewStmt::ViewReturnStmt(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FuncStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlOpeningElementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ViewStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IdentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NameRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FuncBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ViewBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for StateStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ViewReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlOpeningElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlSelfClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlClosingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for XmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
