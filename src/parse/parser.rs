use std::mem;
use std::collections::HashMap;

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
        mem::swap(&mut self.last, &mut self.curr);
        mem::swap(&mut self.curr, &mut self.peek);
        self.peek = try!(self.lexer.next_real());
        Ok(())
    }

    // SQL Commands

    fn parse_commands(&mut self) -> Result<(), ParserError> {
        let curr = self.curr.clone();

        // Parse first word that
        match curr.unwrap().token {
            Token::Word(val) => try!(self.run_major_command(val)),
            _ => return Err(ParserError::FirstCmdNotWord),
        }

        Ok(())
    }

    fn run_major_command(&mut self, cmd: String) -> Result<(), ParserError> {
        match Keyword::from_str(&*cmd) {
            Keyword::Select => self.parse_select(),
            Keyword::Insert => self.parse_insert(),

            _ => return Err(ParserError::FirstCmdNotMajor),
        };

        Ok(())
    }

    fn parse_select(&mut self) -> Query {
        // TODO: impl parse_select

        Query::Table(TableStmt::Select(SelectStmt {
            cols: vec![Col { name: "id".to_owned() }],
            table: Table {
                name: "user_v1".to_owned(),
                alias: None,
            },
        }))
    }

    fn parse_insert(&mut self) -> Query {
        // TODO: impl parse_insert

        Query::Table(TableStmt::Insert(InsertStmt {
            table: Table {
                name: "user_v1".to_owned(),
                alias: None,
            },
            cols: HashMap::new(),
        }))
    }
}

#[derive(Debug)]
pub enum Keyword {
    // Major
    Select,
    Insert,

    // Minor
    From,
}

impl Keyword {
    pub fn from_str(k: &str) -> Keyword {
        match &*k.to_lowercase() {
            "select" => Keyword::Select,
            "insert" => Keyword::Insert,
            "from" => Keyword::From,
            keyword => panic!("unexpected keyword {}", keyword),
        }

    }
}

#[derive(Debug)]
pub enum ParserError {
    InvalidCommand,
    LexerError(LexError),
    FirstCmdNotWord,
    FirstCmdNotMajor,
}

impl From<LexError> for ParserError {
    fn from(e: LexError) -> ParserError {
        ParserError::LexerError(e)
    }
}
