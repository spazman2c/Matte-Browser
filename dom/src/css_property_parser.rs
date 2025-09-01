use crate::css_tokenizer::{CssToken, CssTokenizer};
use crate::cssom::CssValue;
use crate::error::{Error, Result};

/// CSS property value parser
pub struct CssPropertyParser {
    /// Current position in the token stream
    position: usize,
    /// Tokens to parse
    tokens: Vec<CssToken>,
}

/// CSS property value types
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// String value
    String(String),
    /// Number value
    Number(f32),
    /// Percentage value
    Percentage(f32),
    /// Length value with unit
    Length(f32, LengthUnit),
    /// Color value
    Color(ColorValue),
    /// URL value
    Url(String),
    /// Function call
    Function(String, Vec<PropertyValue>),
    /// List of values
    List(Vec<PropertyValue>),
    /// Keyword value
    Keyword(String),
    /// Initial value
    Initial,
    /// Inherit value
    Inherit,
    /// Unset value
    Unset,
}

/// CSS length units
#[derive(Debug, Clone, PartialEq)]
pub enum LengthUnit {
    Px,
    Em,
    Rem,
    Ex,
    Ch,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Pt,
    Pc,
    In,
    Mm,
    Cm,
    Q,
    Percent,
}

/// CSS color values
#[derive(Debug, Clone, PartialEq)]
pub enum ColorValue {
    /// RGB color
    Rgb(u8, u8, u8),
    /// RGBA color
    Rgba(u8, u8, u8, f32),
    /// HSL color
    Hsl(u16, u8, u8),
    /// HSLA color
    Hsla(u16, u8, u8, f32),
    /// Hex color
    Hex(String),
    /// Named color
    Named(String),
    /// Current color
    CurrentColor,
    /// Transparent
    Transparent,
}

impl CssPropertyParser {
    /// Create a new CSS property parser
    pub fn new() -> Self {
        Self {
            position: 0,
            tokens: Vec::new(),
        }
    }
    
    /// Parse a CSS property value string
    pub fn parse_property_value(&mut self, input: &str) -> Result<PropertyValue> {
        // Tokenize the input
        let mut tokenizer = CssTokenizer::new(input);
        self.tokens = tokenizer.tokenize()?;
        self.position = 0;
        
        // Parse the value
        self.parse_value()
    }
    
    /// Parse a single value
    fn parse_value(&mut self) -> Result<PropertyValue> {
        if self.position >= self.tokens.len() {
            return Err(Error::ParseError("Unexpected end of input".to_string()));
        }
        
        let token = self.tokens[self.position].clone();
        
        match token {
            CssToken::String(value) => {
                self.position += 1;
                Ok(PropertyValue::String(value))
            }
            CssToken::Number(value) => {
                self.position += 1;
                Ok(PropertyValue::Number(value as f32))
            }
            CssToken::Percentage(value) => {
                self.position += 1;
                Ok(PropertyValue::Percentage(value as f32))
            }
            CssToken::Dimension(value, unit) => {
                self.position += 1;
                self.parse_dimension(value as f32, &unit)
            }
            CssToken::Hash(value) => {
                self.position += 1;
                self.parse_color_hash(&value)
            }
            CssToken::Function(name) => {
                self.position += 1;
                self.parse_function(&name)
            }
            CssToken::Ident(value) => {
                self.position += 1;
                self.parse_identifier(&value)
            }
            CssToken::Url(value) => {
                self.position += 1;
                Ok(PropertyValue::Url(value))
            }
            _ => Err(Error::ParseError(format!("Unexpected token: {:?}", token))),
        }
    }
    
    /// Parse a dimension value
    fn parse_dimension(&self, value: f32, unit: &str) -> Result<PropertyValue> {
        match unit.to_lowercase().as_str() {
            "px" => Ok(PropertyValue::Length(value, LengthUnit::Px)),
            "em" => Ok(PropertyValue::Length(value, LengthUnit::Em)),
            "rem" => Ok(PropertyValue::Length(value, LengthUnit::Rem)),
            "ex" => Ok(PropertyValue::Length(value, LengthUnit::Ex)),
            "ch" => Ok(PropertyValue::Length(value, LengthUnit::Ch)),
            "vw" => Ok(PropertyValue::Length(value, LengthUnit::Vw)),
            "vh" => Ok(PropertyValue::Length(value, LengthUnit::Vh)),
            "vmin" => Ok(PropertyValue::Length(value, LengthUnit::Vmin)),
            "vmax" => Ok(PropertyValue::Length(value, LengthUnit::Vmax)),
            "pt" => Ok(PropertyValue::Length(value, LengthUnit::Pt)),
            "pc" => Ok(PropertyValue::Length(value, LengthUnit::Pc)),
            "in" => Ok(PropertyValue::Length(value, LengthUnit::In)),
            "mm" => Ok(PropertyValue::Length(value, LengthUnit::Mm)),
            "cm" => Ok(PropertyValue::Length(value, LengthUnit::Cm)),
            "q" => Ok(PropertyValue::Length(value, LengthUnit::Q)),
            "%" => Ok(PropertyValue::Length(value, LengthUnit::Percent)),
            _ => Err(Error::ParseError(format!("Unknown unit: {}", unit))),
        }
    }
    
