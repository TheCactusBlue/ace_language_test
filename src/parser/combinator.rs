use super::parser::Parser;

pub fn concat<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| parser1.parse(input).and_then(|(next_input, result1)| {
        parser2.parse(next_input)
            .map(|(last_input, result2)| (last_input, (result1, result2)))
    })
}

pub fn alter<'a, P, P2, R>(parser1: P, parser2: P2) -> impl Parser<'a, R>
where
    P: Parser<'a, R>,
    P2: Parser<'a, R>
{
    move |input| {
        parser1.parse(input).or_else(|_e| {
            parser2.parse(input)
        })
    }
}

pub fn opt<'a, P, R>(parser: P) -> impl Parser<'a, Option<R>>
where
    P: Parser<'a, R>
{
    move |input| {
        parser.parse(input)
            .map(|(next_input, r)| (next_input, Some(r)))
            .or_else(|_e| Ok((input, None)))
    }
}

// zero or more
pub fn many<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }
        Ok((input, result))
    }
}

// one or more
pub fn some<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

// Takes parser that returns A and makes parser that returns B
pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| parser.parse(input)
        .map(|(next_input, result)| (next_input, map_fn(result)))
}

pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(concat(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(concat(parser1, parser2), |(_left, right)| right)
}

pub fn wrap<'a, P, R, LP, RP, D1, D2>(lp: LP, parser: P, rp: RP) -> impl Parser<'a, R>
where
    LP: Parser<'a, D1>,
    P: Parser<'a, R>,
    RP: Parser<'a, D2>
{
    left(right(lp, parser), rp)
}

pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

pub fn all<'a, P, R>(parser: P) -> impl Parser<'a, R>
where
    P: Parser<'a, R>,
{
    move |input| {
        parser.parse(input).and_then(|(next_input, value)| {
            if next_input != "" {
                Err(next_input)
            } else {
                Ok((next_input, value))
            }
        })
    }
}