use bevy::prelude::*;

#[derive(Component)]
pub struct DebuggerWindow;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(super::State::Enabled), spawn_window);
}

fn spawn_window(mut commands: Commands) {
    commands.spawn((
        DebuggerWindow,
        Window {
            title: "Debugger for Untifted".into(),
            ..default()
        },
    ));
}
