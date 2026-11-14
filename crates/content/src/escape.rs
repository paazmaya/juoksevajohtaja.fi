//! Escaping helpers for HTML/XML attributes, text and JSON strings.

/// Escapes text for safe use inside HTML/XML text nodes and double-quoted attributes.
pub fn xml(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Encodes a JSON string literal, including the surrounding quotes.
///
/// `<` is escaped as well so the JSON can be embedded in a `<script>` element safely.
pub fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_xml() {
        assert_eq!(
            xml(r#"a & <b> "c" 'd'"#),
            "a &amp; &lt;b&gt; &quot;c&quot; &#39;d&#39;"
        );
        assert_eq!(xml("Hyvä päivä"), "Hyvä päivä");
    }

    #[test]
    fn escapes_json() {
        assert_eq!(json_string("plain"), "\"plain\"");
        assert_eq!(
            json_string("q\"b\\n\nr\rt\t</script>\u{1}"),
            "\"q\\\"b\\\\n\\nr\\rt\\t\\u003c/script>\\u0001\""
        );
    }
}
