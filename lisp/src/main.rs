use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{map, map_res},
    sequence::tuple,
    IResult,
};

enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

// Atoms
enum Atom {
    Number(i32),
    Operator(Operator),
}

fn parse_atom(input: &str) -> IResult<&str, Atom> {
    map(
        alt((
            map(tag("+"), |_| Operator::Plus),
            map(tag("-"), |_| Operator::Minus),
            map(tag("*"), |_| Operator::Times),
            map(tag("/"), |_| Operator::Divide),
        )),
        |operator| Atom::Operator(operator),
    )(input)
}

fn main() {
}
