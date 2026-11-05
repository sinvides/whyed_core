/// Parsed user input.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedInput {
    /// A slash command with arguments.
    Command { name: String, args: Vec<String> },
    /// Plain user message (may contain @file references).
    Message { text: String, file_refs: Vec<String> },
}

/// Extract @file references from a message.
/// Returns (clean_text, list_of_referenced_files).
pub fn extract_file_refs(text: &str) -> (String, Vec<String>) {
    let mut refs = Vec::new();
    let mut clean = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '@' {
            // Collect the filename reference
            let mut name = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() || c == '.' || c == '/' || c == '\\' || c == '_' || c == '-' {
                    name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !name.is_empty() {
                refs.push(name.clone());
                clean.push_str(&format!("@{}", name));
            } else {
                clean.push('@');
            }
        } else {
            clean.push(ch);
        }
    }

    (clean, refs)
}

/// Parse raw input into either a command or a message.
pub fn parse(raw: &str) -> ParsedInput {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return ParsedInput::Message {
            text: String::new(),
            file_refs: Vec::new(),
        };
    }

    if !trimmed.starts_with('/') {
        let (text, refs) = extract_file_refs(trimmed);
        return ParsedInput::Message { text, file_refs: refs };
    }

    // Slash command
    let mut parts = trimmed.split_whitespace();
    let cmd = parts.next().unwrap_or("").trim_start_matches('/').to_lowercase();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    ParsedInput::Command { name: cmd, args }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_refs() {
        let (text, refs) = extract_file_refs("Read @src/main.rs and @src/app/config.rs");
        assert_eq!(text, "Read @src/main.rs and @src/app/config.rs");
        assert_eq!(refs, vec!["src/main.rs".to_string(), "src/app/config.rs".to_string()]);
    }

    #[test]
    fn test_parse_command() {
        let input = parse("/read src/main.rs");
        match input {
            ParsedInput::Command { name, args } => {
                assert_eq!(name, "read");
                assert_eq!(args, vec!["src/main.rs".to_string()]);
            }
            _ => panic!("expected command"),
        }
    }

    #[test]
    fn test_parse_message() {
        let input = parse("hello world");
        match input {
            ParsedInput::Message { text, .. } => {
                assert_eq!(text, "hello world");
            }
            _ => panic!("expected message"),
        }
    }
}
