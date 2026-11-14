//! Enforces the design system: all colors, spacing, typography, sizes, motion and layers live in
//! `style/_tokens.scss`; component and page stylesheets may only reference them.

use std::fs;
use std::path::{Path, PathBuf};

/// Component directories without visual output, which therefore have no stylesheet.
const NON_VISUAL: &[&str] = &["page_head"];

/// Units that must come from tokens.
const FORBIDDEN_UNITS: &[&str] = &["px", "rem", "em", "pt", "ms", "s"];

fn site_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn ui_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    for group in ["src/components", "src/pages"] {
        for entry in fs::read_dir(site_dir().join(group)).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
    }
    dirs.sort();
    dirs
}

fn stylesheet(dir: &Path) -> PathBuf {
    let name = dir.file_name().unwrap().to_str().unwrap();
    dir.join(format!("{name}.scss"))
}

/// Strips `//` and `/* */` comments.
fn strip_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("/*") {
            rest = after.split_once("*/").map(|(_, tail)| tail).unwrap_or("");
        } else if rest.starts_with("//") {
            rest = rest.split_once('\n').map(|(_, tail)| tail).unwrap_or("");
            out.push('\n');
        } else {
            let ch = rest.chars().next().unwrap();
            out.push(ch);
            rest = &rest[ch.len_utf8()..];
        }
    }
    out
}

/// Literal values that should have been tokens, e.g. `12px`, `#fff`, `200ms`, `rgb(`.
fn violations(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (index, line) in strip_comments(source).lines().enumerate() {
        let line_no = index + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("@use") {
            continue;
        }
        let lower = trimmed.to_lowercase();
        for function in ["rgb(", "rgba(", "hsl(", "hsla(", "color-mix("] {
            if lower.contains(function) {
                found.push(format!("line {line_no}: color function in '{trimmed}'"));
            }
        }
        for property in [
            "font-family:",
            "z-index:",
            "color:",
            "background:",
            "font-size:",
        ] {
            if let Some(value) = lower.strip_prefix(property) {
                let value = value.trim().trim_end_matches(';');
                // Multi-line functions (gradients) are checked line by line by the literal rules below.
                let allowed = value.starts_with("var(")
                    || value.ends_with('(')
                    || ["inherit", "transparent", "currentcolor", "none", "initial"]
                        .contains(&value);
                if !allowed {
                    found.push(format!("line {line_no}: '{trimmed}' must use a token"));
                }
            }
        }
        let bytes = trimmed.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let ch = bytes[i] as char;
            if ch == '#'
                && bytes.get(i + 1).is_some_and(|b| b.is_ascii_hexdigit())
                && !lower[..i].ends_with('&')
            {
                let hex: String = trimmed[i + 1..]
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .collect();
                if [3, 4, 6, 8].contains(&hex.len()) && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    found.push(format!("line {line_no}: hex color #{hex}"));
                }
            }
            let starts_number = ch.is_ascii_digit()
                && (i == 0
                    || !matches!(bytes[i - 1] as char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '$' | '.'));
            if starts_number {
                let mut end = i;
                while end < bytes.len() && (bytes[end].is_ascii_digit() || bytes[end] == b'.') {
                    end += 1;
                }
                let unit: String = trimmed[end..]
                    .chars()
                    .take_while(|c| c.is_ascii_alphabetic())
                    .collect();
                let number = &trimmed[i..end];
                if FORBIDDEN_UNITS.contains(&unit.as_str())
                    && number.parse::<f64>().unwrap_or(0.0) != 0.0
                {
                    found.push(format!(
                        "line {line_no}: literal length/duration {number}{unit}"
                    ));
                }
                i = end.max(i + 1);
                continue;
            }
            i += 1;
        }
    }
    found
}

#[test]
fn every_ui_directory_has_its_own_stylesheet() {
    for dir in ui_dirs() {
        let name = dir.file_name().unwrap().to_str().unwrap();
        if NON_VISUAL.contains(&name) {
            continue;
        }
        assert!(
            stylesheet(&dir).is_file(),
            "{} has no {name}.scss",
            dir.display()
        );
    }
}

#[test]
fn main_stylesheet_uses_every_ui_stylesheet() {
    let main = fs::read_to_string(site_dir().join("style/main.scss")).unwrap();
    for dir in ui_dirs() {
        let sheet = stylesheet(&dir);
        if !sheet.is_file() {
            continue;
        }
        let group = dir.parent().unwrap().file_name().unwrap().to_str().unwrap();
        let name = dir.file_name().unwrap().to_str().unwrap();
        let expected = format!("@use \"../src/{group}/{name}/{name}\";");
        assert!(
            main.contains(&expected),
            "style/main.scss is missing {expected}"
        );
    }
}

#[test]
fn ui_stylesheets_only_use_design_tokens() {
    let mut problems = Vec::new();
    for dir in ui_dirs() {
        let sheet = stylesheet(&dir);
        if let Ok(source) = fs::read_to_string(&sheet) {
            for violation in violations(&source) {
                problems.push(format!("{}: {violation}", sheet.display()));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "design token violations:\n{}",
        problems.join("\n")
    );
}

#[test]
fn main_stylesheet_only_uses_design_tokens() {
    let main = fs::read_to_string(site_dir().join("style/main.scss")).unwrap();
    assert_eq!(violations(&main), Vec::<String>::new());
}

#[test]
fn detects_violations() {
    let source = "// 12px comment is fine\n.a {\n  padding: 12px 0;\n  color: #fff;\n  margin: var(--space-1);\n  \
                  transition: opacity 200ms;\n  background: rgb(0 0 0);\n  font-family: Arial;\n  z-index: 3;\n  \
                  flex: 0 0 auto;\n  width: 50%;\n  /* 3rem */\n  &:hover { color: inherit; }\n  gap: 0rem;\n}";
    let found = violations(source);
    assert_eq!(found.len(), 8, "{found:#?}");
    assert!(found.iter().any(|v| v.contains("12px")));
    assert!(found.iter().any(|v| v.contains("#fff")));
    assert!(found.iter().any(|v| v.contains("200ms")));
    assert!(found.iter().any(|v| v.contains("rgb(")));
    assert!(found.iter().any(|v| v.contains("font-family")));
    assert!(found.iter().any(|v| v.contains("z-index")));
}

#[test]
fn tokens_file_defines_every_referenced_variable() {
    let tokens = fs::read_to_string(site_dir().join("style/_tokens.scss")).unwrap();
    let mut sheets: Vec<PathBuf> = ui_dirs()
        .iter()
        .map(|d| stylesheet(d))
        .filter(|p| p.is_file())
        .collect();
    sheets.push(site_dir().join("style/main.scss"));
    for sheet in sheets {
        let source = fs::read_to_string(&sheet).unwrap();
        for part in source.split("var(--").skip(1) {
            let name: String = part
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
                .collect();
            assert!(
                tokens.contains(&format!("--{name}:")),
                "{} uses undefined token --{name}",
                sheet.display()
            );
        }
    }
}
