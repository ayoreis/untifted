pub mod block;
pub mod camera;
pub mod cube;
mod game_loader;
pub mod level_loader;
pub mod loading;
mod physics;
pub mod plane;
pub mod player;
mod playing;
mod splash;

use bevy::{prelude::*, window::WindowResolution};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
	#[default]
	Splash,
	Loading,
	Playing,
}

const TILE_SIZE: u32 = 8;
const SCALE: u32 = 7;
const SCALED_TILE_SIZE: u32 = TILE_SIZE * SCALE;
const TILES_X: u32 = 16;
const TILES_Y: u32 = 16;

pub fn plugin(app: &mut App) {
	let window_plugin = WindowPlugin {
		primary_window: Some(Window {
			title: "Untifted".into(),
			resolution: WindowResolution::new(
				(SCALED_TILE_SIZE * TILES_X) as f32,
				(SCALED_TILE_SIZE * TILES_Y) as f32,
			),
			resizable: false,
			..default()
		}),
		..default()
	};

	let image_plugin = ImagePlugin::default_nearest();

	app.add_plugins(DefaultPlugins.set(window_plugin).set(image_plugin))
		.init_state::<State>()
		.insert_resource(ClearColor(Color::BLACK))
		.add_plugins((splash::plugin, loading::plugin, playing::plugin));
}
