use crate::comment::comment_block;
use nom::character::complete::multispace0;
use nom::multi::separated_list0;
use nom::IResult;

use crate::entry::{entry, Entry};

mod comment;
mod entry;
mod field;
mod header;

pub fn file(input: &str) -> IResult<&str, Vec<Entry>> {
    let (input, content) = separated_list0(multispace0, entry)(input)?;
    let (remainder, _) = comment_block(input)?;
    Ok((remainder, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_dot_bib() {
        let test_str = std::fs::read_to_string("tests/test.bib").unwrap();
        let (remainder, _entries) = file(&test_str).unwrap();
        assert_eq!(remainder, "");
    }

    #[test]
    fn test_bib_dot_bib() {
        let test_str = std::fs::read_to_string("tests/bib.bib").unwrap();
        let (remainder, _entries) = file(&test_str).unwrap();
        assert_eq!(remainder, "");
    }
}
