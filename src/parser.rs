use crate::types::*;

use nom::character::complete::{alpha1, digit1, space0, char};
use nom::combinator::map;
use nom::sequence::{delimited, pair, preceded};
use nom::branch::alt;
use nom::IResult;
use nom::Err::Error;
use nom::error::ParseError;
use nom::error::ErrorKind;

#[derive(PartialEq)]
enum CustomError<I> {
  Conversion(String),
  Nom(I, ErrorKind),
}

impl<I> ParseError<I> for CustomError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    CustomError::Nom(input, kind)
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

type CResult<I, T> = IResult<I, T, CustomError<I>>;

fn create_number(number: &str) -> CResult<&str, Expr> {
    match number.parse::<i32>() {
        Ok(num) => Ok((number, Expr::Number(num))),
        Err(_) => Err(Error(CustomError::Conversion(number.to_owned())))
    }
}

fn parse_number(input: &str) -> CResult<&str, Expr> {
    delimited(space0, digit1, space0)(input).and_then(|(_, o)| create_number(o))
}

fn create_reference<'a>(input: &'a str, (col, row): (&str, &str)) -> CResult<&'a str, Expr> {
    match col.chars().next() {
        Some(c) => {
            match row.parse::<i32>() {
                Ok(r) => Ok((input, Expr::Reference((c, r)))),
                Err(_) => Err(Error(CustomError::Conversion(row.to_owned())))
            }
        },
        None => Err(Error(CustomError::Conversion(col.to_owned())))
    }
}

fn parse_reference(input: &str) -> CResult<&str, Expr> {
    pair(alpha1, digit1)(input).and_then(|(i, o)| create_reference(i, o))
}

fn parse_bracket(input: &str) -> CResult<&str, Expr> {
    delimited(char('('), delimited(space0, parse_expr, space0), char(')'))(input)
}

fn parse_term(input: &str) -> CResult<&str, Expr> {
    alt((parse_number, parse_reference, parse_bracket))(input)
}

fn create_binary(((expr1, operator), expr2): ((Expr, char), Expr)) -> Expr {
    Expr::Binary(Box::new(expr1), operator, Box::new(expr2))
}

fn parse_operator(input: &str) -> CResult<&str, char> {
    alt((char('+'), char('-'), char('*'), char('/')))(input)
}

fn parse_binary(input: &str) -> CResult<&str, Expr> {
    map(pair(pair(parse_term, delimited(space0, parse_operator, space0)), parse_term), create_binary)(input)
}

fn parse_expr(input: &str) -> CResult<&str, Expr> {
    alt((parse_binary, parse_term))(input)
}

// Formula starts with `=` followed by expression
fn parse_formula(input: &str) -> CResult<&str, Expr> {
    preceded(char('='), preceded(space0, parse_expr))(input)
}

// Equation you can write in a cell is either number or a formula
fn parse_equation(input: &str) -> CResult<&str, Expr> {
    delimited(space0, alt((parse_formula, parse_number)), space0)(input)
}

// Run the parser on a given input
pub fn run(input: &str) -> Option<Expr> {
    match parse_equation(input) {
         Ok((_, expr)) => Some(expr),
         Err(_) => None
    }
}
