use nom::{
    branch::alt,
    bytes::complete::{take_till, take_till1},
    character::complete::{char, line_ending, multispace0, none_of, one_of, space0, space1},
    combinator::all_consuming,
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, Parser,
};

type Input<'a> = &'a str;

type Result<'a, O = Input<'a>> = nom::IResult<Input<'a>, O>;

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

fn string(i: Input) -> Result {
    alt((unquoted_string, quoted_string))(i)
}

fn string_list(i: Input) -> Result<Vec<&str>> {
    // get first element
    let (i, first_element) = string(i)?;

    // get the rest
    let (i, mut other_elements) = many0(preceded(space1, string))(i)?;

    // prepend first element into list
    // TODO: inserting is inefficient, think of way to improve this
    other_elements.insert(0, first_element);

    Ok((i, other_elements))
}

fn element_start(i: Input) -> Result<()> {
    char('<')(i).map(|(rest, _)| (rest, ()))
}

fn element_end(i: Input) -> Result<()> {
    char('>')(i).map(|(rest, _)| (rest, ()))
}

fn element_tag(i: Input) -> Result {
    unquoted_string(i)
}

fn element(i: Input) -> Result<Element> {
    // line starts with '<TAG'
    let (i, _) = element_start(i)?;
    let (i, tag) = element_tag(i)?;

    // rest of line is attr
    // (consumes newline)
    let (i, attr) = terminated(
        many0(preceded(space1, string)),
        tuple((space0, line_ending)),
    )(i)?;

    let (i, children) = many0(delimited(
        space0,
        alt((
            element.map(|x| Child::Element(x)),
            string_list.map(|x| Child::Line(x)),
        )),
        tuple((space0, line_ending)),
    ))(i)?;

    // element ends with a single line containing only '>'
    // but this function isn't responsible for checking the newline
    let (i, _) = tuple((space0, element_end))(i)?;

    let element = Element {
        tag,
        attr,
        children,
    };

    Ok((i, element))
}

pub fn parse_element(i: Input) -> std::result::Result<Element, nom::error::Error<Input>> {
    all_consuming(delimited(multispace0, element, multispace0))(i)
        .finish()
        .map(|(_, element)| element)
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
