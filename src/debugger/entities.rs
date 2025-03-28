//! Adds labels and axes to entities with a [`Name`] and [`Transform`].

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, axes.run_if(in_state(super::State::Enabled)));
}

fn axes(mut gizmos: Gizmos, entities: Query<&GlobalTransform, With<Name>>) {
    for transform in &entities {
        gizmos.axes(*transform, 3.0);
    }
}
