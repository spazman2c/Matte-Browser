use crate::error::{Error, Result};
use std::collections::HashMap;

/// JavaScript token types
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,

    // Identifiers
    Identifier(String),

    // Keywords
    Let,
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    Try,
    Catch,
    Finally,
    Throw,
    Class,
    Extends,
    Super,
    New,
    This,
    Import,
    Export,
    From,
    As,
    Default,
    Async,
    Await,
    Yield,
    Generator,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    ExponentAssign,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Increment,
    Decrement,

    // Punctuation
    Semicolon,
    Comma,
    Dot,
    Colon,
    QuestionMark,
    ExclamationMark,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Arrow,

    // Template literals
    TemplateStart,
    TemplateEnd,
    TemplatePart,

    // Comments
    LineComment,
    BlockComment,

    // End of file
    Eof,
}

/// JavaScript token with position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, lexeme: String, position: usize, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            position,
            line,
            column,
        }
    }
}

/// JavaScript lexical analyzer
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    /// Create a new lexer for the given source code
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        
        // Add JavaScript keywords
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("const".to_string(), TokenType::Const);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("function".to_string(), TokenType::Function);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("do".to_string(), TokenType::Do);
        keywords.insert("switch".to_string(), TokenType::Switch);
        keywords.insert("case".to_string(), TokenType::Case);
        keywords.insert("default".to_string(), TokenType::Default);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("try".to_string(), TokenType::Try);
        keywords.insert("catch".to_string(), TokenType::Catch);
        keywords.insert("finally".to_string(), TokenType::Finally);
        keywords.insert("throw".to_string(), TokenType::Throw);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("extends".to_string(), TokenType::Extends);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("new".to_string(), TokenType::New);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("import".to_string(), TokenType::Import);
        keywords.insert("export".to_string(), TokenType::Export);
        keywords.insert("from".to_string(), TokenType::From);
        keywords.insert("as".to_string(), TokenType::As);
        keywords.insert("async".to_string(), TokenType::Async);
        keywords.insert("await".to_string(), TokenType::Await);
        keywords.insert("yield".to_string(), TokenType::Yield);
        keywords.insert("true".to_string(), TokenType::Boolean(true));
        keywords.insert("false".to_string(), TokenType::Boolean(false));
        keywords.insert("null".to_string(), TokenType::Null);
        keywords.insert("undefined".to_string(), TokenType::Undefined);

        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }

    /// Get the next token from the source
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token::new(
                TokenType::Eof,
                "".to_string(),
                self.position,
                self.line,
                self.column,
            ));
        }

        let start_position = self.position;
        let start_line = self.line;
        let start_column = self.column;

        let c = self.advance();

        match c {
            // Single character tokens
            '(' => self.make_token(TokenType::LeftParen, start_position, start_line, start_column),
            ')' => self.make_token(TokenType::RightParen, start_position, start_line, start_column),
            '{' => self.make_token(TokenType::LeftBrace, start_position, start_line, start_column),
            '}' => self.make_token(TokenType::RightBrace, start_position, start_line, start_column),
            '[' => self.make_token(TokenType::LeftBracket, start_position, start_line, start_column),
            ']' => self.make_token(TokenType::RightBracket, start_position, start_line, start_column),
            ';' => self.make_token(TokenType::Semicolon, start_position, start_line, start_column),
            ',' => self.make_token(TokenType::Comma, start_position, start_line, start_column),
            '.' => self.make_token(TokenType::Dot, start_position, start_line, start_column),
            ':' => self.make_token(TokenType::Colon, start_position, start_line, start_column),
            '?' => self.make_token(TokenType::QuestionMark, start_position, start_line, start_column),
            '!' => self.make_token(TokenType::ExclamationMark, start_position, start_line, start_column),

            // Two character tokens
            '+' => {
                if self.match_char('+') {
                    self.make_token(TokenType::Increment, start_position, start_line, start_column)
                } else if self.match_char('=') {
                    self.make_token(TokenType::PlusAssign, start_position, start_line, start_column)
                } else {
                    self.make_token(TokenType::Plus, start_position, start_line, start_column)
                }
            }
            '-' => {
                if self.match_char('-') {
                    self.make_token(TokenType::Decrement, start_position, start_line, start_column)
                } else if self.match_char('=') {
                    self.make_token(TokenType::MinusAssign, start_position, start_line, start_column)
                } else if self.match_char('>') {
                    self.make_token(TokenType::Arrow, start_position, start_line, start_column)
                } else {
                    self.make_token(TokenType::Minus, start_position, start_line, start_column)
                }
            }
            '*' => {
                if self.match_char('=') {
                    self.make_token(TokenType::MultiplyAssign, start_position, start_line, start_column)
                } else if self.match_char('*') {
                    if self.match_char('=') {
                        self.make_token(TokenType::ExponentAssign, start_position, start_line, start_column)
                    } else {
                        self.make_token(TokenType::Exponent, start_position, start_line, start_column)
                    }
                } else {
                    self.make_token(TokenType::Multiply, start_position, start_line, start_column)
                }
            }
            '/' => {
                if self.match_char('=') {
                    self.make_token(TokenType::DivideAssign, start_position, start_line, start_column)
                } else if self.match_char('/') {
                    self.line_comment()
                } else if self.match_char('*') {
                    self.block_comment()
                } else {
                    self.make_token(TokenType::Divide, start_position, start_line, start_column)
                }
            }
            '%' => {
                if self.match_char('=') {
                    self.make_token(TokenType::ModuloAssign, start_position, start_line, start_column)
                } else {
                    self.make_token(TokenType::Modulo, start_position, start_line, start_column)
                }
            }
            '=' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.make_token(TokenType::StrictEqual, start_position, start_line, start_column)
                    } else {
                        self.make_token(TokenType::Equal, start_position, start_line, start_column)
                    }
                } else {
                    self.make_token(TokenType::Assign, start_position, start_line, start_column)
                }
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.make_token(TokenType::StrictNotEqual, start_position, start_line, start_column)
                    } else {
                        self.make_token(TokenType::NotEqual, start_position, start_line, start_column)
                    }
                } else {
                    self.make_token(TokenType::LogicalNot, start_position, start_line, start_column)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessThanOrEqual, start_position, start_line, start_column)
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        self.make_token(TokenType::LeftShiftAssign, start_position, start_line, start_column)
                    } else {
                        self.make_token(TokenType::LeftShift, start_position, start_line, start_column)
                    }
                } else {
                    self.make_token(TokenType::LessThan, start_position, start_line, start_column)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterThanOrEqual, start_position, start_line, start_column)
                } else if self.match_char('>') {
                    if self.match_char('>') {
                        if self.match_char('=') {
                            self.make_token(TokenType::UnsignedRightShiftAssign, start_position, start_line, start_column)
                        } else {
                            self.make_token(TokenType::UnsignedRightShift, start_position, start_line, start_column)
                        }
                    } else if self.match_char('=') {
                        self.make_token(TokenType::RightShiftAssign, start_position, start_line, start_column)
                    } else {
                        self.make_token(TokenType::RightShift, start_position, start_line, start_column)
                    }
                } else {
                    self.make_token(TokenType::GreaterThan, start_position, start_line, start_column)
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.make_token(TokenType::LogicalAnd, start_position, start_line, start_column)
                } else if self.match_char('=') {
                    self.make_token(TokenType::BitwiseAndAssign, start_position, start_line, start_column)
                } else {
                    self.make_token(TokenType::BitwiseAnd, start_position, start_line, start_column)
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.make_token(TokenType::LogicalOr, start_position, start_line, start_column)
                } else if self.match_char('=') {
                    self.make_token(TokenType::BitwiseOrAssign, start_position, start_line, start_column)
                } else {
                    self.make_token(TokenType::BitwiseOr, start_position, start_line, start_column)
                }
            }
            '^' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BitwiseXorAssign, start_position, start_line, start_column)
                } else {
                    self.make_token(TokenType::BitwiseXor, start_position, start_line, start_column)
                }
            }
            '~' => self.make_token(TokenType::BitwiseNot, start_position, start_line, start_column),

            // String literals
            '"' | '\'' => self.string(c),

            // Template literals
            '`' => self.template_literal(),

            // Numbers
            '0'..='9' => self.number(),

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' | '$' => self.identifier(),

            _ => Err(Error::lexical(start_position, format!("Unexpected character: {}", c))),
        }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.advance();
        }
    }

    /// Check if we're at the end of the source
    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Get the current character without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    /// Get the next character without advancing
    fn peek_next(&self) -> char {
        if self.position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.position + 1]
        }
    }

    /// Advance to the next character
    fn advance(&mut self) -> char {
        let c = self.peek();
        self.position += 1;
        self.column += 1;
        c
    }

    /// Match and consume a character if it matches
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    /// Create a token
    fn make_token(&self, token_type: TokenType, start_position: usize, start_line: usize, start_column: usize) -> Result<Token> {
        let lexeme = self.source[start_position..self.position].iter().collect();
        Ok(Token::new(token_type, lexeme, start_position, start_line, start_column))
    }

    /// Parse a string literal
    fn string(&mut self, quote: char) -> Result<Token> {
        let start_position = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;

        while !self.is_at_end() && self.peek() != quote {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::lexical(start_position, "Unterminated string"));
        }

        // Consume the closing quote
        self.advance();

        let lexeme = self.source[start_position..self.position].iter().collect();
        Ok(Token::new(TokenType::String(lexeme), lexeme, start_position, start_line, start_column))
    }

    /// Parse a template literal
    fn template_literal(&mut self) -> Result<Token> {
        let start_position = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;

        while !self.is_at_end() && self.peek() != '`' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::lexical(start_position, "Unterminated template literal"));
        }

        // Consume the closing backtick
        self.advance();

        let lexeme = self.source[start_position..self.position].iter().collect();
        Ok(Token::new(TokenType::String(lexeme), lexeme, start_position, start_line, start_column))
    }

    /// Parse a number literal
    fn number(&mut self) -> Result<Token> {
        let start_position = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;

        // Parse integer part
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        // Parse fractional part
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume the '.'
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Parse exponent
        if !self.is_at_end() && (self.peek() == 'e' || self.peek() == 'E') {
            self.advance();
            if !self.is_at_end() && (self.peek() == '+' || self.peek() == '-') {
                self.advance();
            }
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[start_position..self.position].iter().collect();
        let number = lexeme.parse::<f64>().map_err(|_| {
            Error::lexical(start_position, format!("Invalid number: {}", lexeme))
        })?;

        Ok(Token::new(TokenType::Number(number), lexeme, start_position, start_line, start_column))
    }

    /// Parse an identifier or keyword
    fn identifier(&mut self) -> Result<Token> {
        let start_position = self.position - 1;
        let start_line = self.line;
        let start_column = self.column - 1;

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_' || self.peek() == '$') {
            self.advance();
        }

        let lexeme: String = self.source[start_position..self.position].iter().collect();
        let token_type = self.keywords.get(&lexeme).cloned().unwrap_or(TokenType::Identifier(lexeme.clone()));

        Ok(Token::new(token_type, lexeme, start_position, start_line, start_column))
    }

    /// Parse a line comment
    fn line_comment(&mut self) -> Result<Token> {
        let start_position = self.position - 2; // -2 for the '//'
        let start_line = self.line;
        let start_column = self.column - 2;

        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }

        let lexeme: String = self.source[start_position..self.position].iter().collect();
        Ok(Token::new(TokenType::LineComment, lexeme, start_position, start_line, start_column))
    }

    /// Parse a block comment
    fn block_comment(&mut self) -> Result<Token> {
        let start_position = self.position - 2; // -2 for the '/*'
        let start_line = self.line;
        let start_column = self.column - 2;

        while !self.is_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::lexical(start_position, "Unterminated block comment"));
        }

        // Consume the closing '*/'
        self.advance(); // '*'
        self.advance(); // '/'

        let lexeme: String = self.source[start_position..self.position].iter().collect();
        Ok(Token::new(TokenType::BlockComment, lexeme, start_position, start_line, start_column))
    }
}
