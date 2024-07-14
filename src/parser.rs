use nom::{
    bytes::complete::{take_till, take_till1, take_while1},
    character::{
        complete::{char, none_of, one_of},
        is_alphabetic,
    },
    sequence::delimited,
};
use thiserror::Error;

type Input<'a> = &'a str;

type Result<'a, O = Input<'a>> = nom::IResult<Input<'a>, O, ParseError<'a>>;

#[derive(Error, Debug)]
pub enum ParseError<'a> {
    #[error("invalid string literal: {0}")]
    MalformedString(Input<'a>),
    #[error("invalid path: {0}")]
    MalformedPath(Input<'a>),
    #[error("only relative paths are allowed: {0}")]
    NotRelativePath(Input<'a>),
    #[error("invalid glob pattern: {0}")]
    InvalidGlobPattern(Input<'a>),
    #[error("unknown directive: {0}")]
    UnknownDirective(Input<'a>),
    #[error("hash char is not walter code: {0}")]
    NonWALTERHash(Input<'a>),
    #[error("incorrect #include syntax: {0}")]
    MalformedIncludeDirective(Input<'a>),
    #[error("incorrect #resource syntax: {0}")]
    MalformedResourceDirective(Input<'a>),
    #[error("invalid syntax: {0}")]
    Nom(Input<'a>, nom::error::ErrorKind),
}

impl<'a> nom::error::ParseError<Input<'a>> for ParseError<'a> {
    fn from_error_kind(input: Input<'a>, kind: nom::error::ErrorKind) -> Self {
        ParseError::Nom(input.into(), kind)
    }

    fn append(_input: Input, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub struct Element<'a> {
    tag: &'a str,
    attr: Vec<&'a str>,
    children: Vec<Child<'a>>,
}

pub enum Child<'a> {
    Line(Vec<&'a str>),
    Element(Element<'a>),
}

fn quoted_string(i: Input) -> Result {
    let (i, quote_char) = one_of("\"'`")(i)?;
    let (i, contents) = take_till(|x| x == quote_char || x == '\n' || x == '\r')(i)?;
    let (i, _) = char(quote_char)(i)?;

    Ok((i, contents))
}

fn unquoted_string(i: Input) -> Result {
    // first character must not be quote
    none_of("\"'`")(i)?;
    // now take characters until we reach space / end of line
    take_till1(|x| x == ' ' || x == '\n' || x == '\r')(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod quoted_string {
        use super::*;

        #[test]
        fn test_01() {
            let input = "'apple'";
            let expected = "apple";
            let (_, result) = quoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_02() {
            let input = "'app\"le'";
            let expected = "app\"le";
            let (_, result) = quoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_03() {
            let input = "\"asd asjhd basjh \"";
            let expected = "asd asjhd basjh ";
            let (_, result) = quoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_04() {
            let input = "`asd asjhd basjh `";
            let expected = "asd asjhd basjh ";
            let (_, result) = quoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_05() {
            let input = "`asd asjhd basjh ` 'hello'";
            let expected = "asd asjhd basjh ";
            let (_, result) = quoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_06() {
            let input = "`asd asjhd b\nasjh ` 'hello'";
            assert!(quoted_string(input).is_err())
        }
    }

    #[cfg(test)]
    mod unquoted_string {
        use super::*;

        #[test]
        fn test_01() {
            let input = "apple";
            let expected = "apple";
            let (_, result) = unquoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_02() {
            let input = "hasquote''``' askjdla";
            let expected = "hasquote''``'";
            let (_, result) = unquoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_03() {
            let input = "has space";
            let expected = "has";
            let (_, result) = unquoted_string(input).unwrap();
            assert_eq!(result, expected)
        }

        #[test]
        fn test_04() {
            let input = "  leading space";
            assert!(unquoted_string(input).is_err())
        }

        #[test]
        fn test_05() {
            let input = "'hello'";
            assert!(unquoted_string(input).is_err())
        }
    }
}
