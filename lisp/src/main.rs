use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, char},
    combinator::{map, map_res, opt},
    sequence::{tuple, delimited, preceded}, Parser, error, multi::many0,
};

// Helpers
type IResult<'a, T, E = error::Error<&'a str>> = nom::IResult<&'a str, T, E>;

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<O>
where
    F: FnMut(&'a str) -> IResult<O>,
{
    delimited(multispace0, inner, multispace0)
}

fn sexp<'a, O1, F>(inner: F) -> impl FnMut(&'a str) -> IResult<O1>
where
    F: Parser<&'a str, O1, error::Error<&'a str>>,
{
    delimited(
        char('('),
        delimited(multispace0, inner, multispace0),
        char(')'),
    )
}

// Operator
#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

// Atoms
#[derive(Debug)]
enum Atom {
    Number(i32),
    Operator(Operator),
}

fn parse_operator(input: &str) -> IResult<Atom> {
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

fn parse_number(input: &str) -> IResult<Atom> {
    map(digit1, |digits: &str| {
        Atom::Number(digits.parse::<i32>().unwrap())
    })(input)
}

fn parse_atom(input: &str) -> IResult<Atom> {
    alt((
        parse_number,
        parse_operator,
    ))(input)
}

// Expressions
#[derive(Debug)]
enum Expr {
    Constant(Atom),
    /// (fn arg1 arg2 arg3 ...)
    Call(Box<Expr>, Vec<Expr>),
}

fn parse_constant(input: &str) -> IResult<Expr> {
    map(parse_atom, |atom| Expr::Constant(atom))(input)
}

fn parse_call(input: &str) -> IResult<Expr> {
    let inner = map(tuple((parse_expr, many0(parse_expr))), |(head, tail)| {
        Expr::Call(Box::new(head), tail)
    });
    sexp(inner)(input)
}

fn parse_expr(input: &str) -> IResult<Expr> {
    ws(alt((
        parse_constant,
        parse_call,
    )))(input)
}

fn main() {
    dbg!(parse_expr("(+ 1 2 3)"));
}
