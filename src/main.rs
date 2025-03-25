#![feature(let_chains, stmt_expr_attributes)]

mod debugger;
mod game;
mod systems;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(game::plugin);

    if cfg!(debug_assertions) {
        app.add_plugins(debugger::plugin);
    }

    app.run();
}
