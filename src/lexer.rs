use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;

/// Represents a syntax token
#[derive(Debug)]
pub enum Token {
    RParen,
    LParen,
    RCurly,
    LCurly,
    Colon,
    Semicolon,
    Comma,
    Equals,
    Exclamation,
    Function,
    Return,
    Operator(char),
    Ident(String),
    Float(f64),
    Int(i64),
}

/// Holds a syntax token along with position metadata
#[derive(Debug)]
pub struct LexedToken {
    line: usize,
    pos: usize,
    tok: Token,
}

/// Provides an error message with location
#[derive(Debug)]
pub struct LexError {
    line: usize,
    pos: usize,
    err: String,
}

pub type LexResult = Result<Option<LexedToken>, LexError>;

struct LexNumericErr(String);

impl From<std::num::ParseIntError> for LexNumericErr {
    fn from(e: std::num::ParseIntError) -> Self {
        LexNumericErr(format!("{:?}", e))
    }
}

impl From<std::num::ParseFloatError> for LexNumericErr {
    fn from(e: std::num::ParseFloatError) -> Self {
        LexNumericErr(format!("{:?}", e))
    }
}

/// Defines a lexer which transforms an input string into a Token stream.
pub struct Lexer<'inpt> {
    input: &'inpt str,
    chars: Box<Peekable<Chars<'inpt>>>,
    line: usize,
    pos: usize,
}

impl<'inpt> Lexer<'inpt> {
    /// Creates a new `Lexer`, given its source input.
    pub fn new(input: &'inpt str) -> Self {
        Self {
            input,
            chars: Box::new(input.chars().peekable()),
            pos: 0,
            line: 0,
        }
    }

    /// Lexes and returns the next `Token` from the source code
    pub fn lex(&mut self) -> LexResult {
        let chars = self.chars.deref_mut();

        // Skip whitespaces
        loop {
            // Note: the following lines are in their own scope to
            // limit how long `chars` is borrowed, and in order to
            // allow it to be borrowed again in the loop by `chars.next()`.
            {
                let ch = match chars.peek() {
                    Some(c) => *c,
                    None => return Ok(None), // We've reached EOF
                };

                if !ch.is_whitespace() && !is_newline(ch) {
                    break;
                }
            }

            let c = chars.next().unwrap();
            if is_newline(c) {
                self.line += 1;
            }

            self.pos += 1;
        }

        // Keeping track of where we start lets us take a string slice to pull out
        // identifiers and literals, as well as returning it as the start of the lex
        // attempt in case of an error.
        let start = self.pos;
        let next = chars.next().unwrap(); // This can't be EOF since it would've been caught in the above loop

        self.pos += 1;

        let token = match next {
            '{' => Token::RCurly,
            '}' => Token::LCurly,
            '(' => Token::RParen,
            ')' => Token::LParen,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '=' => Token::Equals,
            '!' => Token::Exclamation,

            '0'..='9' => match self.lex_numeric(Some(start)) {
                Ok(tok) => tok,
                Err(e) => {
                    return Err(LexError {
                        line: self.line,
                        pos: self.pos,
                        err: e.0,
                    })
                }
            },

            '-' => match self.lex_numeric(None) {
                // If this is Ok, negate the number
                Ok(Token::Float(f)) => Token::Float(-f),
                Ok(Token::Int(i)) => Token::Int(-i),

                // If this is an error, there was no number (so treat it as an operator)
                _ => Token::Operator('-'),
            },
            '+' => Token::Operator('+'),
            '*' => Token::Operator('*'),
            '/' => {
                if let Some('/') = chars.peek() {
                    // This is a comment, keep going until we get to a newline
                    while !is_newline(*chars.peek().unwrap_or(&'\n')) {
                        self.pos += 1;
                        chars.next();
                    }

                    // Skip over this comment and return the next token
                    return self.lex();
                } else {
                    // This is just the `/` operator
                    Token::Operator('/')
                }
            }

            _ => {
                loop {
                    match chars.peek() {
                        Some(c) if c.is_whitespace() || is_newline(*c) => break,

                        _ => {
                            chars.next();
                            self.pos += 1;
                        }
                    }
                }

                match &self.input[start..self.pos] {
                    "function" => Token::Function,
                    "return" => Token::Return,
                    tok => {
                        if tok.contains("l") {
                            return Err(LexError {
                                line: self.line,
                                pos: self.pos,
                                err: String::from("'l' faiLs the ELi Linter"),
                            });
                        }

                        Token::Ident(tok.to_string())
                    }
                }
            }
        };

        Ok(Some(LexedToken {
            line: self.line,
            pos: self.pos,
            tok: token,
        }))
    }

    fn lex_numeric(&mut self, start: Option<usize>) -> Result<Token, LexNumericErr> {
        let start = start.unwrap_or(self.pos);
        loop {
            let c = *self.chars.peek().unwrap_or(&' ');
            if !c.is_numeric() && c != '.' {
                break;
            }

            self.chars.next();
            self.pos += 1;
        }

        let num_str = self.input[start..self.pos].to_string();

        if num_str.contains(".") {
            Ok(Token::Float(num_str.parse()?))
        } else {
            Ok(Token::Int(num_str.parse()?))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexedToken;

    /// Lexes the next `Token` and returns it.
    /// On EOF or failure, `None` will be returned.
    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(token) => token,
            Err(e) => {
                eprintln!("Error lexing input: {:#?}", e);
                None
            }
        }
    }
}

fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}
