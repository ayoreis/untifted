mod physics;
pub mod state_machine;

use super::plane;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((plane::plugin, state_machine::plugin, physics::plugin));
}
