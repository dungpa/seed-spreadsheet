use nom::IResult;
use nom::character::complete::{alpha1, digit1, space0, char};
use nom::combinator::map;
use nom::sequence::{delimited, pair, tuple};
use nom::branch::alt;

type Pos = (char, i32);

enum Expr {
  Number(i32),
  Reference(Pos),
  Binary(Box<Expr>, char, Box<Expr>)
}

fn create_number(number: &str) -> Expr {
    let num = number.parse::<i32>().unwrap();
    Expr::Number(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), create_number)(input)
}

fn create_reference((col, row): (&str, &str)) -> Expr {
    let c = col.chars().nth(0).unwrap();
    let r = row.parse::<i32>().unwrap();
    Expr::Reference((c, r))
}

fn parse_reference(input: &str) -> IResult<&str, Expr> {
    map(pair(alpha1, digit1), create_reference)(input)
}

fn parse_bracket(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), delimited(space0, parse_expr, space0), char(')'))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    alt((alt((parse_number, parse_reference)), parse_bracket))(input)
}

fn create_binary(((expr1, operator), expr2): ((Expr, char), Expr)) -> Expr {
    Expr::Binary(Box::new(expr1), operator, Box::new(expr2))
}

fn parse_operator(input: &str) -> IResult<&str, char> {
    let add_subtract = alt((char('+'), char('-')));
    let mult_divide = alt((char('*'), char('/')));
    alt((add_subtract, mult_divide))(input)
}

fn parse_binary(input: &str) -> IResult<&str, Expr> {
    map(pair(pair(parse_term, delimited(space0, parse_operator, space0)), parse_term), create_binary)(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_binary, parse_term))(input)
}
