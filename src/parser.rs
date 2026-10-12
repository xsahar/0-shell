pub fn parse_command(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
                escaped = true;
            }
            '\'' | '"' => {
                if quote == Some(ch) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(ch);
                } else {
                    current.push(ch);
                }
            }
            ' ' | '\t' | '\n' | '\r' if quote.is_none() => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }

    if escaped {
        current.push('\\');
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::parse_command;

    #[test]
    fn parses_basic_tokens() {
        assert_eq!(
            parse_command("echo hello world"),
            vec!["echo", "hello", "world"]
        );
    }

    #[test]
    fn parses_quoted_text() {
        assert_eq!(
            parse_command("echo \"hello world\""),
            vec!["echo", "hello world"]
        );
    }

    #[test]
    fn parses_escaped_spaces() {
        assert_eq!(
            parse_command(r#"echo hello\ world"#),
            vec!["echo", "hello world"]
        );
    }
}
