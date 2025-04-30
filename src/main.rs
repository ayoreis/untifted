#![feature(let_chains, stmt_expr_attributes, try_blocks)]

mod debugger;
mod game;
mod systems;

use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins((game::plugin, debugger::plugin))
		.run();
}
