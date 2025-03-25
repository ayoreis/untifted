use super::{physics, plane, player};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((physics::plugin, plane::plugin, player::plugin));
}
