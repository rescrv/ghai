use serde_json::Value;

/// Parse a line containing text followed by JSON into separate components
/// Returns (text, json) where text is the prefix and json is the parsed JSON value
///
/// The parser tries to find the longest valid JSON suffix by iterating from the end
/// of the string. This approach handles cases where the text portion might contain
/// JSON-like syntax that should not be parsed as JSON.
pub fn parse_text_json(line: &str) -> Result<(String, Value), ParseError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    // Collect characters for unicode-safe iteration
    let chars: Vec<char> = line.chars().collect();
    let char_count = chars.len();

    // Try parsing JSON from the end, starting with 1 character and growing
    // This is unicode-safe as we work with character boundaries
    // We need to find the LONGEST valid JSON, not the first valid JSON
    let mut longest_match: Option<(usize, Value)> = None;

    for start_char_idx in (0..char_count).rev() {
        // Convert character index back to byte index for slicing
        let start_byte_idx = chars
            .iter()
            .take(start_char_idx)
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let json_candidate = &line[start_byte_idx..];

        if let Ok(json_value) = serde_json::from_str::<Value>(json_candidate) {
            // Keep track of the longest valid JSON found
            longest_match = Some((start_byte_idx, json_value));
        }
    }

    if let Some((start_byte_idx, json_value)) = longest_match {
        let text_part = line[..start_byte_idx].trim_end();
        return Ok((text_part.to_string(), json_value));
    }

    Err(ParseError::NoValidJson)
}

/// Parse multiple lines, each containing text followed by JSON
///
/// Returns a Vec of Results, where each Result corresponds to one non-empty line.
/// Empty lines are filtered out before processing.
pub fn parse_lines(input: &str) -> Vec<Result<(String, Value), ParseError>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_text_json)
        .collect()
}

/// Parse multiple lines with line number context for better error reporting
///
/// Returns a Vec of Results with line numbers, where each Result corresponds to one line.
/// Empty lines are included in the line count but filtered from processing.
pub fn parse_lines_with_line_numbers(
    input: &str,
) -> Vec<Result<(usize, String, Value), ParseErrorWithContext>> {
    input
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let line_number = line_num + 1; // Convert to 1-based indexing
            if line.trim().is_empty() {
                return Err(ParseErrorWithContext::EmptyLine { line_number });
            }
            match parse_text_json(line) {
                Ok((text, json)) => Ok((line_number, text, json)),
                Err(ParseError::EmptyLine) => Err(ParseErrorWithContext::EmptyLine { line_number }),
                Err(ParseError::NoValidJson) => Err(ParseErrorWithContext::NoValidJson {
                    line_number,
                    line_content: line.to_string(),
                }),
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyLine,
    NoValidJson,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "Empty line cannot be parsed"),
            ParseError::NoValidJson => write!(f, "No valid JSON found in line"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, PartialEq)]
pub enum ParseErrorWithContext {
    EmptyLine {
        line_number: usize,
    },
    NoValidJson {
        line_number: usize,
        line_content: String,
    },
}

impl std::fmt::Display for ParseErrorWithContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorWithContext::EmptyLine { line_number } => {
                write!(f, "Empty line cannot be parsed at line {}", line_number)
            }
            ParseErrorWithContext::NoValidJson {
                line_number,
                line_content,
            } => {
                write!(
                    f,
                    "No valid JSON found at line {}: '{}'",
                    line_number, line_content
                )
            }
        }
    }
}

impl std::error::Error for ParseErrorWithContext {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_simple_text_json() {
        let result = parse_text_json(r#"This line is the text {"is_json": true}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "This line is the text");
        assert_eq!(json, json!({"is_json": true}));
    }

