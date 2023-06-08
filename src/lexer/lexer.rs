use std::fmt::Display;

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(String),
    LPar,
    RPar,
    Bool,
    Asterisk,
    Plus,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Int(x) => write!(f, "Int({})", x),
            Token::LPar => write!(f, "LPar"),
            Token::RPar => write!(f, "RPar"),
            Token::Bool => write!(f, "Bool"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Plus => write!(f, "Plus"),
            Token::Eof => write!(f, "Eof"),
        };
    }
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();
        return lex;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'(' => Token::LPar,
            b')' => Token::RPar,
            b'*' => Token::Asterisk,
            b'+' => Token::Plus,
            b'0'..=b'9' => return Ok(Token::Int(self.read_int())),
            0 => Token::Eof,
            _ => unreachable!("Unallow characters"),
        };

        self.read_char();
        return Ok(tok);
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char()
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "(10 * 0) + 5 * 4";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::LPar,
            Token::Int(String::from("10")),
            Token::Asterisk,
            Token::Int(String::from("0")),
            Token::RPar,
            Token::Plus,
            Token::Int(String::from("5")),
            Token::Asterisk,
            Token::Int(String::from("4")),
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }
}
