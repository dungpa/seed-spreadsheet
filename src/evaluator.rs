use crate::parser::*;
use crate::*;

use std::collections::*;

fn evaluate(visited: &mut HashSet<Pos>, cells: &HashMap<Pos, String>, expr: Expr) -> Option<i32> { 
  match expr {
    Expr::Number(num) => Some(num),
    Expr::Binary(e1, op, e2) => {
        match evaluate(visited, cells, *e1) {
            Some(val1) => {
                match evaluate(visited, cells, *e2) {
                    Some(val2) => {
                        match op {
                            '+' => Some(val1 + val2),
                            '-' => Some(val1 - val2),
                            '*' => Some(val1 * val2),
                            '/' => Some(val1 / val2),
                            _ => None 
                        }
                    },
                    None => None
                }
            },
            None => None
        }

    }
    Expr::Reference(pos) => {
        if visited.contains(&pos) {
            None
        } else {
            match cells.get(&pos) {
                Some(value) => {
                    match parser::run(value) {
                        Some(parsed) => {
                            visited.insert(pos);
                            evaluate(visited, cells, parsed)
                        },
                        None => None
                    }
                },
                None => None
            }
        }
    }
  }
}

pub fn run (value: &str, cells: &HashMap<Pos, String>) -> Option<i32> {
    let val = parser::run(value)?;
    let mut visited = HashSet::new();
    evaluate(&mut visited, cells, val)
}