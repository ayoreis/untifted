mod loading;
mod paused;
mod playing;

use super::AppState;
use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug, SubStates, Default)]
#[source(AppState = AppState::Game)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
}

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>().add_plugins((
        loading::plugin,
        playing::plugin,
        paused::plugin,
    ));
}