    /// Parse a color hash value
    fn parse_color_hash(&self, value: &str) -> Result<PropertyValue> {
        // The hash token contains the value without the '#' prefix
        // For color values, we need to add the '#' back
        Ok(PropertyValue::Color(ColorValue::Hex(format!("#{}", value))))
    }
    
    /// Parse a function call
    fn parse_function(&mut self, name: &str) -> Result<PropertyValue> {
        let mut arguments = Vec::new();
        
        // Parse arguments until closing parenthesis
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            
            match token {
                CssToken::RightParen => {
                    self.position += 1;
                    break;
                }
                CssToken::Comma => {
                    self.position += 1;
                    continue;
                }
                CssToken::Delim(',') => {
                    self.position += 1;
                    continue;
                }
                _ => {
                    let arg = self.parse_value()?;
                    arguments.push(arg);
                }
            }
        }
        
        // Parse specific functions
        match name.to_lowercase().as_str() {
            "rgb" => self.parse_rgb_function(arguments),
            "rgba" => self.parse_rgba_function(arguments),
            "hsl" => self.parse_hsl_function(arguments),
            "hsla" => self.parse_hsla_function(arguments),
            _ => Ok(PropertyValue::Function(name.to_string(), arguments)),
        }
    }
    
    /// Parse RGB function
    fn parse_rgb_function(&self, args: Vec<PropertyValue>) -> Result<PropertyValue> {
        if args.len() != 3 {
            return Err(Error::ParseError("RGB function requires exactly 3 arguments".to_string()));
        }
        
        let r = self.extract_number(&args[0])?;
        let g = self.extract_number(&args[1])?;
        let b = self.extract_number(&args[2])?;
        
        Ok(PropertyValue::Color(ColorValue::Rgb(r, g, b)))
    }
    
    /// Parse RGBA function
    fn parse_rgba_function(&self, args: Vec<PropertyValue>) -> Result<PropertyValue> {
        if args.len() != 4 {
            return Err(Error::ParseError("RGBA function requires exactly 4 arguments".to_string()));
        }
        
        let r = self.extract_number(&args[0])?;
        let g = self.extract_number(&args[1])?;
        let b = self.extract_number(&args[2])?;
        let a = self.extract_number_f32(&args[3])?;
        
        Ok(PropertyValue::Color(ColorValue::Rgba(r, g, b, a)))
    }
    
    /// Parse HSL function
    fn parse_hsl_function(&self, args: Vec<PropertyValue>) -> Result<PropertyValue> {
        if args.len() != 3 {
            return Err(Error::ParseError("HSL function requires exactly 3 arguments".to_string()));
        }
        
        let h = self.extract_number_u16(&args[0])?;
        let s = self.extract_number(&args[1])?;
        let l = self.extract_number(&args[2])?;
        
        Ok(PropertyValue::Color(ColorValue::Hsl(h, s, l)))
    }
    
    /// Parse HSLA function
    fn parse_hsla_function(&self, args: Vec<PropertyValue>) -> Result<PropertyValue> {
        if args.len() != 4 {
            return Err(Error::ParseError("HSLA function requires exactly 4 arguments".to_string()));
        }
        
        let h = self.extract_number_u16(&args[0])?;
        let s = self.extract_number(&args[1])?;
        let l = self.extract_number(&args[2])?;
        let a = self.extract_number_f32(&args[3])?;
        
        Ok(PropertyValue::Color(ColorValue::Hsla(h, s, l, a)))
    }
    
    /// Parse an identifier
    fn parse_identifier(&self, value: &str) -> Result<PropertyValue> {
        match value.to_lowercase().as_str() {
            "initial" => Ok(PropertyValue::Initial),
            "inherit" => Ok(PropertyValue::Inherit),
            "unset" => Ok(PropertyValue::Unset),
            "currentcolor" => Ok(PropertyValue::Color(ColorValue::CurrentColor)),
            "transparent" => Ok(PropertyValue::Color(ColorValue::Transparent)),
            _ => {
                // Check if it's a named color
                if self.is_named_color(value) {
                    Ok(PropertyValue::Color(ColorValue::Named(value.to_string())))
                } else {
                    Ok(PropertyValue::Keyword(value.to_string()))
                }
            }
        }
    }
    
    /// Extract number from property value
    fn extract_number(&self, value: &PropertyValue) -> Result<u8> {
        match value {
            PropertyValue::Number(n) => {
                if *n >= 0.0 && *n <= 255.0 {
                    Ok(*n as u8)
                } else {
                    Err(Error::ParseError("Number out of range for color component".to_string()))
                }
            }
            PropertyValue::Percentage(p) => {
                let n = (*p / 100.0) * 255.0;
                if n >= 0.0 && n <= 255.0 {
                    Ok(n as u8)
                } else {
                    Err(Error::ParseError("Percentage out of range for color component".to_string()))
                }
            }
            _ => Err(Error::ParseError("Expected number or percentage".to_string())),
        }
    }
    
    /// Extract number as f32 from property value
    fn extract_number_f32(&self, value: &PropertyValue) -> Result<f32> {
        match value {
            PropertyValue::Number(n) => Ok(*n),
            _ => Err(Error::ParseError("Expected number".to_string())),
        }
    }
    
    /// Extract number as u16 from property value
    fn extract_number_u16(&self, value: &PropertyValue) -> Result<u16> {
        match value {
            PropertyValue::Number(n) => {
                if *n >= 0.0 && *n <= 360.0 {
                    Ok(*n as u16)
                } else {
                    Err(Error::ParseError("Number out of range for hue".to_string()))
                }
            }
            _ => Err(Error::ParseError("Expected number".to_string())),
        }
    }
    
    /// Check if a string is a named color
    fn is_named_color(&self, name: &str) -> bool {
        let named_colors = [
            "black", "white", "red", "green", "blue", "yellow", "cyan", "magenta",
            "gray", "grey", "orange", "purple", "brown", "pink", "lime", "navy",
            "teal", "silver", "gold", "maroon", "olive", "aqua", "fuchsia"
        ];
        
        named_colors.contains(&name.to_lowercase().as_str())
    }
    
    /// Parse a list of values
    pub fn parse_value_list(&mut self, input: &str) -> Result<Vec<PropertyValue>> {
        // Tokenize the input
        let mut tokenizer = CssTokenizer::new(input);
        self.tokens = tokenizer.tokenize()?;
        self.position = 0;
        
        let mut values = Vec::new();
        
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            
            // Check for EOF
            if let CssToken::Eof = token {
                break;
            }
            
            let value = self.parse_value()?;
            values.push(value);
            
            // Skip comma if present
            if self.position < self.tokens.len() {
                match self.tokens[self.position] {
                    CssToken::Comma | CssToken::Delim(',') => {
                        self.position += 1;
                    }
                    _ => {}
                }
            }
        }
        
        Ok(values)
    }
    
    /// Convert property value to CSS value
    pub fn to_css_value(&self, property_value: &PropertyValue) -> CssValue {
        match property_value {
            PropertyValue::String(s) => CssValue::String(s.clone()),
            PropertyValue::Number(n) => CssValue::Number((*n).into()),
            PropertyValue::Percentage(p) => CssValue::Percentage((*p).into()),
            PropertyValue::Length(v, unit) => {
                let unit_str = match unit {
                    LengthUnit::Px => "px",
                    LengthUnit::Em => "em",
                    LengthUnit::Rem => "rem",
                    LengthUnit::Ex => "ex",
                    LengthUnit::Ch => "ch",
                    LengthUnit::Vw => "vw",
                    LengthUnit::Vh => "vh",
                    LengthUnit::Vmin => "vmin",
                    LengthUnit::Vmax => "vmax",
                    LengthUnit::Pt => "pt",
                    LengthUnit::Pc => "pc",
                    LengthUnit::In => "in",
                    LengthUnit::Mm => "mm",
                    LengthUnit::Cm => "cm",
                    LengthUnit::Q => "q",
                    LengthUnit::Percent => "%",
                };
                CssValue::Length((*v).into(), unit_str.to_string())
            }
            PropertyValue::Color(color) => {
                let color_str = match color {
                    ColorValue::Rgb(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
                    ColorValue::Rgba(r, g, b, a) => format!("rgba({}, {}, {}, {})", r, g, b, a),
                    ColorValue::Hsl(h, s, l) => format!("hsl({}, {}%, {}%)", h, s, l),
                    ColorValue::Hsla(h, s, l, a) => format!("hsla({}, {}%, {}%, {})", h, s, l, a),
                    ColorValue::Hex(h) => h.clone(),
                    ColorValue::Named(n) => n.clone(),
                    ColorValue::CurrentColor => "currentColor".to_string(),
                    ColorValue::Transparent => "transparent".to_string(),
                };
                CssValue::String(color_str)
            }
            PropertyValue::Url(u) => CssValue::Url(u.clone()),
            PropertyValue::Function(name, args) => {
                let args_str = args.iter()
                    .map(|arg| format!("{:?}", self.to_css_value(arg)))
                    .collect::<Vec<_>>()
                    .join(", ");
                CssValue::String(format!("{}({})", name, args_str))
            }
            PropertyValue::List(values) => {
                let values_str = values.iter()
                    .map(|v| format!("{:?}", self.to_css_value(v)))
                    .collect::<Vec<_>>()
                    .join(", ");
                CssValue::String(values_str)
            }
            PropertyValue::Keyword(k) => CssValue::String(k.clone()),
            PropertyValue::Initial => CssValue::String("initial".to_string()),
            PropertyValue::Inherit => CssValue::String("inherit".to_string()),
            PropertyValue::Unset => CssValue::String("unset".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("\"hello world\"");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PropertyValue::String(_)));
    }

    #[test]
    fn test_parse_number_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("42");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PropertyValue::Number(42.0)));
    }

    #[test]
    fn test_parse_percentage_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("50%");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PropertyValue::Percentage(50.0)));
    }

    #[test]
    fn test_parse_length_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("16px");
        assert!(result.is_ok());
        if let PropertyValue::Length(value, unit) = result.unwrap() {
            assert_eq!(value, 16.0);
            assert_eq!(unit, LengthUnit::Px);
        } else {
            panic!("Expected length value");
        }
    }

    #[test]
    fn test_parse_color_hex() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("#ff0000");
        assert!(result.is_ok());
        if let PropertyValue::Color(ColorValue::Hex(value)) = result.unwrap() {
            assert_eq!(value, "#ff0000");
        } else {
            panic!("Expected hex color");
        }
    }

    #[test]
    fn test_parse_color_rgb() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("rgb(255, 0, 0)");
        assert!(result.is_ok());
        if let PropertyValue::Color(ColorValue::Rgb(r, g, b)) = result.unwrap() {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
        } else {
            panic!("Expected RGB color");
        }
    }

    #[test]
    fn test_parse_color_rgba() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("rgba(255, 0, 0, 0.5)");
        assert!(result.is_ok());
        if let PropertyValue::Color(ColorValue::Rgba(r, g, b, a)) = result.unwrap() {
            assert_eq!(r, 255);
            assert_eq!(g, 0);
            assert_eq!(b, 0);
            assert_eq!(a, 0.5);
        } else {
            panic!("Expected RGBA color");
        }
    }

    #[test]
    fn test_parse_keyword_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("auto");
        assert!(result.is_ok());
        if let PropertyValue::Keyword(value) = result.unwrap() {
            assert_eq!(value, "auto");
        } else {
            panic!("Expected keyword value");
        }
    }

    #[test]
    fn test_parse_initial_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("initial");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PropertyValue::Initial));
    }

    #[test]
    fn test_parse_inherit_value() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_property_value("inherit");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), PropertyValue::Inherit));
    }

    #[test]
    fn test_parse_value_list() {
        let mut parser = CssPropertyParser::new();
        let result = parser.parse_value_list("1px, 2px, 3px");
        if let Err(e) = &result {
            println!("Value list error: {:?}", e);
        }
        assert!(result.is_ok());
        let values = result.unwrap();
        assert_eq!(values.len(), 3);
        
        if let PropertyValue::Length(value, unit) = &values[0] {
            assert_eq!(*value, 1.0);
            assert_eq!(*unit, LengthUnit::Px);
        } else {
            panic!("Expected length value");
        }
    }

    #[test]
    fn test_to_css_value() {
        let parser = CssPropertyParser::new();
        let property_value = PropertyValue::Length(16.0, LengthUnit::Px);
        let css_value = parser.to_css_value(&property_value);
        
        if let CssValue::Length(value, unit) = css_value {
            assert_eq!(value, 16.0);
            assert_eq!(unit, "px");
        } else {
            panic!("Expected dimension CSS value");
        }
    }
}