    #[test]
    fn parse_no_text_only_json() {
        let result = parse_text_json(r#"{"only": "json"}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "");
        assert_eq!(json, json!({"only": "json"}));
    }

    #[test]
    fn parse_complex_json() {
        let result = parse_text_json(
            r#"Event occurred {"timestamp": 1234567890, "data": {"nested": true, "values": [1, 2, 3]}}"#,
        );
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Event occurred");
        assert_eq!(
            json,
            json!({"timestamp": 1234567890, "data": {"nested": true, "values": [1, 2, 3]}})
        );
    }

    #[test]
    fn parse_json_with_spaces_in_text() {
        let result = parse_text_json(r#"Multiple words in text part {"value": 42}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Multiple words in text part");
        assert_eq!(json, json!({"value": 42}));
    }

    #[test]
    fn parse_no_valid_json() {
        let result = parse_text_json("This line has no JSON");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::NoValidJson);
    }

    #[test]
    fn parse_empty_line() {
        let result = parse_text_json("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::EmptyLine);
    }

    #[test]
    fn parse_multiple_lines() {
        let input = r#"Line one {"a": 1}
Line two {"b": 2}
Line three {"c": 3}"#;

        let results = parse_lines(input);
        assert_eq!(results.len(), 3);

        let (text1, json1) = results[0].as_ref().unwrap();
        assert_eq!(text1, "Line one");
        assert_eq!(*json1, json!({"a": 1}));

        let (text2, json2) = results[1].as_ref().unwrap();
        assert_eq!(text2, "Line two");
        assert_eq!(*json2, json!({"b": 2}));

        let (text3, json3) = results[2].as_ref().unwrap();
        assert_eq!(text3, "Line three");
        assert_eq!(*json3, json!({"c": 3}));
    }

    #[test]
    fn parse_json_array() {
        let result = parse_text_json(r#"Array data [1, 2, 3, {"nested": true}]"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Array data");
        assert_eq!(json, json!([1, 2, 3, {"nested": true}]));
    }

    #[test]
    fn parse_json_string() {
        let result = parse_text_json(r#"Simple string "hello world""#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Simple string");
        assert_eq!(json, json!("hello world"));
    }

    #[test]
    fn parse_unicode_text() {
        let result = parse_text_json(r#"Unicode text 你好世界 {"greeting": "hello"}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Unicode text 你好世界");
        assert_eq!(json, json!({"greeting": "hello"}));
    }

    #[test]
    fn parse_emoji_in_text() {
        let result = parse_text_json(r#"Status update 🎉🚀 {"status": "success"}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Status update 🎉🚀");
        assert_eq!(json, json!({"status": "success"}));
    }

    #[test]
    fn parse_whitespace_only_line() {
        let result = parse_text_json("   \t  ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::EmptyLine);
    }

    #[test]
    fn parse_json_with_escaped_quotes() {
        let result = parse_text_json(r#"Message received {"text": "She said \"hello\" to me"}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Message received");
        assert_eq!(json, json!({"text": "She said \"hello\" to me"}));
    }

    #[test]
    fn parse_text_containing_json_like_syntax() {
        // This tests that the parser correctly identifies the actual JSON at the end
        // and doesn't get confused by JSON-like syntax in the text portion
        let result = parse_text_json(r#"Found config {old: value} in cache {"new": "value"}"#);
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Found config {old: value} in cache");
        assert_eq!(json, json!({"new": "value"}));
    }

    #[test]
    fn parse_json_number() {
        let result = parse_text_json("Temperature reading 42.5");
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Temperature reading");
        assert_eq!(json, json!(42.5));
    }

    #[test]
    fn parse_json_boolean() {
        let result = parse_text_json("Operation succeeded true");
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "Operation succeeded");
        assert_eq!(json, json!(true));
    }

    #[test]
    fn parse_json_null() {
        let result = parse_text_json("No data available null");
        assert!(result.is_ok());
        let (text, json) = result.unwrap();
        assert_eq!(text, "No data available");
        assert_eq!(json, json!(null));
    }

    #[test]
    fn parse_malformed_json() {
        let result = parse_text_json(r#"Bad JSON {"incomplete": }"#);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParseError::NoValidJson);
    }

    #[test]
    fn parse_multiple_lines_with_errors() {
        let input = r#"Good line {"valid": true}
Bad line with no JSON
Another good line {"also": "valid"}"#;

        let results = parse_lines(input);
        assert_eq!(results.len(), 3);

        // First line should succeed
        assert!(results[0].is_ok());
        let (text1, json1) = results[0].as_ref().unwrap();
        assert_eq!(text1, "Good line");
        assert_eq!(*json1, json!({"valid": true}));

        // Second line should fail
        assert!(results[1].is_err());
        assert_eq!(results[1].as_ref().unwrap_err(), &ParseError::NoValidJson);

        // Third line should succeed
        assert!(results[2].is_ok());
        let (text3, json3) = results[2].as_ref().unwrap();
        assert_eq!(text3, "Another good line");
        assert_eq!(*json3, json!({"also": "valid"}));
    }

    #[test]
    fn parse_lines_with_context() {
        let input = r#"Good line {"valid": true}

Bad line with no JSON
Another good line {"also": "valid"}"#;

        let results = parse_lines_with_line_numbers(input);
        assert_eq!(results.len(), 4); // Including the empty line

        // First line should succeed
        assert!(results[0].is_ok());
        let (line_num1, text1, json1) = results[0].as_ref().unwrap();
        assert_eq!(*line_num1, 1);
        assert_eq!(text1, "Good line");
        assert_eq!(*json1, json!({"valid": true}));

        // Second line (empty) should fail with context
        assert!(results[1].is_err());
        assert_eq!(
            results[1].as_ref().unwrap_err(),
            &ParseErrorWithContext::EmptyLine { line_number: 2 }
        );

        // Third line should fail with context
        assert!(results[2].is_err());
        assert_eq!(
            results[2].as_ref().unwrap_err(),
            &ParseErrorWithContext::NoValidJson {
                line_number: 3,
                line_content: "Bad line with no JSON".to_string()
            }
        );

        // Fourth line should succeed
        assert!(results[3].is_ok());
        let (line_num4, text4, json4) = results[3].as_ref().unwrap();
        assert_eq!(*line_num4, 4);
        assert_eq!(text4, "Another good line");
        assert_eq!(*json4, json!({"also": "valid"}));
    }
}
