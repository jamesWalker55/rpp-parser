use itertools::Itertools;

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
    let needs_to_be_quoted = (first_char == '\'' || first_char == '"') || text.contains(' ');
    if !needs_to_be_quoted {
        return text;
    }

    let quote_char = {
        let has_dbl = text.contains('"');
        let has_sgl = text.contains('\'');

        if has_dbl && has_sgl {
            '`'
        } else if has_dbl {
            '\''
        } else {
            '"'
        }
    };

    format!("{}{}{}", quote_char, text, quote_char).into()
}

pub fn serialize_to_string(element: &Element) -> String {
    serialize_to_lines(element).join("\n")
}

fn serialize_to_lines(element: &Element) -> Vec<String> {
    let first_line = if element.attr.is_empty() {
        format!("<{}", element.tag)
    } else {
        let attr = element.attr.iter().map(|x| serialise_term(x)).join(" ");
        format!("<{} {}", element.tag, attr)
    };

    let mut lines = vec![first_line];

    for child in element.children.iter() {
        match child {
            Child::Line(child) => {
                let line = child.iter().map(|x| serialise_term(x)).join(" ");
                lines.push(format!("  {line}"));
            }
            Child::Element(child) => {
                let sublines = serialize_to_lines(&child);
                lines.extend(sublines.iter().map(|x| format!("  {x}")));
            }
        }
    }

    lines.push(">".into());

    lines
}
