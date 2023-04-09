use nom::branch::alt;
use nom::character::complete::{char as this_char, multispace1, not_line_ending};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

fn comment(input: &str) -> IResult<&str, &str> {
    let (remainder, (_, content)) = pair(this_char('%'), not_line_ending)(input)?;
    Ok((remainder, content))
}

fn comment_or_blank(input: &str) -> IResult<&str, &str> {
    alt((comment, multispace1))(input)
}

pub(crate) fn comment_block(input: &str) -> IResult<&str, Vec<&str>> {
    many0(comment_or_blank)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comment() {
        let test_str = "% comment content\n";
        assert_eq!(comment(test_str), Ok(("\n", " comment content")));
    }

    #[test]
    fn test_comment_or_blank() {
        let test_str_comment = "% comment content\n";
        let test_str_blank = "\n\n\t  \t";

        assert_eq!(
            comment_or_blank(test_str_comment),
            Ok(("\n", " comment content"))
        );
        assert_eq!(comment_or_blank(test_str_blank), Ok(("", "\n\n\t  \t")));
    }

    #[test]
    fn test_block_comment() {
        let test_str = "% comment 1\n\t% comment 2\n\n  % comment 3\n \t something after";
        assert_eq!(
            comment_block(test_str),
            Ok((
                "something after",
                vec![
                    " comment 1",
                    "\n\t",
                    " comment 2",
                    "\n\n  ",
                    " comment 3",
                    "\n \t "
                ]
            ))
        );
    }

    #[test]
    fn test_multiple_block_comment() {
        let test_str = "%%% first line\n%%% second line\n %%% third line\n\n\n%%% fourth line\n";
        assert_eq!(
            comment_block(test_str),
            Ok((
                "",
                vec![
                    "%% first line",
                    "\n",
                    "%% second line",
                    "\n ",
                    "%% third line",
                    "\n\n\n",
                    "%% fourth line",
                    "\n",
                ]
            ))
        );
    }
}
