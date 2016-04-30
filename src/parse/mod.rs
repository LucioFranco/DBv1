pub mod lexer;
pub mod parser;
pub mod token;
pub mod ast;

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
