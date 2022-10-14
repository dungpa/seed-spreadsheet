use nom::IResult;
use nom::character::complete::{alpha1, digit1, space0};
use nom::combinator::map;
use nom::sequence::{delimited, pair};

type Pos = (char, i32);

enum Expr {
  Number(i32),
  Reference(Pos),
  Binary(Box<Expr>, char, Box<Expr>)
}

fn parse_num(number: &str) -> Expr {
    let num = number.parse::<i32>().unwrap();
    Expr::Number(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_num)(input)
}

fn parse_ref((col, row): (&str, &str)) -> Expr {
    let c = col.chars().nth(0).unwrap();
    let r = row.parse::<i32>().unwrap();
    Expr::Reference((c, r))
}

fn parse_reference(input: &str) -> IResult<&str, Expr> {
    map(pair(alpha1, digit1), parse_ref)(input)
}