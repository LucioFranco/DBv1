use std::str::Chars;
use super::Span;
use super::token::{Token, TokenSpan, Lit};

pub struct Lexer<'a> {
    chars: Chars<'a>,
    last: Option<char>,
    last_pos: Option<usize>,
    curr: Option<char>,
    curr_pos: Option<usize>,
    next: Option<char>,
    span_pos: Option<usize>,
}

impl<'a> Lexer<'a> {
    pub fn from_query(q: &str) -> Lexer {
        let mut lex = Lexer {
            chars: q.chars(),
            last: None,
            last_pos: None,
            curr: None,
            curr_pos: None,
            next: None,
            span_pos: None,
        };

        lex.bump();
        lex.bump();
        lex
    }

    pub fn next_real(&mut self) -> Result<Option<TokenSpan>, LexError> {
        let tokenspn = try!(self.next());

        let wspace = match tokenspn {
            Some(ref token) => {
                match token.token {
                    Token::Whitespace => true,
                    _ => false,
                }
            }
            _ => false,
        };

        if wspace {
            self.next()
        } else {
            Ok(tokenspn)
        }
    }

    fn bump(&mut self) {
        self.last = self.curr;
        self.curr = self.next;
        self.next = self.chars.next();

        self.last_pos = self.curr_pos;

        match self.next {
            Some(c) => {
                if let Some(n) = self.curr_pos {
                    self.curr_pos = Some(n + c.len_utf8());
                } else {
                    self.curr_pos = Some(c.len_utf8());
                }
            }
            _ => {}
        }
    }

    fn scan_words(&mut self) -> String {
        let mut s = String::new();

        loop {
            match self.curr.unwrap_or(' ') {
                c @ 'a'...'z' |
                c @ 'A'...'Z' |
                c @ '0'...'9' |
                c @ '_' => {
                    s.push(c);
                }
                _ => break,
            }

            self.bump();
        }

        s
    }

    fn scan_nums(&mut self) -> String {
        let mut s = String::new();

        loop {
            match self.curr.unwrap_or(' ') {
                c @ '0'...'9' |
                c @ '.' => {
                    s.push(c);
                }
                _ => break,
            }

            self.bump();
        }

        s
    }

    fn scan_lits(&mut self) -> Result<String, LexError> {
        let mut s = String::new();

        self.bump(); // skip first char of the literal

        loop {
            match self.curr {
                Some('\'') |
                Some('"') => break,

                Some(c) => s.push(c),

                None => return Err(LexError::UnclosedQuote),
            }

            self.bump();
        }
        self.bump();

        Ok(s)
    }

    fn skip_whitespaces(&mut self) {
        loop {
            if Self::is_whitespace(self.curr.unwrap_or('x')) {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn is_whitespace(c: char) -> bool {
        match c {
            ' ' | '\n' | '\t' => true,
            _ => false,
        }
    }

    fn next(&mut self) -> Result<Option<TokenSpan>, LexError> {
        let nextchar = self.next.unwrap_or('\x00').to_lowercase().next().unwrap();

        self.span_pos = self.curr_pos;

        let curr = match self.curr {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match curr {
            'a'...'z' | 'A'...'Z' => {
                let word = self.scan_words();
                Token::Word(word)
            }

            // TODO: check to see if '.' is not a valid Literal
            '0'...'9' => {
                let n = self.scan_nums();
                if let Ok(i) = n.parse::<i64>() {
                    Token::Literal(Lit::Int(i))
                } else {
                    if let Ok(f) = n.parse::<f64>() {
                        Token::Literal(Lit::Float(f))
                    } else {
                        Token::Unknown
                    }
                }
            }

            ';' => {
                self.bump();
                Token::Semi
            }
            '.' => {
                self.bump();
                Token::Dot
            }
            ',' => {
                self.bump();
                Token::Comma
            }

            '(' => {
                self.bump();
                Token::ParentOP
            }

            ')' => {
                self.bump();
                Token::ParentCL
            }

            // Literals
            '\'' | '"' => {
                let lit = try!(self.scan_lits());
                Token::Literal(Lit::String(lit))
            }

            '=' => {
                self.bump();
                Token::Equ
            }

            // Not Equal
            '!' if nextchar == '=' => {
                self.bump();
                self.bump();

                Token::NEqu
            }
            '<' if nextchar == '>' => {
                self.bump();
                self.bump();

                Token::NEqu
            }

            '>' => {
                self.bump();
                if nextchar == '=' {
                    self.bump();
                    Token::GEThan
                } else {
                    Token::GThan
                }
            }
            '<' => {
                self.bump();
                if nextchar == '=' {
                    self.bump();
                    Token::LEThan
                } else {
                    Token::LThan
                }
            }

            '+' => {
                self.bump();
                Token::Add
            }
            '-' => {
                self.bump();
                Token::Sub
            }
            '/' => {
                self.bump();
                Token::Div
            }
            '%' => {
                self.bump();
                Token::Mod
            }

            '*' => {
                self.bump();
                Token::Star
            }

            c if Self::is_whitespace(c) => {
                self.skip_whitespaces();
                Token::Whitespace
            }

            _ => {
                self.bump();
                Token::Unknown
            }

        };

        Ok(Some(TokenSpan {
            token: token,
            span: Span {
                // TODO: clean up the unwraps
                start: self.span_pos.unwrap(),
                end: self.curr_pos.unwrap(),
            },
        }))
    }
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    UnclosedQuote,
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use super::super::token::*;

    #[test]
    fn lexer_simple() {
        compare_lex("select", &[Token::Word("select".to_owned())]);
    }

    #[test]
    fn lexer_medium() {
        let cmds = &[Token::Word("select".to_owned()), Token::Word("from".to_owned())];

        compare_lex("select from", cmds);
    }

    #[test]
    fn lexer_hard() {
        let cmds = &[Token::Word("select".to_owned()),
                     Token::Star,
                     Token::Word("from".to_owned()),
                     Token::Semi];
        compare_lex("select * from;", cmds);
    }

    #[test]
    fn lexer_lit() {
        let cmds = &[Token::Word("insert".to_owned()), Token::Literal(Lit::Int(43)), Token::Semi];
        compare_lex("insert 43;", cmds);

        let cmds = &[Token::Word("insert".to_owned()),
                     Token::Literal(Lit::Float(43.32)),
                     Token::Semi];
        compare_lex("insert 43.32;", cmds);

        let cmds = &[Token::Word("insert".to_owned()),
                     Token::Literal(Lit::String("true".to_owned())),
                     Token::Semi];
        compare_lex("insert \"true\";", cmds);
    }

    #[test]
    fn lexer_parent() {
        let cmds = &[Token::Word("insert".to_owned()),
                     Token::Word("into".to_owned()),
                     Token::ParentOP,
                     Token::Word("bool_string".to_owned()),
                     Token::ParentCL,
                     Token::Word("VALUES".to_owned()),
                     Token::ParentOP,
                     Token::Literal(Lit::String("true".to_owned())),
                     Token::ParentCL,
                     Token::Semi];
        compare_lex("insert into (bool_string) VALUES (\"true\");", cmds);
    }

    fn compare_lex(q: &str, cmds: &[Token]) {
        let mut lex = Lexer::from_query(q);
        let mut index = 0;

        loop {
            let val = match lex.next_real().unwrap() {
                Some(val) => val,
                None => break,
            };
            println!("{:?}", val);

            // println!("{:?}, {:?}", val.token, cmds[index]);
            assert_eq!(val.token, cmds[index]);
            index += 1;
        }
    }
}
