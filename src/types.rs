use std::collections::HashMap;

// ------ ------
//     Model
// ------ ------

pub type Position = (char, i32);

pub enum Expr {
  Number(i32),
  Reference(Position),
  Binary(Box<Expr>, char, Box<Expr>)
}

// `Model` describes our app state.
pub struct Model {
    pub rows: Vec<i32>,
    pub cols: Vec<char>,
    pub active: Option<Position>,
    pub cells: HashMap<Position, String>,
}

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    StartEdit(Position),
    KeyDown(u32),
    UpdateValue(Position, String)
}
