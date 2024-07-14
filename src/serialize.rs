use crate::parser::{Child, Element};
use std::{borrow::Cow, iter};

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
    let mut buf = String::new();
    process(&mut buf, element, 0);
    buf
}

fn process<'a>(mut buf: &mut String, element: &'a Element, indent_level: usize) {
    // first line
    buf.extend(iter::repeat(Cow::from("  ")).take(indent_level));
    buf.push('<');
    buf.push_str(element.tag);
    buf.extend(
        element
            .attr
            .iter()
            .flat_map(|x| [" ".into(), serialise_term(x)]),
    );
    buf.push('\n');

    for child in element.children.iter() {
        match child {
            Child::Line(child) => {
                buf.extend(iter::repeat(Cow::from("  ")).take(indent_level + 1));
                buf.extend(
                    child
                        .iter()
                        .flat_map(|x| [" ".into(), serialise_term(x)])
                        .skip(1),
                );
                buf.push('\n');
            }
            Child::Element(child) => {
                process(&mut buf, &child, indent_level + 1);
                buf.push('\n');
            }
        }
    }

    // last line
    buf.extend(iter::repeat(Cow::from("  ")).take(indent_level));
    buf.push('>');
}
