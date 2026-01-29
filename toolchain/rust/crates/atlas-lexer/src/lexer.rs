use crate::{LexError, Token, token::{Directive, Span, SpannedToken}};


pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    line: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<SpannedToken, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace and comments
        self.skip();

        // look for end of file
        if self.pos >= self.src.len() {
            return Some(Ok(SpannedToken {
                token: crate::Token::EoF,
                span: Span {
                    start: self.pos,
                    end: self.pos
                }
            }))
        }

        // start tokenizing
        let start = self.pos;

        let next = self.peek().unwrap();

        if next == '.' {
            // get directive text
            let word = self.get_word();

            Some(Ok(SpannedToken {
                token: Token::Directive(Directive::from_str(word).unwrap()),
                span: Span {
                    start,
                    end: self.pos
                }
            }))
        } else if next == 'r' {
            let reg = self.get_number().parse().unwrap();

            // test if valid register
            if reg > 15 {
                return Some(Err(LexError::InvalidNumber("?".into(), reg)));
            }

            Some(Ok(SpannedToken {
                token: Token::Register(reg as u8), 
                span: Span {
                    start,
                    end: self.pos
                }
            }))
        } else if next.is_alphabetic() {
            let word = self.get_word();

            if 
        } else {
            None
        }

        
        
    }
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src, pos: 0, line: 0
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    /// skip whitespace and comments
    fn skip(&mut self) {
        while let Some(c) = self.peek() {
            if Self::is_whitespace(c) {
                self.advance(c.len_utf8());
            } else if c == ';' {
                while let Some(c2) = self.peek() {
                    self.advance(c2.len_utf8());
                    if c2 == '\n' {
                        break;
                    }
                }
            } else {
                break; // reached a real token
            }
        }
    }


    /// advance until next whitespace
    fn get_word(&mut self) -> &'a str {
        
        let start = self.pos;

        while Self::is_whitespace(self.peek().unwrap()) {
            self.advance(self.peek().unwrap().len_utf8());
        }

        &self.src[start..self.pos]
    }

    fn get_number(&mut self) -> &'a str {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }

        &self.src[start..self.pos]
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t'
    }
}