use std::mem::swap;
use super::lexer::{Lexer, LexError};
use super::token::{TokenSpan, Token, Lit};
use super::exec::*; // TODO: remove * import

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    curr: Option<TokenSpan>,
    last: Option<TokenSpan>,
    peek: Option<TokenSpan>,
}

impl<'a> Parser<'a> {
    pub fn from_query(q: &'a str) -> Parser<'a> {
        let lex = Lexer::from_query(q);
        let mut p = Parser {
            lexer: lex,
            curr: None,
            last: None,
            peek: None,
        };

        // TODO: fix this
        p.bump().unwrap();
        p.bump().unwrap();
        p
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        try!(self.parse_commands());
        Ok(())
    }

    fn bump(&mut self) -> Result<(), ParserError> {
        // do stuff. Mainly swap last = curr, curr = peek, then peek = next_real
        swap(&mut self.last, &mut self.curr);
        swap(&mut self.curr, &mut self.peek);
        self.peek = try!(self.lexer.next_real());
        Ok(())
    }

    // SQL Commands

    fn parse_commands(&mut self) -> Result<(), ParserError> {
        let curr = self.curr.clone();

        match curr.unwrap().token {
            Token::Word(val) => self.run_major_command(val),
            _ => return Err(ParserError::FirstCmdNonMajor),
        }

        Ok(())
    }

    fn run_major_command(&mut self, cmd: String) {
        match Keyword::from_str(&*cmd) {
            Keyword::Select => self.parse_select(),
            _ => panic!("not select"),
        };
    }

    fn parse_select(&mut self) -> Query {
        Query::TableStmt(TableStmt::SelectStmt)
    }
}

#[derive(Debug)]
pub enum Keyword {
    // Major
    Select,

    // Minor
    From,
}

impl Keyword {
    pub fn from_str(k: &str) -> Keyword {
        match &*k.to_lowercase() {
            "select" => Keyword::Select,
            "from" => Keyword::From,
            keyword => panic!("unexpected keyword {}", keyword),
        }

    }
}

#[derive(Debug)]
pub enum ParserError {
    InvalidCommand,
    LexerError(LexError),
    FirstCmdNonMajor,
}

impl From<LexError> for ParserError {
    fn from(e: LexError) -> ParserError {
        ParserError::LexerError(e)
    }
}
