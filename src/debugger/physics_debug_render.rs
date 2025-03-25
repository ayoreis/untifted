use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(RapierDebugRenderPlugin::default().disabled())
        .add_systems(OnEnter(super::State::Enabled), enable)
        .add_systems(OnExit(super::State::Enabled), disable);
}

fn enable(mut context: ResMut<DebugRenderContext>) {
    context.enabled = true;
}

fn disable(mut context: ResMut<DebugRenderContext>) {
    context.enabled = false;
}
