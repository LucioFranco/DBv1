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
        Parser {
            lexer: lex,
            curr: None,
            last: None,
            peek: None,
        }
    }

    pub fn parse(&mut self) -> Result<Query, ParserError> {
        // TODO: handle this with a better error message
        try!(self.bump());
        try!(self.bump());

        self.parse_commands()
    }

    fn bump(&mut self) -> Result<(), ParserError> {
        // do stuff. Mainly swap last = curr, curr = peek, then peek = next_real
        mem::swap(&mut self.last, &mut self.curr);
        mem::swap(&mut self.curr, &mut self.peek);
        self.peek = try!(self.lexer.next_real());
        Ok(())
    }

    // SQL Commands

    fn parse_commands(&mut self) -> Result<Query, ParserError> {
        let curr = self.curr.clone();

        // Parse first word that
        match curr.unwrap().token {
            Token::Word(val) => self.run_major_command(val),
            _ => Err(ParserError::FirstCmdNotWord),
        }
    }

    fn run_major_command(&mut self, cmd: String) -> Result<Query, ParserError> {
        match Keyword::from_str(&*cmd).unwrap() { // TODO: clean up unwrap
            Keyword::Select => self.parse_select(),
            Keyword::Insert => self.parse_insert(),

            _ => Err(ParserError::FirstCmdNotMajor),
        }
    }

    fn parse_select(&mut self) -> Result<Query, ParserError> {
        // TODO: impl parse_select

        Ok(Query::Table(TableStmt::Select(SelectStmt {
            cols: vec![Col { name: "id".to_owned() }],
            table: Table {
                name: "user_v1".to_owned(),
                alias: None,
            },
        })))
    }

    fn parse_insert(&mut self) -> Result<Query, ParserError> {
        // TODO: impl parse_insert
        try!(self.expect_keyword(Keyword::Into));

        //try!();

        try!(self.expect_token(Token::ParentOP));

        let mut cols = Vec::<Col>::new();
        cols.push(Col { name: try!(self.expect_word())});

        Ok(Query::Table(TableStmt::Insert(InsertStmt {
            table: Table {
                name: "user_v1".to_owned(),
                alias: None,
            },
            cols: HashMap::new(),
        })))
    }
}

// Helper function
impl<'a> Parser<'a> {
    fn expect_keyword(&mut self, exp: Keyword) -> Result<Keyword, ParserError> {
        // TODO: clean up unwrap but they should be safe for the moment

        try!(self.bump());
        let curr = {
            let token = &self.curr.clone().unwrap();

            match &token.token {
                &Token::Word(ref word) => word.clone(),
                t => return Err(ParserError::ExpectedKeyword(exp, format!("{:?}", t))),
            }
        };


        let actual = try!(Keyword::from_str(&curr));

        if actual == exp {
            Ok(actual)
        } else {
            Err(ParserError::ExpectedKeyword(exp, curr))
        }
    }

    fn expect_token(&mut self, exp: Token)  -> Result<Token, ParserError> {
        try!(self.bump());

        let token = self.curr.clone().unwrap();
        let actual = token.token.clone();

        if actual == exp {
            Ok(actual)
        } else {
            Err(ParserError::ExpectedToken(exp, format!("{:?}", actual)))
        }
    }

    // expect word case insensitive.
    fn expect_word(&mut self) -> Result<String, ParserError> {
        try!(self.bump());

        let token = match self.curr.clone() {
            Some(t) => t,
            None => return Err(ParserError::ExpectedTokenButGotNone),
        };
        let actual = token.token.clone();

        let word = match actual {
            Token::Word(ref word) => word.to_lowercase(), // always lowercase
            t => return Err(ParserError::ExpectedToken(Token::Word(String::new()), format!("{:?}", t))),
        };

        Ok(word)
    }

    fn expect_lit(&mut self) -> Result<Lit, ParserError> {
        try!(self.bump());

        let token = self.curr.clone().unwrap();
        let actual = token.token.clone();

        match actual {
            Token::Literal(lit) => Ok(lit),
            t => Err(ParserError::ExpectedToken(Token::Literal(Lit::String(String::new())), format!("{:?}", t))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    // Major
    Select,
    Insert,

    // Minor
    From,
    Into,
}

impl Keyword {
    pub fn from_str(k: &str) -> Result<Keyword, ParserError> {
        let keyword = match &*k.to_lowercase() {
            "select" => Keyword::Select,
            "insert" => Keyword::Insert,

            "from" => Keyword::From,
            "into" => Keyword::Into,

            // Keyword not found
            keyword => return Err(ParserError::UnexpectedKeyword(keyword.to_owned())), // TODO: clean up panic
        };

        Ok(keyword)
    }
}

#[derive(Debug)]
pub enum ParserError {
    InvalidCommand,
    LexerError(LexError),

    FirstCmdNotWord,
    FirstCmdNotMajor,

    ExpectedKeyword(Keyword, String), // exp, actual
    ExpectedToken(Token, String), // exp, actual
    ExpectedTokenButGotNone,

    UnexpectedKeyword(String),
}

impl From<LexError> for ParserError {
    fn from(e: LexError) -> ParserError {
        ParserError::LexerError(e)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn select() {
        let mut p = Parser::from_query("select");
        p.parse().unwrap();
    }

    #[test]
    fn insert() {
        let mut p = Parser::from_query("insert into (name)");
        let q = p.parse().unwrap();

    }

    // #[test]
    // fn first_non_major() {
    //     let err = Parser::from_query("alskdfj").parse();
    //     assert!(err.is_err());
    // }
}
