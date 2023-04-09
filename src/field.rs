use nom::branch::alt;
use nom::bytes::complete::{escaped, take_until};
use nom::character::complete::{alphanumeric1, char as this_char, multispace0};
use nom::sequence::{delimited, Tuple};
use nom::IResult;
use parse_hyperlinks::take_until_unbalanced;

fn braced_content(input: &str) -> IResult<&str, &str> {
    let (remainder, content) = delimited(
        this_char('{'),
        take_until_unbalanced('{', '}'),
        this_char('}'),
    )(input)?;
    Ok((remainder, content))
}

fn field_name(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn equals(input: &str) -> IResult<&str, char> {
    this_char('=')(input)
}

fn quote(input: &str) -> IResult<&str, char> {
    this_char('"')(input)
}

fn escaped_field_value(input: &str) -> IResult<&str, &str> {
    escaped(take_until("\""), '\\', this_char('"'))(input)
}

fn quoted_field_value(input: &str) -> IResult<&str, &str> {
    let (remainder, (_, field_value, _)) = (quote, escaped_field_value, quote).parse(input)?;
    Ok((remainder, field_value))
}

fn unbounded_field_value(input: &str) -> IResult<&str, &str> {
    take_until(",")(input)
}

fn field_value(input: &str) -> IResult<&str, &str> {
    alt((quoted_field_value, braced_content, unbounded_field_value))(input)
}

pub(crate) fn field(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (name, _, _, _, value)) =
        (field_name, multispace0, equals, multispace0, field_value).parse(input)?;
    Ok((input, (name, value)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn field_name_test() {
        let test_str = r#"author="henry""#;
        assert_eq!(field_name(test_str), Ok((r#"="henry""#, "author")));
    }

    #[test]
    fn equals_test() {
        let test_str = r#"="henry""#;
        assert_eq!(equals(test_str), Ok((r#""henry""#, '=')));
    }

    #[test]
    fn quote_test() {
        let test_str = r#""henry""#;
        assert_eq!(quote(test_str), Ok((r#"henry""#, '"')));
    }

    #[test]
    fn braced_field_value_test() {
        let test_str = "{henry}";
        assert_eq!(braced_content(test_str), Ok(("", "henry")));
    }

    #[test]
    fn double_braced_field_value_test() {
        let test_str = "{{henry}}";
        assert_eq!(braced_content(test_str), Ok(("", "{henry}")));
    }

    #[test]
    fn field_value_test() {
        let test_str_braced = "{henry}";
        let test_str_quoted = "\"henry\"";
        assert_eq!(field_value(test_str_braced), Ok(("", "henry")));
        assert_eq!(field_value(test_str_quoted), Ok(("", "henry")));
    }

    #[test]
    fn field_test() {
        let test_str_braced = "author={henry}";
        let test_str_quoted = "author=\"henry\"";

        assert_eq!(field(test_str_braced), Ok(("", ("author", "henry"))));
        assert_eq!(field(test_str_quoted), Ok(("", ("author", "henry"))));
    }

    #[test]
    fn real_field_tests() {
        let test_str = "title = {122-Channel Squid Instrument for Investigating the Magnetic Signals from the Human Brain},";
        assert_eq!(field(test_str), Ok((
            ",", ("title", "122-Channel Squid Instrument for Investigating the Magnetic Signals from the Human Brain")
            )))
    }
}
