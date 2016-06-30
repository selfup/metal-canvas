#![feature(custom_derive)]

#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, Module};
use neon::js::JsObject;

mod game;
mod grid;
mod bike;
mod check;

use game::*;
use grid::*;
use bike::*;

fn game(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;

    let game = Game::new(Grid::new(800, 0, 800, 0),
                         Bike::new(0, 400),
                         Bike::new(800, 400));

    let js_obj = JsObject::new(&mut game);

    Ok(js_obj)
}

register_module!(m, {
    m.export("game", game)
});
