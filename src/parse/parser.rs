use std::mem;
use std::collections::HashMap;

use super::lexer::{Lexer, LexError};
use super::token::{TokenSpan, Token, Lit};
use super::ast::*; // TODO: remove * import

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
        let mut cols = Vec::new();

        loop {
            match self.expect_word() {
                Ok(ref word) => {
                    cols.push(Col { name: word.to_owned() });
                    match try!(self.peek_clone()) {
                        Token::Comma => try!(self.bump()),

                        Token::Word(_) => break,

                        token => return Err(ParserError::ExpectedToken(Token::Comma, format!("{:?}", token))),
                    }
                },
                Err(err) => return Err(err),
            }
        }

        try!(self.expect_keyword(Keyword::From));

        let table = {
            let name = try!(self.expect_word());
            Table {
                name: name,
                alias: None,
            }
        };

        try!(self.expect_token(Token::Semi));

        Ok(Query::Table(TableStmt::Select(SelectStmt {
            cols: cols,
            table: table,
        })))
    }

    fn parse_insert(&mut self) -> Result<Query, ParserError> {
        // TODO: impl parse_insert
        try!(self.expect_keyword(Keyword::Into));

        let table = {
            let name = try!(self.expect_word());
            Table {
                name: name,
                alias: None,
            }
        };

        let cols = try!(self.expect_comma_dil_word());

        try!(self.expect_keyword(Keyword::Values));


        let mut values = try!(self.expect_comma_dil_lit());

        if cols.capacity() != values.capacity() {
            return Err(ParserError::ColumsDoNotMatchValues);
        }

        let mut cols_map = HashMap::new();
        for col in cols {
            let col = Col { name: col };
            cols_map.insert(col, values.remove(0));
        }

        try!(self.expect_token(Token::Semi));
        Ok(Query::Table(TableStmt::Insert(InsertStmt {
            table: table,
            cols: cols_map,
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

    fn expect_comma_dil_lit(&mut self) -> Result<Vec<Lit>, ParserError> {
        try!(self.expect_token(Token::ParentOP));

        let mut cols = Vec::new();

        loop {
            match self.expect_lit() {
                Ok(ref lit) => {
                    cols.push(lit.clone());
                    match try!(self.peek_clone()) {
                        Token::Comma => try!(self.bump()),
                        Token::ParentCL => {
                            try!(self.bump());
                            break;
                        },
                        token => return Err(ParserError::ExpectedToken(Token::Comma, format!("{:?}", token))),
                    }
                },
                Err(err) => return Err(err),
            }
        }

        Ok(cols)
    }

    fn expect_comma_dil_word(&mut self) -> Result<Vec<String>, ParserError> {
        try!(self.expect_token(Token::ParentOP));

        let mut cols = Vec::new();

        loop {
            match self.expect_word() {
                Ok(ref word) => {
                    cols.push(word.to_owned());
                    match try!(self.peek_clone()) {
                        Token::Comma => try!(self.bump()),
                        Token::ParentCL => {
                            try!(self.bump());
                            break;
                        },
                        token => return Err(ParserError::ExpectedToken(Token::Comma, format!("{:?}", token))),
                    }
                },
                Err(err) => return Err(err),
            }
        }

        Ok(cols)
    }

    fn peek_clone(&mut self) -> Result<Token, ParserError> {
        let peek = try!(Self::unwrap_tokenspan(self.peek.clone()));

        Ok(peek.token)
    }

    fn unwrap_tokenspan(t: Option<TokenSpan>) -> Result<TokenSpan, ParserError> {
        match t {
            Some(val) =>  Ok(val),
            None => Err(ParserError::ExpectedTokenButGotNone),
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
    Values,
}

impl Keyword {
    pub fn from_str(k: &str) -> Result<Keyword, ParserError> {
        let keyword = match &*k.to_lowercase() {
            "select" => Keyword::Select,
            "insert" => Keyword::Insert,

            "from" => Keyword::From,
            "into" => Keyword::Into,
            "values" => Keyword::Values,

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

    ColumsDoNotMatchValues,

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
    use super::super::token::*;
    use super::super::ast::*;
    use std::collections::HashMap;

    #[test]
    fn select() {
        let mut p = Parser::from_query("select name, email from users;");
        let q = p.parse().unwrap();

        let q_exp = Query::Table(TableStmt::Select(SelectStmt {
            table: Table {
                name: "users".into(),
                alias: None
            },
            cols: vec![Col { name: "name".into() }, Col { name: "email".into() }]
        }));

        assert_eq!(q_exp, q);
    }

    #[test]
    #[should_panic]
    fn select_panic() {
        let mut p = Parser::from_query("select name, email users;");
        p.parse().unwrap();
    }

    #[test]
    fn insert() {
        let mut p = Parser::from_query("INSERT INTO users (name, email) VALUES (\"first last\", \"first.last@example.com\");");
        let q = p.parse().unwrap();

        let mut cols = HashMap::new();
        cols.insert(Col { name: "name".to_owned() }, Lit::String("first last".to_owned()));
        cols.insert(Col { name: "email".to_owned() }, Lit::String("first.last@example.com".to_owned()));

        let q_exp = Query::Table(TableStmt::Insert(InsertStmt {
            table: Table {
                name: "users".to_owned(),
                alias: None,
            },
            cols: cols,
        }));

        assert_eq!(q_exp, q);
    }

    #[test]
    #[should_panic]
    fn insert_no_table_name() {
        Parser::from_query("insert into (asdf aslkdfhjahh dsfkjals)").parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_non_proper_col_list() {
        Parser::from_query("insert into users (asdf aslkdfhjahh dsfkjals)").parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn first_non_major() {
        let err = Parser::from_query("alskdfj").parse();
        assert!(err.is_err());
    }
}
