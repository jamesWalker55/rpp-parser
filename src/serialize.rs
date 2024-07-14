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
    process(element, 0).iter().map(|x| x.as_ref()).collect()
}

fn process<'a>(element: &'a Element, indent_level: usize) -> Vec<Cow<'a, str>> {
    let mut parts: Vec<Cow<'a, str>> = vec![];

    // first line
    parts.extend(iter::repeat(Cow::from("  ")).take(indent_level));
    parts.push("<".into());
    parts.push(element.tag.into());
    parts.extend(
        element
            .attr
            .iter()
            .flat_map(|x| [" ".into(), serialise_term(x)]),
    );
    parts.push("\n".into());

    for child in element.children.iter() {
        match child {
            Child::Line(child) => {
                parts.extend(iter::repeat(Cow::from("  ")).take(indent_level + 1));
                parts.extend(
                    child
                        .iter()
                        .flat_map(|x| [" ".into(), serialise_term(x)])
                        .skip(1),
                );
                parts.push("\n".into());
            }
            Child::Element(child) => {
                parts.extend(process(&child, indent_level + 1));
                parts.push("\n".into());
            }
        }
    }

    // last line
    parts.extend(iter::repeat(Cow::from("  ")).take(indent_level));
    parts.push(">".into());

    parts
}
