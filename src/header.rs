use crate::comment::comment_block;
use nom::bytes::complete::take_until1;
use nom::character::complete::char as this_char;
use nom::sequence::Tuple;
use nom::IResult;

#[derive(PartialEq, Debug)]
pub struct Header {
    entry_type: String,
    cite_key: String,
}

impl Header {
    pub fn new(entry_type: &str, cite_key: &str) -> Self {
        Self {
            entry_type: entry_type.to_string(),
            cite_key: cite_key.to_string(),
        }
    }
}

fn at(input: &str) -> IResult<&str, char> {
    this_char('@')(input)
}

fn entry_type(input: &str) -> IResult<&str, &str> {
    take_until1("{")(input)
}

fn open_brace(input: &str) -> IResult<&str, char> {
    this_char('{')(input)
}

fn cite_key(input: &str) -> IResult<&str, &str> {
    take_until1(",")(input)
}

fn comma(input: &str) -> IResult<&str, char> {
    this_char(',')(input)
}

pub fn header(input: &str) -> IResult<&str, Header> {
    let (remainder, (_, entry_type_, _, cite_key_, _, _)) =
        (at, entry_type, open_brace, cite_key, comma, comment_block).parse(input)?;
    Ok((remainder, Header::new(entry_type_, cite_key_)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_at() {
        let test_str = "@book{hobbs-2013,";
        assert_eq!(at(test_str), Ok(("book{hobbs-2013,", '@')));
    }

    #[test]
    fn test_entry_type() {
        let test_str = "book{hobbs-2013,";
        assert_eq!(entry_type(test_str), Ok(("{hobbs-2013,", "book")));
    }

    #[test]
    fn test_open_brace() {
        let test_str = "{hobbs-2013,";
        assert_eq!(open_brace(test_str), Ok(("hobbs-2013,", '{')));
    }

    #[test]
    fn test_cite_key() {
        let test_str = "hobbs-2013,";
        assert_eq!(cite_key(test_str), Ok((",", "hobbs-2013")));
    }

    #[test]
    fn test_comma() {
        let test_str = ",";
        assert_eq!(comma(test_str), Ok(("", ',')));
    }

    #[test]
    fn test_header() {
        let test_str = "@book{hobbs-2013,";
        assert_eq!(
            header(test_str),
            Ok(("", Header::new("book", "hobbs-2013")))
        );
    }
}
