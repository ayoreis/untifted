use crate::systems::despawn;
use bevy::prelude::*;

#[derive(Component)]
#[require(Window {
	title: "Debugger".into(),
	..default()
})]
pub struct DebuggerWindow;

pub fn plugin(app: &mut App) {
	app.add_systems(OnEnter(super::State::Enabled), spawn)
		.add_systems(
			OnExit(super::State::Enabled),
			despawn::<With<DebuggerWindow>>,
		);
}

fn spawn(mut commands: Commands) {
	commands.spawn(DebuggerWindow);
}
