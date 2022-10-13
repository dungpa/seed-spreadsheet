use std::collections::HashMap;

// ------ ------
//     Model
// ------ ------

type Position = (char, i32);

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
    UpdateValue(Position, String)
}
