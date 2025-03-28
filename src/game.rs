pub mod block;
pub mod camera;
mod game_loader;
mod level_loader;
pub mod loading;
mod physics;
mod plane;
mod player;
mod playing;

use bevy::prelude::*;
use bevy::window::WindowResolution;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    Loading,
    Playing,
}

const SCALE: u32 = 5;
const WIDTH: u32 = 180;
const HEIGHT: u32 = 180;
const SCALED_WIDTH: f32 = (WIDTH * SCALE) as f32;
const SCALED_HEIGHT: f32 = (HEIGHT * SCALE) as f32;

pub fn plugin(app: &mut App) {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Untifted".into(),
            resolution: WindowResolution::new(SCALED_WIDTH, SCALED_HEIGHT),
            resizable: false,
            ..default()
        }),
        ..default()
    };

    let image_plugin = ImagePlugin::default_nearest();

    app.add_plugins(DefaultPlugins.set(window_plugin).set(image_plugin))
        .init_state::<State>()
        .add_plugins((loading::plugin, playing::plugin));
}
