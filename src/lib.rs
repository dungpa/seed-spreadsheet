// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::collections::HashMap;

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        rows: (1..16).collect(), 
        cols: ('A'..'L').collect(), 
        active: None, 
        cells: HashMap::new() 
    }
}

// ------ ------
//     Model
// ------ ------

type Position = (char, i32);

// `Model` describes our app state.
struct Model {
    rows: Vec<i32>,
    cols: Vec<char>,
    active: Option<Position>,
    cells: HashMap<Position, String>,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => (),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    fn create_headers(model: &Model) -> Vec<Node<Msg>> {
        let empty = td! [];
        let mut headers: Vec<Node<Msg>> = model.cols.iter().map(|c| { th! [c.to_string()] }).collect();
        headers.insert(0, empty);
        headers
    }

    fn create_rows(model: &Model) -> Vec<Node<Msg>> {
        fn render_cell(model: &Model, col: &char, row: &i32) -> Node<Msg> {
            let value = model.cells.get(&(*col, *row));
            let val =
                match value {
                    Some(val) => Some(val.clone()),
                    None => Some("B".to_owned()),
                };
            td! [
                if val.is_none() { 
                    style! {
                        St::Background => "#ffb0b0"
                    }
                } else {
                    style! {
                        St::Background => "white"
                    }
                },
                val.unwrap_or("#ERR".to_owned()),
            ]
        }

        fn create_cells(model: &Model, row: &i32) -> Vec<Node<Msg>> {
            let mut cells: Vec<Node<Msg>> = model.cols.iter().map(|c| { render_cell(model, c, row) }).collect();
            cells.insert(0, th! [row.to_string()]);
            cells
        }

        model.rows.iter().map (|r| { tr! [create_cells(&model, r)] }).collect()
    }

    table! [
        tr!(create_headers(model)),
        tbody!(create_rows(model)),
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
