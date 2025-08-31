use common::error::{Error, Result};
use std::collections::VecDeque;

/// CSS token types according to the CSS specification
#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    /// Identifiers (e.g., class names, element names)
    Ident(String),
    /// Function tokens (e.g., url(), calc())
    Function(String),
    /// At-keyword tokens (e.g., @media, @import)
    AtKeyword(String),
    /// Hash tokens (e.g., #id)
    Hash(String, bool), // (value, is_id)
    /// String literals
    String(String),
    /// URL tokens
    Url(String),
    /// Number tokens
    Number(f64, String), // (value, representation)
    /// Percentage tokens
    Percentage(f64),
    /// Dimension tokens (e.g., 10px, 2em)
    Dimension(f64, String, String), // (value, unit, representation)
    /// Unicode range tokens
    UnicodeRange(u32, u32), // (start, end)
    /// Include match token
    IncludeMatch,
    /// Dash match token
    DashMatch,
    /// Prefix match token
    PrefixMatch,
    /// Suffix match token
    SuffixMatch,
    /// Substring match token
    SubstringMatch,
    /// Column token
    Column,
    /// Whitespace
    Whitespace,
    /// CDO (<!--)
    CDO,
    /// CDC (-->)
    CDC,
    /// Colon
    Colon,
    /// Semicolon
    Semicolon,
    /// Comma
    Comma,
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
    /// Delimiter (any other character)
    Delimiter(char),
    /// End of file
    EOF,
}

/// CSS tokenizer state machine
#[derive(Debug, Clone, PartialEq)]
enum TokenizerState {
    Data,
    String,
    StringEscape,
    Comment,
    Number,
    NumberFraction,
    NumberExponent,
    NumberExponentSign,
    Ident,
    Hash,
    Url,
    UrlEscape,
    UrlBad,
    AtKeyword,
    Function,
    UnicodeRange,
    Dimension,
}

/// CSS tokenizer implementation
pub struct CssTokenizer {
    input: Vec<char>,
    position: usize,
    state: TokenizerState,
    tokens: VecDeque<CssToken>,
    current_token: String,
    current_number_value: f64,
    current_number_representation: String,
    current_decimal_places: usize,
    current_is_negative: bool,
    current_escape_sequence: String,
    current_unicode_range_start: Option<u32>,
    current_unicode_range_end: Option<u32>,
}

