// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod types;
mod parser;
mod evaluator;

use std::collections::HashMap;

use seed::{prelude::*, *};
use crate::types::*;

// ------ ------
//     Init
// ------ ------

static ROW_COUNT: i32 = 16;

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { 
        rows: (1..17).collect(), 
        cols: ('A'..'M').collect(), 
        active: None, 
        cells: HashMap::new() 
    }
}

// ------ ------
//    Update
// ------ ------

static ENTER_KEY_CODE: u32 = 12;

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartEdit(pos) => { model.active = Some(pos); },
        Msg::KeyDown(key_code) => {
            if key_code == ENTER_KEY_CODE {
                match model.active {
                    Some((col, row)) => {
                        let new_pos = (col, (row + 1) % ROW_COUNT); 
                        model.active = Some(new_pos);
                    },
                    None => ()
                } 
            }
        },
        Msg::UpdateValue(pos, value) => { model.cells.insert(pos, value); },
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
            let pos = (*col, *row);
            let value = model.cells.get(&pos);
            if model.active == Some(pos) {
                td! [
                    C! ["selected"],
                    input! [
                        attrs! {
                            At::AutoFocus => true,
                            At::Value => value.unwrap_or(&"".to_owned());
                        },
                        keyboard_ev(Ev::KeyDown, |ev| Msg::KeyDown(ev.key_code())),
                        input_ev(Ev::Input, move |v| Msg::UpdateValue(pos, v)),
                    ]
                ]
            } else {
                let val =
                    match value {
                        Some(val) => {
                            evaluator::run(val, &model.cells).map(|v| v.to_string())
                        },
                        None => Some("".to_owned()),
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
                    val.unwrap_or_else(|| "#ERR".to_owned()),
                    ev(Ev::Click, move |_| Msg::StartEdit(pos)),
                ]
            }
            
        }

        fn create_cells(model: &Model, row: &i32) -> Vec<Node<Msg>> {
            let mut cells: Vec<Node<Msg>> = model.cols.iter().map(|c| { render_cell(model, c, row) }).collect();
            cells.insert(0, th! [row.to_string()]);
            cells
        }

        model.rows.iter().map (|r| { tr! [create_cells(model, r)] }).collect()
    }

    div! [
        h4! [
            "Double click on a cell for editing.",
        ],
        table! [
            tr!(create_headers(model)),
            tbody!(create_rows(model)),
        ]
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
