use crate::systems::despawn_recursive;
use bevy::prelude::*;

#[derive(Component)]
pub struct DebuggerWindow;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(super::State::Enabled), spawn)
        .add_systems(
            OnExit(super::State::Enabled),
            despawn_recursive::<With<DebuggerWindow>>,
        );
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        DebuggerWindow,
        Window {
            title: "Debugger for Untifted".into(),
            ..default()
        },
    ));
}
