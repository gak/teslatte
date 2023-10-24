use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, line_ending, space1};
use nom::multi::many0;
use nom::IResult;
use tracing::trace;

/// Ignore the quotes and return the inner string.
/// e.g. "unlock"
pub fn quoted_string(s: &str) -> IResult<&str, &str> {
    short_trace("quoted string", s);
    let (s, _) = char('"')(s)?;
    let (s, string) = take_until("\"")(s)?;
    let (s, _) = char('"')(s)?;
    Ok((s, string))
}

pub fn ignore_whitespace(s: &str) -> IResult<&str, ()> {
    short_trace("ignore whitespace", s);
    let (s, ws) = many0(alt((tag("\t"), space1, line_ending)))(s)?;
    short_trace("ignore whitespace afterwards", s);
    Ok((s, ()))
}

pub fn until_eol(s: &str) -> IResult<&str, &str> {
    short_trace("eol", s);
    let (s, line) = take_until("\n")(s)?;
    let (s, _) = line_ending(s)?;
    short_trace("eol afterwards", s);
    Ok((s, line))
}

pub fn short_trace(prefix: &str, s: &str) {
    let mut max_len_left = 20;
    if s.len() < max_len_left {
        max_len_left = s.len();
    }
    trace!("{}: {:?}...", prefix, &s[0..max_len_left])
}