impl CssTokenizer {
    /// Create a new CSS tokenizer
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            state: TokenizerState::Data,
            tokens: VecDeque::new(),
            current_token: String::new(),
            current_number_value: 0.0,
            current_number_representation: String::new(),
            current_decimal_places: 0,
            current_is_negative: false,
            current_escape_sequence: String::new(),
            current_unicode_range_start: None,
            current_unicode_range_end: None,
        }
    }

    /// Tokenize the input CSS
    pub fn tokenize(&mut self) -> Result<Vec<CssToken>> {
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            self.process_char(ch)?;
            self.position += 1;
        }

        // Handle end of input
        self.handle_eof()?;

        Ok(self.tokens.drain(..).collect())
    }

    /// Process a single character based on current state
    fn process_char(&mut self, ch: char) -> Result<()> {
        match self.state {
            TokenizerState::Data => self.handle_data_state(ch)?,
            TokenizerState::String => self.handle_string_state(ch)?,
            TokenizerState::StringEscape => self.handle_string_escape_state(ch)?,
            TokenizerState::Comment => self.handle_comment_state(ch)?,
            TokenizerState::Number => self.handle_number_state(ch)?,
            TokenizerState::NumberFraction => self.handle_number_fraction_state(ch)?,
            TokenizerState::NumberExponent => self.handle_number_exponent_state(ch)?,
            TokenizerState::NumberExponentSign => self.handle_number_exponent_sign_state(ch)?,
            TokenizerState::Ident => self.handle_ident_state(ch)?,
            TokenizerState::Hash => self.handle_hash_state(ch)?,
            TokenizerState::Url => self.handle_url_state(ch)?,
            TokenizerState::UrlEscape => self.handle_url_escape_state(ch)?,
            TokenizerState::UrlBad => self.handle_url_bad_state(ch)?,
            TokenizerState::AtKeyword => self.handle_at_keyword_state(ch)?,
            TokenizerState::Function => self.handle_function_state(ch)?,
            TokenizerState::UnicodeRange => self.handle_unicode_range_state(ch)?,
            TokenizerState::Dimension => self.handle_dimension_state(ch)?,
        }
        Ok(())
    }

    /// Handle the data state
    fn handle_data_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '\t' | '\n' | '\r' | ' ' => {
                self.emit_token(CssToken::Whitespace);
            }
            '"' => {
                self.state = TokenizerState::String;
                self.current_token.clear();
            }
            '#' => {
                self.state = TokenizerState::Hash;
                self.current_token.clear();
            }
            '\'' => {
                self.state = TokenizerState::String;
                self.current_token.clear();
            }
            '(' => {
                self.emit_token(CssToken::LeftParen);
            }
            ')' => {
                self.emit_token(CssToken::RightParen);
            }
            '*' => {
                self.emit_token(CssToken::Delimiter('*'));
            }
            '+' => {
                if self.would_start_number() {
                    self.state = TokenizerState::Number;
                    self.current_number_value = 0.0;
                    self.current_number_representation = ch.to_string();
                    self.current_decimal_places = 0;
                } else {
                    self.emit_token(CssToken::Delimiter('+'));
                }
            }
            ',' => {
                self.emit_token(CssToken::Comma);
            }
            '-' => {
                if self.would_start_number() {
                    self.state = TokenizerState::Number;
                    self.current_number_value = 0.0;
                    self.current_number_representation = ch.to_string();
                    self.current_is_negative = true;
                    self.current_decimal_places = 0;
                } else if self.would_start_identifier() {
                    self.state = TokenizerState::Ident;
                    self.current_token = ch.to_string();
                } else {
                    self.emit_token(CssToken::Delimiter('-'));
                }
            }
            '.' => {
                if self.would_start_number() {
                    self.state = TokenizerState::NumberFraction;
                    self.current_number_value = 0.0;
                    self.current_number_representation = ch.to_string();
                    self.current_decimal_places = 0;
                } else {
                    self.emit_token(CssToken::Delimiter('.'));
                }
            }
            ':' => {
                self.emit_token(CssToken::Colon);
            }
            ';' => {
                self.emit_token(CssToken::Semicolon);
            }
            '<' => {
                if self.peek_char() == Some('!') && self.peek_char_at(1) == Some('-') && self.peek_char_at(2) == Some('-') {
                    self.emit_token(CssToken::CDO);
                    self.position += 2; // Skip "!-"
                } else {
                    self.emit_token(CssToken::Delimiter('<'));
                }
            }
            '@' => {
                if self.position + 1 < self.input.len() && self.input[self.position + 1].is_alphabetic() {
                    self.state = TokenizerState::AtKeyword;
                    self.current_token = String::new(); // Don't include @ in the token
                } else {
                    self.emit_token(CssToken::Delimiter('@'));
                }
            }
            '[' => {
                self.emit_token(CssToken::LeftBracket);
            }
            '\\' => {
                if self.would_start_identifier() {
                    self.state = TokenizerState::Ident;
                    self.current_token = self.consume_escape_sequence()?;
                } else {
                    return Err(Error::ParseError("Invalid escape sequence".to_string()));
                }
            }
            ']' => {
                self.emit_token(CssToken::RightBracket);
            }
            '{' => {
                self.emit_token(CssToken::LeftBrace);
            }
            '}' => {
                self.emit_token(CssToken::RightBrace);
            }
            '0'..='9' => {
                self.state = TokenizerState::Number;
                self.current_number_value = ch.to_digit(10).unwrap() as f64;
                self.current_number_representation = ch.to_string();
                self.current_decimal_places = 0;
            }
            'U' | 'u' => {
                if self.peek_char() == Some('+') && self.is_hex_digit(self.peek_char_at(1)) {
                    self.state = TokenizerState::UnicodeRange;
                    self.current_unicode_range_start = None;
                    self.current_unicode_range_end = None;
                } else if self.would_start_identifier() {
                    self.state = TokenizerState::Ident;
                    self.current_token = ch.to_string();
                } else {
                    self.emit_token(CssToken::Delimiter(ch));
                }
            }
            _ if ch.is_alphabetic() => {
                self.state = TokenizerState::Ident;
                self.current_token = ch.to_string();
            }
            _ => {
                self.emit_token(CssToken::Delimiter(ch));
            }
        }
        Ok(())
    }

    /// Handle string state
    fn handle_string_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '"' | '\'' => {
                self.emit_token(CssToken::String(self.current_token.clone()));
                self.state = TokenizerState::Data;
            }
            '\\' => {
                self.state = TokenizerState::StringEscape;
            }
            '\n' => {
                // Unterminated string
                self.state = TokenizerState::Data;
            }
            _ => {
                self.current_token.push(ch);
            }
        }
        Ok(())
    }

    /// Handle string escape state
    fn handle_string_escape_state(&mut self, ch: char) -> Result<()> {
        if ch.is_ascii_hexdigit() {
            self.current_escape_sequence.push(ch);
            if self.current_escape_sequence.len() == 6 {
                // Handle hex escape
                let hex_value = u32::from_str_radix(&self.current_escape_sequence, 16)
                    .map_err(|_| Error::ParseError("Invalid hex escape".to_string()))?;
                if let Some(unicode_char) = char::from_u32(hex_value) {
                    self.current_token.push(unicode_char);
                }
                self.current_escape_sequence.clear();
                self.state = TokenizerState::String;
            }
        } else {
            // Handle single character escape
            self.current_token.push(ch);
            self.current_escape_sequence.clear();
            self.state = TokenizerState::String;
        }
        Ok(())
    }

    /// Handle comment state
    fn handle_comment_state(&mut self, ch: char) -> Result<()> {
        if ch == '*' && self.peek_char() == Some('/') {
            self.position += 1; // Skip '/'
            self.state = TokenizerState::Data;
        }
        Ok(())
    }

    /// Handle number state
    fn handle_number_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '0'..='9' => {
                self.current_number_value = self.current_number_value * 10.0 + ch.to_digit(10).unwrap() as f64;
                self.current_number_representation.push(ch);
            }
            '.' => {
                self.state = TokenizerState::NumberFraction;
                self.current_number_representation.push(ch);
            }
            'E' | 'e' => {
                self.state = TokenizerState::NumberExponent;
                self.current_number_representation.push(ch);
            }
            '%' => {
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Percentage(final_value));
                self.state = TokenizerState::Data;
            }
            _ if ch.is_alphabetic() => {
                self.state = TokenizerState::Dimension;
                self.current_token = ch.to_string();
            }
            _ => {
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Number(final_value, self.current_number_representation.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle number fraction state
    fn handle_number_fraction_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '0'..='9' => {
                self.current_decimal_places += 1;
                let decimal_value = ch.to_digit(10).unwrap() as f64;
                let divisor = 10.0_f64.powi(self.current_decimal_places as i32);
                self.current_number_value += decimal_value / divisor;
                self.current_number_representation.push(ch);
            }
            'E' | 'e' => {
                // Check if the next character is a digit (exponent) or a letter (dimension unit)
                if self.position + 1 < self.input.len() && self.input[self.position + 1].is_ascii_digit() {
                    self.state = TokenizerState::NumberExponent;
                    self.current_number_representation.push(ch);
                } else {
                    // This is a dimension unit, not an exponent
                    self.state = TokenizerState::Dimension;
                    self.current_token = ch.to_string();
                }
            }
            '%' => {
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Percentage(final_value));
                self.state = TokenizerState::Data;
            }
            _ if ch.is_alphabetic() => {
                self.state = TokenizerState::Dimension;
                self.current_token = ch.to_string();
            }
            _ => {
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Number(final_value, self.current_number_representation.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle number exponent state
    fn handle_number_exponent_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '+' | '-' => {
                self.state = TokenizerState::NumberExponentSign;
                self.current_number_representation.push(ch);
            }
            '0'..='9' => {
                // Handle exponent
                self.current_number_representation.push(ch);
            }
            _ => {
                // Invalid exponent
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Number(final_value, self.current_number_representation.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle number exponent sign state
    fn handle_number_exponent_sign_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '0'..='9' => {
                // Handle exponent digits
                self.current_number_representation.push(ch);
            }
            _ => {
                // Invalid exponent
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Number(final_value, self.current_number_representation.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle identifier state
    fn handle_ident_state(&mut self, ch: char) -> Result<()> {
        match ch {
            '(' => {
                self.emit_token(CssToken::Function(self.current_token.clone()));
                self.state = TokenizerState::Data;
            }
            _ if ch.is_alphanumeric() || ch == '-' || ch == '_' => {
                self.current_token.push(ch);
            }
            '\\' => {
                let escape = self.consume_escape_sequence()?;
                self.current_token.push_str(&escape);
            }
            _ => {
                self.emit_token(CssToken::Ident(self.current_token.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle hash state
    fn handle_hash_state(&mut self, ch: char) -> Result<()> {
        match ch {
            _ if ch.is_alphanumeric() || ch == '-' || ch == '_' => {
                self.current_token.push(ch);
            }
            '\\' => {
                let escape = self.consume_escape_sequence()?;
                self.current_token.push_str(&escape);
            }
            _ => {
                let is_id = self.current_token.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_' || c == '-');
                self.emit_token(CssToken::Hash(self.current_token.clone(), is_id));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle URL state
    fn handle_url_state(&mut self, ch: char) -> Result<()> {
        match ch {
            ')' => {
                self.emit_token(CssToken::Url(self.current_token.clone()));
                self.state = TokenizerState::Data;
            }
            '\\' => {
                self.state = TokenizerState::UrlEscape;
            }
            '\t' | '\n' | '\r' | ' ' => {
                // Skip whitespace
            }
            '"' | '\'' => {
                self.state = TokenizerState::UrlBad;
            }
            _ => {
                self.current_token.push(ch);
            }
        }
        Ok(())
    }

    /// Handle URL escape state
    fn handle_url_escape_state(&mut self, ch: char) -> Result<()> {
        if ch.is_ascii_hexdigit() {
            self.current_escape_sequence.push(ch);
            if self.current_escape_sequence.len() == 6 {
                // Handle hex escape
                let hex_value = u32::from_str_radix(&self.current_escape_sequence, 16)
                    .map_err(|_| Error::ParseError("Invalid hex escape".to_string()))?;
                if let Some(unicode_char) = char::from_u32(hex_value) {
                    self.current_token.push(unicode_char);
                }
                self.current_escape_sequence.clear();
                self.state = TokenizerState::Url;
            }
        } else {
            // Handle single character escape
            self.current_token.push(ch);
            self.current_escape_sequence.clear();
            self.state = TokenizerState::Url;
        }
        Ok(())
    }

    /// Handle URL bad state
    fn handle_url_bad_state(&mut self, ch: char) -> Result<()> {
        if ch == ')' {
            self.state = TokenizerState::Data;
        }
        Ok(())
    }

    /// Handle at-keyword state
    fn handle_at_keyword_state(&mut self, ch: char) -> Result<()> {
        match ch {
            _ if ch.is_alphanumeric() || ch == '-' || ch == '_' => {
                self.current_token.push(ch);
            }
            '\\' => {
                let escape = self.consume_escape_sequence()?;
                self.current_token.push_str(&escape);
            }
            _ => {
                self.emit_token(CssToken::AtKeyword(self.current_token.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle function state
    fn handle_function_state(&mut self, ch: char) -> Result<()> {
        match ch {
            _ if ch.is_alphanumeric() || ch == '-' || ch == '_' => {
                self.current_token.push(ch);
            }
            '\\' => {
                let escape = self.consume_escape_sequence()?;
                self.current_token.push_str(&escape);
            }
            _ => {
                self.emit_token(CssToken::Function(self.current_token.clone()));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle unicode range state
    fn handle_unicode_range_state(&mut self, ch: char) -> Result<()> {
        // Simplified unicode range handling
        if ch.is_ascii_hexdigit() {
            // Build the range
            if self.current_unicode_range_start.is_none() {
                self.current_unicode_range_start = Some(ch.to_digit(16).unwrap() as u32);
            } else {
                self.current_unicode_range_end = Some(ch.to_digit(16).unwrap() as u32);
            }
        } else if ch == '?' {
            // Handle wildcard
        } else {
            // End of unicode range
            let start = self.current_unicode_range_start.unwrap_or(0);
            let end = self.current_unicode_range_end.unwrap_or(start);
            self.emit_token(CssToken::UnicodeRange(start, end));
            self.state = TokenizerState::Data;
            // Reconsume the character
            self.position -= 1;
        }
        Ok(())
    }

    /// Handle dimension state
    fn handle_dimension_state(&mut self, ch: char) -> Result<()> {
        match ch {
            _ if ch.is_alphanumeric() || ch == '-' || ch == '_' => {
                self.current_token.push(ch);
            }
            '\\' => {
                let escape = self.consume_escape_sequence()?;
                self.current_token.push_str(&escape);
            }
            _ => {
                self.emit_token(CssToken::Dimension(
                    self.current_number_value,
                    self.current_token.clone(),
                    self.current_number_representation.clone(),
                ));
                self.state = TokenizerState::Data;
                // Reconsume the character
                self.position -= 1;
            }
        }
        Ok(())
    }

    /// Handle end of file
    fn handle_eof(&mut self) -> Result<()> {
        match self.state {
            TokenizerState::String => {
                // Unterminated string
                self.emit_token(CssToken::String(self.current_token.clone()));
            }
            TokenizerState::Url => {
                self.emit_token(CssToken::Url(self.current_token.clone()));
            }
            TokenizerState::Ident => {
                self.emit_token(CssToken::Ident(self.current_token.clone()));
            }
            TokenizerState::Hash => {
                let is_id = self.current_token.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_' || c == '-');
                self.emit_token(CssToken::Hash(self.current_token.clone(), is_id));
            }
            TokenizerState::Number | TokenizerState::NumberFraction => {
                let final_value = if self.current_is_negative { -self.current_number_value } else { self.current_number_value };
                self.emit_token(CssToken::Number(final_value, self.current_number_representation.clone()));
            }
            TokenizerState::AtKeyword => {
                self.emit_token(CssToken::AtKeyword(self.current_token.clone()));
            }
            _ => {}
        }
        self.emit_token(CssToken::EOF);
        Ok(())
    }

    /// Emit a token
    fn emit_token(&mut self, token: CssToken) {
        self.tokens.push_back(token);
    }

    /// Check if the next characters would start a number
    fn would_start_number(&self) -> bool {
        if self.position >= self.input.len() {
            return false;
        }
        
        let ch = self.input[self.position];
        if ch.is_ascii_digit() {
            return true;
        }
        
        if ch == '.' {
            return self.position + 1 < self.input.len() && self.input[self.position + 1].is_ascii_digit();
        }
        
        if ch == '+' || ch == '-' {
            if self.position + 1 >= self.input.len() {
                return false;
            }
            let next_ch = self.input[self.position + 1];
            return next_ch.is_ascii_digit() || (next_ch == '.' && self.position + 2 < self.input.len() && self.input[self.position + 2].is_ascii_digit());
        }
        
        false
    }

    /// Check if the next characters would start an identifier
    fn would_start_identifier(&self) -> bool {
        if self.position >= self.input.len() {
            return false;
        }
        
        let ch = self.input[self.position];
        ch.is_alphabetic() || ch == '_'
    }

    /// Peek at the next character
    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    /// Peek at a character at a specific offset
    fn peek_char_at(&self, offset: usize) -> Option<char> {
        if self.position + offset < self.input.len() {
            Some(self.input[self.position + offset])
        } else {
            None
        }
    }

    /// Check if a character is a hex digit
    fn is_hex_digit(&self, ch: Option<char>) -> bool {
        ch.map_or(false, |c| c.is_ascii_hexdigit())
    }

    /// Consume an escape sequence
    fn consume_escape_sequence(&mut self) -> Result<String> {
        if self.position >= self.input.len() {
            return Err(Error::ParseError("Unexpected end of input in escape sequence".to_string()));
        }
        
        let ch = self.input[self.position];
        if ch.is_ascii_hexdigit() {
            // Hex escape
            let mut hex_chars = String::new();
            hex_chars.push(ch);
            
            // Consume up to 5 more hex digits
            for _ in 0..5 {
                self.position += 1;
                if self.position >= self.input.len() {
                    break;
                }
                let next_ch = self.input[self.position];
                if next_ch.is_ascii_hexdigit() {
                    hex_chars.push(next_ch);
                } else {
                    break;
                }
            }
            
            // Parse hex value
            let hex_value = u32::from_str_radix(&hex_chars, 16)
                .map_err(|_| Error::ParseError("Invalid hex escape".to_string()))?;
            
            if let Some(unicode_char) = char::from_u32(hex_value) {
                Ok(unicode_char.to_string())
            } else {
                Ok(format!("\\u{:06X}", hex_value))
            }
        } else {
            // Single character escape
            self.position += 1;
            Ok(ch.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut tokenizer = CssTokenizer::new("div { color: red; }");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens[0], CssToken::Ident("div".to_string()));
        assert_eq!(tokens[1], CssToken::Whitespace);
        assert_eq!(tokens[2], CssToken::LeftBrace);
        assert_eq!(tokens[3], CssToken::Whitespace);
        assert_eq!(tokens[4], CssToken::Ident("color".to_string()));
        assert_eq!(tokens[5], CssToken::Colon);
        assert_eq!(tokens[6], CssToken::Whitespace);
        assert_eq!(tokens[7], CssToken::Ident("red".to_string()));
        assert_eq!(tokens[8], CssToken::Semicolon);
        assert_eq!(tokens[9], CssToken::Whitespace);
        assert_eq!(tokens[10], CssToken::RightBrace);
        assert_eq!(tokens[11], CssToken::EOF);
    }

    #[test]
    fn test_numbers() {
        let mut tokenizer = CssTokenizer::new("10px 2.5em -3.14");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens[0], CssToken::Dimension(10.0, "px".to_string(), "10".to_string()));
        assert_eq!(tokens[1], CssToken::Whitespace);
        assert_eq!(tokens[2], CssToken::Dimension(2.5, "em".to_string(), "2.5".to_string()));
        assert_eq!(tokens[3], CssToken::Whitespace);
        assert_eq!(tokens[4], CssToken::Number(-3.14, "-3.14".to_string()));
        assert_eq!(tokens[5], CssToken::EOF);
    }

    #[test]
    fn test_strings() {
        let mut tokenizer = CssTokenizer::new(r#""Hello, World!" 'test'"#);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens[0], CssToken::String("Hello, World!".to_string()));
        assert_eq!(tokens[1], CssToken::Whitespace);
        assert_eq!(tokens[2], CssToken::String("test".to_string()));
        assert_eq!(tokens[3], CssToken::EOF);
    }

    #[test]
    fn test_selectors() {
        let mut tokenizer = CssTokenizer::new("#id .class[attr='value']");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens[0], CssToken::Hash("id".to_string(), true));
        assert_eq!(tokens[1], CssToken::Whitespace);
        assert_eq!(tokens[2], CssToken::Delimiter('.'));
        assert_eq!(tokens[3], CssToken::Ident("class".to_string()));
        assert_eq!(tokens[4], CssToken::LeftBracket);
        assert_eq!(tokens[5], CssToken::Ident("attr".to_string()));
        assert_eq!(tokens[6], CssToken::Delimiter('='));
        assert_eq!(tokens[7], CssToken::String("value".to_string()));
        assert_eq!(tokens[8], CssToken::RightBracket);
        assert_eq!(tokens[9], CssToken::EOF);
    }

    #[test]
    fn test_at_rules() {
        let mut tokenizer = CssTokenizer::new("@media @import");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens[0], CssToken::AtKeyword("media".to_string()));
        assert_eq!(tokens[1], CssToken::Whitespace);
        assert_eq!(tokens[2], CssToken::AtKeyword("import".to_string()));
        assert_eq!(tokens[3], CssToken::EOF);
    }

    #[test]
    fn test_functions() {
        let mut tokenizer = CssTokenizer::new("url('test.jpg') calc(100% - 20px)");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens[0], CssToken::Function("url".to_string()));
        assert_eq!(tokens[1], CssToken::String("test.jpg".to_string()));
        assert_eq!(tokens[2], CssToken::RightParen);
        assert_eq!(tokens[3], CssToken::Whitespace);
        assert_eq!(tokens[4], CssToken::Function("calc".to_string()));
        assert_eq!(tokens[5], CssToken::Percentage(100.0));
        assert_eq!(tokens[6], CssToken::Whitespace);
        assert_eq!(tokens[7], CssToken::Delimiter('-'));
        assert_eq!(tokens[8], CssToken::Whitespace);
        assert_eq!(tokens[9], CssToken::Dimension(20.0, "px".to_string(), "20".to_string()));
        assert_eq!(tokens[10], CssToken::RightParen);
        assert_eq!(tokens[11], CssToken::EOF);
    }
}
