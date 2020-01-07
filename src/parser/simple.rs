use regex::Regex;
use super::{
    combinator::{pred, many, some, map, concat, wrap, alter},
    parser::{Parser, ParseResult},
};

pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

pub fn comment(input: &str) -> ParseResult<()> {
    lazy_static!{
        static ref COMMENT_REGEX: Regex = Regex::new(r"^/\*[\S\s]*?\*/").unwrap();
    }
    match COMMENT_REGEX.find(input) {
        Some(mat) => {
            Ok((&input[mat.end()..], ()))
        },
        None => Err(input)
    }
}

pub fn ws_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    some(ws_char())
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    many(ws_char())
}

pub fn wrap_space0<'a, P, R>(parser: P) -> impl Parser<'a, R>
where
    P: Parser<'a, R>
{
    wrap(space0(), parser, space0())
}

pub fn literal(expected: &'static str)
    -> impl Fn(&str) -> Result<(&str, ()), &str>
{
    let l = expected.len();

    move |input| match input.get(0..l) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
    }
}
