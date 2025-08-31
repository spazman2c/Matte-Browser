//! CSS Tokenizer implementation.
//! 
//! This module provides CSS tokenization functionality for parsing CSS
//! input into a stream of tokens according to the CSS specification.

use tracing::debug;
use crate::error::{Error, Result};

/// CSS token types
#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    /// Identifier (e.g., "div", "class-name")
    Ident(String),
    /// Number (e.g., "123", "45.6")
    Number(f64),
    /// String (e.g., "hello world")
    String(String),
    /// Hash (e.g., "#main")
    Hash(String),
    /// Delimiter (e.g., ".", "#", "[", "]", "(", ")", ":", ";", ",")
    Delim(char),
    /// Whitespace
    Whitespace,
    /// At-keyword (e.g., "@media")
    AtKeyword(String),
    /// Function (e.g., "url(")
    Function(String),
    /// URL
    Url(String),
    /// Bad URL
    BadUrl,
    /// Bad string
    BadString,
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
    /// Left square bracket
    LeftBracket,
    /// Right square bracket
    RightBracket,
    /// Left curly brace
    LeftBrace,
    /// Right curly brace
    RightBrace,
    /// Semicolon
    Semicolon,
    /// Comma
    Comma,
    /// Colon
    Colon,
    /// Percentage
    Percentage(f64),
    /// Dimension (e.g., "12px", "2em")
    Dimension(f64, String),
    /// Unicode range
    UnicodeRange(u32, u32),
    /// Comment
    Comment(String),
    /// End of file
    Eof,
}

/// CSS Tokenizer for parsing CSS input
pub struct CssTokenizer {
    /// Input string to tokenize
    input: String,
    /// Current position in the input
    position: usize,
    /// Current line number
    line: usize,
    /// Current column number
    column: usize,
}

