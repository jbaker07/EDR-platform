// Utilities for safely normalizing C-style byte arrays into Rust strings.
// These helpers trim trailing NUL/whitespace and guard tag values against
// non-ASCII characters so downstream sinks see stable identifiers.

pub fn cbytes_to_string_lossy(mut bytes: &[u8]) -> String {
    if let Some(pos) = bytes.iter().position(|&b| b == 0) {
        bytes = &bytes[..pos];
    }

    let owned = bytes.to_vec();
    let mut s = String::from_utf8_lossy(&owned).into_owned();
    while s.ends_with('\u{0}') || s.ends_with(' ') || s.ends_with('\t') {
        s.pop();
    }
    s
}

pub fn sanitize_tag_value(s: &str) -> String {
    if s.is_ascii() {
        return s.to_string();
    }

    s.chars()
        .map(|c| {
            if c.is_ascii() && !c.is_control() {
                c.to_string()
            } else {
                format!("\\x{:02x}", (c as u32) & 0xff)
            }
        })
        .collect::<String>()
}
