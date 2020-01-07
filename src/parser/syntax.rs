use lazy_static;
use regex::Regex;

use super::{
    ast::{OpCode, Expr},
    combinator::{concat, alter, map, wrap, all},
    simple::{literal, wrap_space0},
    parser::{Parser, ParseResult},
};

pub fn opcode(expected: &'static str, op: OpCode)
    -> impl Fn(&str) -> Result<(&str, OpCode), &str>
{
    let l = expected.len();

    move |input| match input.get(0..l) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], op))
        }
        _ => Err(input),
    }
}

pub fn integer(input: &str) -> ParseResult<Expr> {
    lazy_static!{
        static ref INT_REGEX: Regex = Regex::new(r"^[0-9]+").unwrap();
    }
    match INT_REGEX.find(input) {
        Some(mat) => {
            Ok((&input[mat.end()..], Expr::Int(mat.as_str().parse().unwrap())))
        },
        None => Err(input)
    }
}

pub fn atom<'a>() -> impl Parser<'a, Expr> {
    alternate!(
        integer,
        wrap(literal("("), wrap_space0(additive), literal(")"))
    )
}

// basic arithematics
macro_rules! make_infix {
    ($name: ident, $prec: expr, $ops: expr) => {
        pub fn $name(input: &str) -> ParseResult<Expr> {
            let p = map(concat(concat($prec, wrap_space0($ops)), $name), |((left, op), right)| {
                Expr::BinOp(Box::new(left), op, Box::new(right))
            });
            alter(p, $prec).parse(input)
        }
    };
}

make_infix!(multiplicative, atom(), alternate!(
    opcode("*", OpCode::Mul),
    opcode("/", OpCode::Div)
));

make_infix!(additive, multiplicative,  alternate!(
    opcode("+", OpCode::Add),
    opcode("-", OpCode::Sub)
));

// testing
pub fn parse(file: &str) -> Expr {
    let (_, r) = all(additive).parse(file).unwrap();
    r
}