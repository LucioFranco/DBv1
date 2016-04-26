#![allow(dead_code)]

use super::Span;

#[derive(Debug, Clone)]
pub struct TokenSpan {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    String(String),
    Int(i64),
    Float(f64),
    Bool(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),

    Literal(Lit),

    Semi,
    Dot,
    Comma,

    // '(' and ')'
    ParentOP,
    ParentCL,

    Equ,
    GThan,
    LThan,
    GEThan,
    LEThan,
    NEqu,
    Add,
    Sub,
    Div,
    Mod,

    Star, // Multi use. Could be wildcard or multi. TODO: add better star token

    Whitespace,

    Unknown,
}
