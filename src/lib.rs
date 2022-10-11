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
    fn get_headers(model: &Model) -> Vec<Node<Msg>> {
        let empty = td! [];
        let mut headers: Vec<Node<Msg>> = model.cols.iter().map(|h| { th! [h.to_string()] }).collect();
        headers.insert(0, empty);
        headers
    }

    fn get_rows(model: &Model) -> Vec<Node<Msg>> {
        fn render_cell(h: &char, n: &i32) -> Node<Msg> {
            td! ["A"]
        }

        fn get_cells(model: &Model, n: &i32) -> Vec<Node<Msg>> {
            let mut cells: Vec<Node<Msg>> = model.cols.iter().map(|h| { render_cell(h, n) }).collect();
            cells.insert(0, th! [n.to_string()]);
            cells
        }

        model.rows.iter().map (|r| { tr! [get_cells(&model, r)] }).collect()
    }

    table! [
        tr!(get_headers(model)),
        tbody!(get_rows(model)),
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