impl CssTokenizer {
    /// Create a new CSS tokenizer
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Tokenize the input string
    pub fn tokenize(&mut self) -> Result<Vec<CssToken>> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            
            if let CssToken::Eof = &token {
                tokens.push(token);
                break;
            } else {
                tokens.push(token);
            }
        }
        
        debug!("Tokenized {} characters into {} tokens", self.input.len(), tokens.len());
        Ok(tokens)
    }
    
    /// Get the next token from the input
    fn next_token(&mut self) -> Result<CssToken> {
        if self.position >= self.input.len() {
            return Ok(CssToken::Eof);
        }
        
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Ok(CssToken::Eof);
        }
        
        let ch = self.current_char();
        
        match ch {
            // Identifiers and numbers
            'a'..='z' | 'A'..='Z' | '_' | '\u{00A0}'..='\u{10FFFF}' => {
                self.consume_identifier_or_number()
            }
            '0'..='9' => {
                self.consume_number()
            }
            
            // Strings
            '"' => self.consume_string('"'),
            '\'' => self.consume_string('\''),
            
            // Hash
            '#' => self.consume_hash(),
            
            // At-keyword
            '@' => self.consume_at_keyword(),
            
            // Delimiters and punctuation
            '.' => {
                self.advance();
                if self.current_char().is_ascii_digit() {
                    self.position -= 1; // Backtrack
                    self.consume_number()
                } else {
                    Ok(CssToken::Delim('.'))
                }
            }
            '+' => {
                self.advance();
                if self.current_char().is_ascii_digit() {
                    self.position -= 1; // Backtrack
                    self.consume_number()
                } else {
                    Ok(CssToken::Delim('+'))
                }
            }
            '-' => {
                self.advance();
                if self.current_char().is_ascii_digit() {
                    self.position -= 1; // Backtrack
                    self.consume_number()
                } else if self.current_char() == '-' {
                    self.advance();
                    Ok(CssToken::Delim('-'))
                } else {
                    Ok(CssToken::Delim('-'))
                }
            }
            '/' => {
                self.advance();
                if self.current_char() == '*' {
                    self.consume_comment()
                } else {
                    Ok(CssToken::Delim('/'))
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '!' {
                    self.advance();
                    if self.current_char() == '-' {
                        self.advance();
                        if self.current_char() == '-' {
                            self.consume_comment()
                        } else {
                            Err(Error::ConfigError("Invalid comment start".to_string()))
                        }
                    } else {
                        Err(Error::ConfigError("Invalid comment start".to_string()))
                    }
                } else {
                    Ok(CssToken::Delim('<'))
                }
            }
            '(' => {
                self.advance();
                Ok(CssToken::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(CssToken::RightParen)
            }
            '[' => {
                self.advance();
                Ok(CssToken::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(CssToken::RightBracket)
            }
            '{' => {
                self.advance();
                Ok(CssToken::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(CssToken::RightBrace)
            }
            ';' => {
                self.advance();
                Ok(CssToken::Semicolon)
            }
            ',' => {
                self.advance();
                Ok(CssToken::Delim(','))
            }
            ':' => {
                self.advance();
                Ok(CssToken::Colon)
            }
            
            // Other delimiters
            _ => {
                let ch = self.current_char();
                self.advance();
                Ok(CssToken::Delim(ch))
            }
        }
    }
    
    /// Consume an identifier or number
    fn consume_identifier_or_number(&mut self) -> Result<CssToken> {
        let start = self.position;
        
        // Consume the first character
        self.advance();
        
        // Consume subsequent characters
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == '\\' {
                self.consume_escape_sequence()?;
            } else if ch.is_alphanumeric() || ch == '_' || ch == '-' || 
                      (ch as u32) >= 0x80 {
                self.advance();
            } else {
                break;
            }
        }
        
        let value = self.input[start..self.position].to_string();
        
        // Check if it's a function
        if self.position < self.input.len() && self.current_char() == '(' {
            self.advance();
            Ok(CssToken::Function(value))
        } else {
            Ok(CssToken::Ident(value))
        }
    }
    
    /// Consume a number
    fn consume_number(&mut self) -> Result<CssToken> {
        let start = self.position;
        
        // Consume integer part
        while self.position < self.input.len() && self.current_char().is_ascii_digit() {
            self.advance();
        }
        
        // Check for decimal point
        if self.position < self.input.len() && self.current_char() == '.' {
            self.advance();
            
            // Consume fractional part
            while self.position < self.input.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        }
        
        // Check for exponent
        if self.position < self.input.len() && 
           (self.current_char() == 'e' || self.current_char() == 'E') {
            self.advance();
            
            if self.position < self.input.len() && 
               (self.current_char() == '+' || self.current_char() == '-') {
                self.advance();
            }
            
            // Must have at least one digit after exponent
            if self.position < self.input.len() && self.current_char().is_ascii_digit() {
                while self.position < self.input.len() && self.current_char().is_ascii_digit() {
                    self.advance();
                }
            } else {
                // Not a valid exponent, backtrack
                self.position = start;
                return self.consume_identifier_or_number();
            }
        }
        
        let number_str = self.input[start..self.position].to_string();
        let number = number_str.parse::<f64>()
            .map_err(|_| Error::ConfigError(format!("Invalid number: {}", number_str)))?;
        
        // Check for unit
        if self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch.is_alphabetic() || ch == '_' || (ch as u32) >= 0x80 {
                let unit_start = self.position;
                self.advance();
                
                while self.position < self.input.len() {
                    let ch = self.current_char();
                    if ch.is_alphanumeric() || ch == '_' || (ch as u32) >= 0x80 {
                        self.advance();
                    } else {
                        break;
                    }
                }
                
                let unit = self.input[unit_start..self.position].to_string();
                Ok(CssToken::Dimension(number, unit))
            } else if ch == '%' {
                self.advance();
                Ok(CssToken::Percentage(number))
            } else {
                Ok(CssToken::Number(number))
            }
        } else {
            Ok(CssToken::Number(number))
        }
    }
    
    /// Consume a string
    fn consume_string(&mut self, quote: char) -> Result<CssToken> {
        self.advance(); // Consume opening quote
        
        let mut value = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == quote {
                self.advance();
                return Ok(CssToken::String(value));
            } else if ch == '\\' {
                self.advance();
                if self.position < self.input.len() {
                    let escaped = self.current_char();
                    if escaped == '\n' {
                        // Ignore escaped newline
                        self.advance();
                    } else {
                        value.push(self.consume_escape_sequence()?);
                    }
                } else {
                    return Ok(CssToken::BadString);
                }
            } else if ch == '\n' {
                return Ok(CssToken::BadString);
            } else {
                value.push(ch);
                self.advance();
            }
        }
        
        Ok(CssToken::BadString)
    }
    
    /// Consume a hash
    fn consume_hash(&mut self) -> Result<CssToken> {
        self.advance(); // Consume '#'
        
        let start = self.position;
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || (ch as u32) >= 0x80 {
                self.advance();
            } else {
                break;
            }
        }
        
        let value = self.input[start..self.position].to_string();
        Ok(CssToken::Hash(value))
    }
    
    /// Consume an at-keyword
    fn consume_at_keyword(&mut self) -> Result<CssToken> {
        self.advance(); // Consume '@'
        
        let start = self.position;
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' || ch == '-' || (ch as u32) >= 0x80 {
                self.advance();
            } else {
                break;
            }
        }
        
        let value = self.input[start..self.position].to_string();
        Ok(CssToken::AtKeyword(value))
    }
    
    /// Consume a comment
    fn consume_comment(&mut self) -> Result<CssToken> {
        let mut value = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == '*' && self.position + 1 < self.input.len() && 
               self.input.chars().nth(self.position + 1) == Some('/') {
                self.advance(); // Consume '*'
                self.advance(); // Consume '/'
                return Ok(CssToken::Comment(value));
            } else {
                value.push(ch);
                self.advance();
            }
        }
        
        Err(Error::ConfigError("Unterminated comment".to_string()))
    }
    
    /// Consume an escape sequence
    fn consume_escape_sequence(&mut self) -> Result<char> {
        if self.position >= self.input.len() {
            return Err(Error::ConfigError("Unexpected end of input in escape sequence".to_string()));
        }
        
        let ch = self.current_char();
        
        if ch.is_ascii_hexdigit() {
            // Hex escape sequence
            let mut hex_value = 0u32;
            let mut digit_count = 0;
            
            while self.position < self.input.len() && digit_count < 6 {
                let ch = self.current_char();
                if ch.is_ascii_hexdigit() {
                    hex_value = hex_value * 16 + ch.to_digit(16).unwrap();
                    self.advance();
                    digit_count += 1;
                } else {
                    break;
                }
            }
            
            // Consume optional whitespace
            if self.position < self.input.len() && self.current_char().is_ascii_whitespace() {
                self.advance();
            }
            
            if hex_value == 0 {
                Ok('\u{FFFD}') // Replacement character
            } else if hex_value > 0x10FFFF {
                Ok('\u{FFFD}') // Replacement character
            } else {
                char::from_u32(hex_value).ok_or_else(|| {
                    Error::ConfigError("Invalid Unicode code point".to_string())
                })
            }
        } else {
            // Simple escape sequence
            self.advance();
            Ok(ch)
        }
    }
    
    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_ascii_whitespace() {
            self.advance();
        }
    }
    
    /// Get the current character
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }
    
    /// Advance to the next character
    fn advance(&mut self) {
        if self.position < self.input.len() {
            let ch = self.current_char();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_identifiers() {
        let mut tokenizer = CssTokenizer::new("div span");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3); // div, span, Eof
        assert_eq!(tokens[0], CssToken::Ident("div".to_string()));
        assert_eq!(tokens[1], CssToken::Ident("span".to_string()));
        assert_eq!(tokens[2], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_numbers() {
        let mut tokenizer = CssTokenizer::new("123 45.6");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3); // 123, 45.6, Eof
        assert_eq!(tokens[0], CssToken::Number(123.0));
        assert_eq!(tokens[1], CssToken::Number(45.6));
        assert_eq!(tokens[2], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_strings() {
        let mut tokenizer = CssTokenizer::new("\"hello world\"");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // "hello world", Eof
        assert_eq!(tokens[0], CssToken::String("hello world".to_string()));
        assert_eq!(tokens[1], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_hash() {
        let mut tokenizer = CssTokenizer::new("#main");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // #main, Eof
        assert_eq!(tokens[0], CssToken::Hash("main".to_string()));
        assert_eq!(tokens[1], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_delimiters() {
        let mut tokenizer = CssTokenizer::new(".,;:()[]{}");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 11); // 10 delimiters + Eof
        assert_eq!(tokens[0], CssToken::Delim('.'));
        assert_eq!(tokens[1], CssToken::Delim(','));
        assert_eq!(tokens[2], CssToken::Semicolon);
        assert_eq!(tokens[3], CssToken::Colon);
        assert_eq!(tokens[4], CssToken::LeftParen);
        assert_eq!(tokens[5], CssToken::RightParen);
        assert_eq!(tokens[6], CssToken::LeftBracket);
        assert_eq!(tokens[7], CssToken::RightBracket);
        assert_eq!(tokens[8], CssToken::LeftBrace);
        assert_eq!(tokens[9], CssToken::RightBrace);
        assert_eq!(tokens[10], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_dimensions() {
        let mut tokenizer = CssTokenizer::new("12px 2em");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3); // 12px, 2em, Eof
        assert_eq!(tokens[0], CssToken::Dimension(12.0, "px".to_string()));
        assert_eq!(tokens[1], CssToken::Dimension(2.0, "em".to_string()));
        assert_eq!(tokens[2], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_percentage() {
        let mut tokenizer = CssTokenizer::new("50%");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // 50%, Eof
        assert_eq!(tokens[0], CssToken::Percentage(50.0));
        assert_eq!(tokens[1], CssToken::Eof);
    }

    #[test]
    fn test_tokenize_function() {
        let mut tokenizer = CssTokenizer::new("url(");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // url(, Eof
        assert_eq!(tokens[0], CssToken::Function("url".to_string()));
        assert_eq!(tokens[1], CssToken::Eof);
    }
}
