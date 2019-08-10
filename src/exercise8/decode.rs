pub fn compute_lengths(lines: &str) -> (usize, usize) {
    let mut code_len = 0;
    let mut mem_len = 0;

    for line in lines.lines() {
        code_len += line.len();
        mem_len += sanitize(line).len();
    }

    (code_len, mem_len)
}

fn sanitize(line: &str) -> String {
    let line = drop_first(line);
    let line = drop_last(line);
    let line = drop_backslashes(line);
    let line = drop_quotes(line);
    let line = drop_hexes(line);
    line
}

fn drop_first(line: &str) -> &str {
    if line.starts_with('\"') {
        &line[1..]
    } else {
        line
    }
}

fn drop_last(line: &str) -> &str {
    if line.ends_with('\"') {
        let len = line.len() - 1;
        &line[..len]
    } else {
        line
    }
}

fn drop_backslashes(line: &str) -> String {
    line.replace("\\\\", "w")
}

fn drop_quotes(line: String) -> String {
    line.replace("\\\"", "w")
}

fn drop_hexes(line: String) -> String {
    let mut line = line;
    while let Some(loc) = line.find("\\x") {
        let chars = &mut line[loc + 2..].chars();
        if is_hex(chars.next()) && is_hex(chars.next()) {
            line.replace_range(loc..loc + 4, "w")
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
    fn sanitize_empty() {
        assert_eq!(sanitize("\"\"").len(), 0)
    }

    #[test]
    fn sanitize_abc() {
        assert_eq!(sanitize("\"abc\"").len(), 3)
    }

    #[test]
    fn sanitize_quote() {
        assert_eq!(sanitize("\"aaa\\\"aaa\"").len(), 7)
    }

    #[test]
    fn sanitize_backslash() {
        assert_eq!(sanitize("\"aa\\\\aa\"").len(), 5)
    }

    #[test]
    fn sanitize_hex() {
        assert_eq!(sanitize("\"\\x27\"").len(), 1)
    }

    #[test]
    fn test_compute_lengths() {
        let text = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"";

        let (code_len, mem_len) = compute_lengths(&text.to_string());
        assert_eq!(code_len, 23);
        assert_eq!(mem_len, 11);
    }
}