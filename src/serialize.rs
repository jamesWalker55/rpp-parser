use crate::parser::{Child, Element};
use std::borrow::Cow;

fn serialise_term(text: &str) -> Cow<str> {
    if text.is_empty() {
        return "\"\"".into();
    }

    let mut text: Cow<str> = text.into();

    // backticks are not allowed in .rpp files, they are always replaced with single quotes
    if text.contains('`') {
        text = text.replace('`', "'").into();
    }

    let first_char = text.chars().next().unwrap();
    let mut needs_to_be_quoted = first_char == '\'' || first_char == '"';

    let mut has_dbl_quote = false;
    let mut has_sgl_quote = false;
    for x in text.chars() {
        if x == ' ' {
            needs_to_be_quoted = true;
        } else if x == '\'' {
            has_sgl_quote = true;
        } else if x == '"' {
            has_dbl_quote = true;
        }
    }

    if !needs_to_be_quoted {
        return text;
    }

    let quote_char = {
        if has_dbl_quote && has_sgl_quote {
            '`'
        } else if has_dbl_quote {
            '\''
        } else {
            '"'
        }
    };

    format!("{}{}{}", quote_char, text, quote_char).into()
}

/// Serialise an element back to a [String] following the RPP format.
pub fn serialize_to_string(element: &Element) -> String {
    let mut buf = String::new();
    process(&mut buf, element, 0);
    buf
}

fn process(buf: &mut String, element: &Element, indent_level: usize) {
    // first line
    for _ in 0..indent_level {
        buf.push_str("  ")
    }
    buf.push('<');
    buf.push_str(element.tag);

    for x in &element.attr {
        let x = serialise_term(x);
        buf.push(' ');
        buf.push_str(&x);
    }

    buf.push('\n');

    for child in element.children.iter() {
        match child {
            Child::Line(child) => {
                for _ in 0..(indent_level + 1) {
                    buf.push_str("  ")
                }

                let mut is_first = true;
                for x in child {
                    let x = serialise_term(x);
                    if is_first {
                        is_first = false;
                    } else {
                        buf.push(' ')
                    }
                    buf.push_str(&x);
                }

                buf.push('\n');
            }
            Child::Element(child) => {
                process(buf, child, indent_level + 1);
                buf.push('\n');
            }
        }
    }

    // last line
    for _ in 0..indent_level {
        buf.push_str("  ")
    }
    buf.push('>');
}
