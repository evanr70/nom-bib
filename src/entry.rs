use crate::comment::comment_block;
use crate::field::field;
use crate::header::{header, Header};
use nom::character::complete::char as this_char;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::Tuple;
use nom::IResult;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Entry {
    header: Header,
    fields: HashMap<String, String>,
}

impl Entry {
    #[must_use]
    pub fn new(header: Header, fields: &[(&str, &str)]) -> Self {
        Self {
            header,
            fields: fields
                .iter()
                .map(|field| (field.0.to_string(), field.1.to_string()))
                .collect(),
        }
    }
}

fn comma_line_end(input: &str) -> IResult<&str, ()> {
    let (remainder, _) = (this_char(','), comment_block).parse(input)?;
    Ok((remainder, ()))
}

fn close_brace(input: &str) -> IResult<&str, char> {
    this_char('}')(input)
}

pub fn entry(input: &str) -> IResult<&str, Entry> {
    let (remainder, (_, header_, _, fields, _, _, _, _)) = (
        comment_block,
        header,
        comment_block,
        separated_list1(comma_line_end, field),
        comment_block,
        opt(this_char(',')),
        comment_block,
        close_brace,
    )
        .parse(input)?;
    Ok((remainder, Entry::new(header_, &fields)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_close_brace() {
        let test_str = "}following content";
        assert_eq!(close_brace(test_str), Ok(("following content", '}')));
    }

    #[test]
    fn test_entry() {
        let test_str =
            "@book{jacobs-1942,\nauthor=\"jacobs, isaac\",\njournal=\"Chemistry Interests\"\n}";
        assert_eq!(
            entry(test_str),
            Ok((
                "",
                Entry::new(
                    Header::new("book", "jacobs-1942"),
                    &vec![
                        ("author", "jacobs, isaac"),
                        ("journal", "Chemistry Interests")
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_single_real_entry() {
        let test_str = r#"@Techreport{bs-1629,
  author =       "BSI",
  title =        "Bibliographic References",
  institution =  "British Standards Institution",
  year =         "1976",
  type =         "BS",
  number =       "1629",
}"#;

        let target = Entry::new(
            Header::new("Techreport", "bs-1629"),
            &vec![
                ("author", "BSI"),
                ("title", "Bibliographic References"),
                ("institution", "British Standards Institution"),
                ("year", "1976"),
                ("type", "BS"),
                ("number", "1629"),
            ],
        );

        assert_eq!(entry(test_str), Ok(("", target)));
    }
}
