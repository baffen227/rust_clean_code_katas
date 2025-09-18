// ============================================
// Kata 8: Simple Parser
// Focus: Combinator Pattern, Functional Style, Error Handling
// ============================================

mod parser {
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub enum ParseError {
        UnexpectedCharacter(char, usize),
        UnexpectedEndOfInput,
        InvalidNumber(String),
        InvalidEscape(char),
        UnclosedString,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ParseError::UnexpectedCharacter(ch, pos) => {
                    write!(f, "Unexpected character '{}' at position {}", ch, pos)
                }
                ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
                ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
                ParseError::InvalidEscape(ch) => write!(f, "Invalid escape sequence: \\{}", ch),
                ParseError::UnclosedString => write!(f, "Unclosed string"),
            }
        }
    }

    type ParseResult<'a, T> = Result<(T, &'a str), ParseError>;

    // Basic Parser trait
    pub trait Parser<T> {
        fn parse<'a>(&self, input: &'a str) -> ParseResult<T>;
    }

    // CSV Parser
    pub struct CsvParser {
        delimiter: char,
    }

    impl CsvParser {
        pub fn new(delimiter: char) -> Self {
            CsvParser { delimiter }
        }

        pub fn parse_line(&self, input: &str) -> Result<Vec<String>, ParseError> {
            if input.is_empty() {
                return Ok(Vec::new());
            }

            let mut result = Vec::new();
            let mut current_field = String::new();
            let mut in_quotes = false;
            let mut escape_next = false;
            let mut chars = input.chars().peekable();

            while let Some(ch) = chars.next() {
                if escape_next {
                    current_field.push(ch);
                    escape_next = false;
                } else if ch == '\\' && in_quotes {
                    escape_next = true;
                } else if ch == '"' {
                    in_quotes = !in_quotes;
                } else if ch == self.delimiter && !in_quotes {
                    result.push(current_field.trim().to_string());
                    current_field = String::new();
                } else {
                    current_field.push(ch);
                }
            }

            if in_quotes {
                return Err(ParseError::UnclosedString);
            }

            result.push(current_field.trim().to_string());
            Ok(result)
        }

        pub fn parse_document(&self, input: &str) -> Result<Vec<Vec<String>>, ParseError> {
            input
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| self.parse_line(line))
                .collect()
        }
    }

    // Simple JSON Value
    #[derive(Debug, PartialEq, Clone)]
    pub enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(Vec<(String, JsonValue)>),
    }

    // JSON Parser (simplified version)
    pub struct JsonParser;

    impl JsonParser {
        pub fn new() -> Self {
            JsonParser
        }

        fn skip_whitespace(input: &str) -> &str {
            input.trim_start()
        }

        fn parse_null(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);
            if input.starts_with("null") {
                Ok((JsonValue::Null, &input[4..]))
            } else {
                Err(ParseError::UnexpectedCharacter(
                    input.chars().next().unwrap_or('\0'),
                    0,
                ))
            }
        }

        fn parse_bool(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);
            if input.starts_with("true") {
                Ok((JsonValue::Bool(true), &input[4..]))
            } else if input.starts_with("false") {
                Ok((JsonValue::Bool(false), &input[5..]))
            } else {
                Err(ParseError::UnexpectedCharacter(
                    input.chars().next().unwrap_or('\0'),
                    0,
                ))
            }
        }

        fn parse_number(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);
            let mut end_idx = 0;
            let mut has_dot = false;

            for (idx, ch) in input.char_indices() {
                if ch.is_numeric() || (ch == '.' && !has_dot) || (ch == '-' && idx == 0) {
                    if ch == '.' {
                        has_dot = true;
                    }
                    end_idx = idx + 1;
                } else {
                    break;
                }
            }

            if end_idx == 0 {
                return Err(ParseError::InvalidNumber(String::new()));
            }

            let num_str = &input[..end_idx];
            match num_str.parse::<f64>() {
                Ok(num) => Ok((JsonValue::Number(num), &input[end_idx..])),
                Err(_) => Err(ParseError::InvalidNumber(num_str.to_string())),
            }
        }

        fn parse_string(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);
            if !input.starts_with('"') {
                return Err(ParseError::UnexpectedCharacter(
                    input.chars().next().unwrap_or('\0'),
                    0,
                ));
            }

            let mut result = String::new();
            let mut chars = input[1..].chars();
            let mut escape_next = false;

            while let Some(ch) = chars.next() {
                if escape_next {
                    match ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        _ => return Err(ParseError::InvalidEscape(ch)),
                    }
                    escape_next = false;
                } else if ch == '\\' {
                    escape_next = true;
                } else if ch == '"' {
                    let remaining = chars.as_str();
                    return Ok((JsonValue::String(result), remaining));
                } else {
                    result.push(ch);
                }
            }

            Err(ParseError::UnclosedString)
        }

        pub fn parse_value(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);

            if input.is_empty() {
                return Err(ParseError::UnexpectedEndOfInput);
            }

            match input.chars().next().unwrap() {
                'n' => Self::parse_null(input),
                't' | 'f' => Self::parse_bool(input),
                '"' => Self::parse_string(input),
                '[' => Self::parse_array(input),
                '{' => Self::parse_object(input),
                '-' | '0'..='9' => Self::parse_number(input),
                ch => Err(ParseError::UnexpectedCharacter(ch, 0)),
            }
        }

        fn parse_array(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);
            if !input.starts_with('[') {
                return Err(ParseError::UnexpectedCharacter(
                    input.chars().next().unwrap_or('\0'),
                    0,
                ));
            }

            let mut input = &input[1..];
            let mut elements = Vec::new();

            loop {
                input = Self::skip_whitespace(input);
                if input.starts_with(']') {
                    return Ok((JsonValue::Array(elements), &input[1..]));
                }

                if !elements.is_empty() {
                    if !input.starts_with(',') {
                        return Err(ParseError::UnexpectedCharacter(
                            input.chars().next().unwrap_or('\0'),
                            0,
                        ));
                    }
                    input = &input[1..];
                    input = Self::skip_whitespace(input);
                }

                let (value, remaining) = Self::parse_value(input)?;
                elements.push(value);
                input = remaining;
            }
        }

        fn parse_object(input: &str) -> ParseResult<JsonValue> {
            let input = Self::skip_whitespace(input);
            if !input.starts_with('{') {
                return Err(ParseError::UnexpectedCharacter(
                    input.chars().next().unwrap_or('\0'),
                    0,
                ));
            }

            let mut input = &input[1..];
            let mut pairs = Vec::new();

            loop {
                input = Self::skip_whitespace(input);
                if input.starts_with('}') {
                    return Ok((JsonValue::Object(pairs), &input[1..]));
                }

                if !pairs.is_empty() {
                    if !input.starts_with(',') {
                        return Err(ParseError::UnexpectedCharacter(
                            input.chars().next().unwrap_or('\0'),
                            0,
                        ));
                    }
                    input = &input[1..];
                    input = Self::skip_whitespace(input);
                }

                // Parse key
                let (key_value, remaining) = Self::parse_string(input)?;
                let key = match key_value {
                    JsonValue::String(s) => s,
                    _ => unreachable!(),
                };

                input = Self::skip_whitespace(remaining);
                if !input.starts_with(':') {
                    return Err(ParseError::UnexpectedCharacter(
                        input.chars().next().unwrap_or('\0'),
                        0,
                    ));
                }

                input = &input[1..];
                let (value, remaining) = Self::parse_value(input)?;
                pairs.push((key, value));
                input = remaining;
            }
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::parser::*;

    #[test]
    fn test_csv_parser() {
        let parser = CsvParser::new(',');

        let result = parser.parse_line("name,age,city").unwrap();
        assert_eq!(result, vec!["name", "age", "city"]);

        let result = parser.parse_line(r#"John Doe",30,"New York"#).unwrap();
        assert_eq!(result, vec!["John Doe", "30", "New York"]);
    }

    #[test]
    fn test_json_parser() {
        let (value, _) = JsonParser::parse_value("123.45").unwrap();
        assert_eq!(value, JsonValue::Number(123.45));

        let (value, _) = JsonParser::parse_value(r#""hello world""#).unwrap();
        assert_eq!(value, JsonValue::String("hello world".to_string()));

        let (value, _) = JsonParser::parse_value("[1, 2, 3]").unwrap();
        assert_eq!(
            value,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }
}
