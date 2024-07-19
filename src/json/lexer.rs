use std::iter::Peekable;

/// An enum representing all the possible Token types in a JSON object
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
    Comma,
    Colon,
    WhiteSpace,
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub enum TokenizerError {
    InvalidString,
    InvalidNumber,
    InvalidLiteral,
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidString => write!(f, "Invalid string token"),
            Self::InvalidNumber => write!(f, "Invalid number token"),
            Self::InvalidLiteral => write!(f, "Invalid literal token"),
        }
    }
}

impl std::error::Error for TokenizerError {}

/// A Struct for handling tokenization of JSON objects
#[derive(Debug, PartialEq, Clone)]
pub struct Tokenizer {
    pub input: Vec<u8>,
}

impl Tokenizer {
    pub fn new(input_string: &str) -> Self {
        Self {
            input: input_string.as_bytes().to_vec(),
        }
    }

    fn get_token(&self, idx: usize) -> Result<Token, TokenizerError> {
        match self.input[idx] {
            b'{' => Ok(Token::CurlyOpen),
            b'}' => Ok(Token::CurlyClose),
            b',' => Ok(Token::Comma),
            b'[' => Ok(Token::SquareOpen),
            b']' => Ok(Token::SquareClose),
            b':' => Ok(Token::Colon),
            b'n' => self.verify_null_token(idx),
            b'f' => self.verify_false_token(idx),
            b't' => self.verify_true_token(idx),
            b'"' => self.validate_string_token(idx),
            b'0'..=b'9' | b'-' => self.validate_number_token(idx),
            _ => Ok(Token::WhiteSpace),
        }
    }

    fn validate_string_token(&self, idx: usize) -> Result<Token, TokenizerError> {
        let end = self.input[idx + 1..]
            .windows(2)
            .position(|window| window == [b'"'] || (window[0] != b'\\' && window[1] == b'"'))
            .map(|pos| pos + 1);
        match end {
            Some(end) => {
                let output = String::from_utf8(self.input[idx + 1..idx + 1 + end].to_vec());
                match output {
                    Ok(output) => Ok(Token::String(output)),
                    Err(_) => Err(TokenizerError::InvalidString),
                }
            }
            None => Err(TokenizerError::InvalidString),
        }
    }

    fn validate_number_token(&self, idx: usize) -> Result<Token, TokenizerError> {
        let end = self.input[idx..]
            .iter()
            .position(|&val| !(val.is_ascii_digit() || val == b'.' || val == b'-'))
            .unwrap_or(self.input.len() - idx);
        let num = String::from_utf8(self.input[idx..idx + end].to_vec())
            .unwrap()
            .parse::<f64>();
        match num {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(TokenizerError::InvalidNumber),
        }
    }

    fn verify_false_token(&self, idx: usize) -> Result<Token, TokenizerError> {
        let false_slice = std::str::from_utf8(&self.input[idx..idx + 5])
            .unwrap_or("Error validating null token. Invalid literal.");
        match false_slice {
            "false" => Ok(Token::Bool(false)),
            _ => Err(TokenizerError::InvalidLiteral),
        }
    }

    fn verify_true_token(&self, idx: usize) -> Result<Token, TokenizerError> {
        let true_slice = std::str::from_utf8(&self.input[idx..idx + 4])
            .unwrap_or("Error validating null token. Invalid literal.");
        match true_slice {
            "true" => Ok(Token::Bool(true)),
            _ => Err(TokenizerError::InvalidLiteral),
        }
    }

    fn verify_null_token(&self, idx: usize) -> Result<Token, TokenizerError> {
        let null_slice = std::str::from_utf8(&self.input[idx..idx + 4])
            .unwrap_or("Error validating null token. Invalid literal.");
        match null_slice {
            "null" => Ok(Token::Null),
            _ => Err(TokenizerError::InvalidLiteral),
        }
    }

    pub fn tokenize(
        self,
    ) -> Result<Peekable<std::vec::IntoIter<Token>>, TokenizerError> {
        let mut output = Vec::new();
        let mut idx = 0;
        while idx < self.input.len() {
            let val = self.get_token(idx)?;
            if val != Token::WhiteSpace {
                output.push(val);
                match output.last().unwrap() {
                    Token::Bool(true) | Token::Null => idx += 4,
                    Token::Bool(false) => idx += 5,
                    Token::Number(val) => idx += val.to_string().len(),
                    Token::String(val) => idx += val.len() + 2,
                    _ => idx += 1,
                }
            } else {
                idx += 1
            }
        }
        Ok(output.into_iter().peekable())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_syntax() {
        let lexer = Tokenizer::new("[]{}:,");
        let expected = vec![
            Token::SquareOpen,
            Token::SquareClose,
            Token::CurlyOpen,
            Token::CurlyClose,
            Token::Colon,
            Token::Comma,
        ];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn integer() {
        let lexer = Tokenizer::new("245");
        let expected = vec![Token::Number(245.0)];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn float() {
        let lexer = Tokenizer::new("245.23");
        let expected = vec![Token::Number(245.23)];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn negative() {
        let lexer = Tokenizer::new("-245");
        let expected = vec![Token::Number(-245.0)];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn negative_float() {
        let lexer = Tokenizer::new("-245.23");
        let expected = vec![Token::Number(-245.23)];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn string() {
        let lexer = Tokenizer::new("\"Abc-243.abc00\"");
        let expected = vec![Token::String("Abc-243.abc00".to_string())];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn string_with_escaped_quote() {
        let lexer = Tokenizer::new("\"Abc-243.\\\"abc00\"");
        let expected = vec![Token::String("Abc-243.\\\"abc00".to_string())];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn null() {
        let lexer = Tokenizer::new("null");
        let expected = vec![Token::Null];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn bool_false() {
        let lexer = Tokenizer::new("false");
        let expected = vec![Token::Bool(false)];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn bool_true() {
        let lexer = Tokenizer::new("true");
        let expected = vec![Token::Bool(true)];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn valid_json() {
        let sample_json = "{
            \"str\": \"value\",
            \"num\": 123,
            \"bool\": true,
            \"null\": null
            }";
        let lexer = Tokenizer::new(sample_json);
        let expected = vec![
            Token::CurlyOpen,
            Token::String("str".to_string()),
            Token::Colon,
            Token::String("value".to_string()),
            Token::Comma,
            Token::String("num".to_string()),
            Token::Colon,
            Token::Number(123.0),
            Token::Comma,
            Token::String("bool".to_string()),
            Token::Colon,
            Token::Bool(true),
            Token::Comma,
            Token::String("null".to_string()),
            Token::Colon,
            Token::Null,
            Token::CurlyClose,
        ];
        let actual: Vec<Token> = lexer.tokenize().unwrap().collect();
        assert_eq!(expected, actual);
    }
}
