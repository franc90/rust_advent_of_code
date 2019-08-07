use std::ops::Add;

pub fn compute_lengths(lines: &String) -> (usize, usize) {
    let mut original_len = 0;
    let mut encoded_len = 0;

    for line in lines.lines() {
        original_len += line.len();
        encoded_len += encode(line).len();
    }

    (encoded_len, original_len)
}

fn encode(line: &str) -> String {
    let line = encode_first(line);
    let line = encode_last(line);
    let line = encode_backslashes(line);
    let line = encode_quotes(line);
    let line = encode_hexes(line);
    line
}

fn encode_first(line: &str) -> String {
    if line.starts_with("\"") {
        line.replacen("\"", "www", 1)
    } else {
        line.to_string()
    }
}

fn encode_last(mut line: String) -> String {
    if line.ends_with("\"") {
        line.pop();
        line.add("www")
    } else {
        line
    }
}

fn encode_backslashes(mut line: String) -> String {
    line.replace("\\\\", "wwww")
}

fn encode_quotes(mut line: String) -> String {
    line.replace("\\\"", "wwww")
}

fn encode_hexes(mut line: String) -> String {
    let mut line = line;
    while let Some(loc) = line.find("\\x") {
        let mut chars = &mut line[loc + 2..].chars();
        if is_hex(chars.next()) && is_hex(chars.next()) {
            line.replace_range(loc..loc + 4, "wwwww")
        } else {
            line.replace_range(loc..loc + 2, "ww")
        }
    }
    line
}

fn is_hex(char: Option<char>) -> bool {
    match char {
        Some(c) => c.is_ascii_hexdigit(),
        None => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_empty() {
        assert_eq!(encode("\"\"").len(), 6)
    }

    #[test]
    fn encode_abc() {
        assert_eq!(encode("\"abc\"").len(), 9)
    }

    #[test]
    fn encode_quote() {
        assert_eq!(encode("\"aaa\\\"aaa\"").len(), 16)
    }

    #[test]
    fn encode_backslash() {
        assert_eq!(encode("\"aa\\\\aa\"").len(), 14)
    }

    #[test]
    fn encode_hex() {
        assert_eq!(encode("\"\\x27\"").len(), 11)
    }

    #[test]
    fn test_compute_lengths() {
        let text = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";

        let (encoded_len, original_len) = compute_lengths(&text.to_string());
        assert_eq!(encoded_len, 42);
        assert_eq!(original_len, 23);
    }
}