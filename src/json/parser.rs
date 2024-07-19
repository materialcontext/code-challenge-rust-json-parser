use super::lexer::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Peekable<IntoIter<Token>>) -> Self {
        Self { tokens }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<JsonValue, String> {
        match self.next_token() {
            Some(Token::CurlyOpen) => self.parse_object(),
            Some(Token::SquareOpen) => self.parse_array(),
            Some(Token::String(s)) => Ok(JsonValue::String(s)),
            Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
            Some(Token::Bool(b)) => Ok(JsonValue::Bool(b)),
            Some(Token::Null) => Ok(JsonValue::Null),
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        let mut map = std::collections::HashMap::new();
        loop {
            match self.next_token() {
                Some(Token::CurlyClose) => break,
                Some(Token::String(key)) => {
                    if let Some(Token::Colon) = self.next_token() {
                        let value = self.parse()?;
                        map.insert(key.clone(), value);
                        match self.next_token() {
                            Some(Token::Comma) => continue,
                            Some(Token::CurlyClose) => break,
                            _ => return Err("Expected comma or closing curly brace".to_string()),
                        }
                    } else {
                        return Err("Expected colon".to_string());
                    }
                }
                _ => return Err("Expected string key or closing curly brace".to_string()),
            }
        }
        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut vec = Vec::new();
        loop {
            match self.peek_token() {
                Some(Token::SquareClose) => {
                    self.next_token();
                    break;
                }
                _ => {
                    let value = self.parse()?;
                    vec.push(value);
                    match self.peek_token() {
                        Some(Token::Comma) => {
                            self.next_token();
                        } // consume comma
                        Some(Token::SquareClose) => continue,
                        _ => return Err("Expected comma or closing square bracket".to_string()),
                    }
                }
            }
        }
        Ok(JsonValue::Array(vec))
    }
}
