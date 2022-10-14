use crate::evaluator::Expr::*;

use nom::IResult;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::sequence::{delimited, tuple};

type Pos = (char, i32);

enum Expr {
  Reference(Pos),
  Number(i32),
  Binary(Box<Expr>, char, Box<Expr>)
}

fn parse_num(n: &str) -> Expr {
    let num = n.parse::<i32>().unwrap();
    Number(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_num)(input)
}