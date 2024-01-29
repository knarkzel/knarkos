use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::map,
    error,
    multi::many0,
    sequence::{delimited, tuple},
    Parser,
};
use std::fmt;

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
pub enum Operator {
    Plus,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
        }
    }
}

// Atoms
#[derive(Debug)]
pub enum Atom {
    Number(usize),
    Operator(Operator),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::Operator(operator) => write!(f, "{operator}"),
        }
    }
}

fn parse_operator(input: &str) -> IResult<Atom> {
    map(
        alt((
            map(tag("+"), |_| Operator::Plus),
        )),
        |operator| Atom::Operator(operator),
    )(input)
}

fn parse_number(input: &str) -> IResult<Atom> {
    map(digit1, |digits: &str| {
        Atom::Number(digits.parse::<usize>().unwrap())
    })(input)
}

fn parse_atom(input: &str) -> IResult<Atom> {
    alt((parse_number, parse_operator))(input)
}

// Expressions
#[derive(Debug)]
pub enum Expr {
    Constant(Atom),
    /// (fn arg1 arg2 arg3 ...)
    Call(Box<Expr>, Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constant(atom) => write!(f, "{atom}"),
            Self::Call(head, tail) => {
                write!(f, "({head}")?;
                for expr in tail {
                    write!(f, " {expr}")?;
                }
                write!(f, ")")
            }
        }
    }
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

pub fn parse_expr(input: &str) -> IResult<Expr> {
    ws(alt((parse_constant, parse_call)))(input)
}
