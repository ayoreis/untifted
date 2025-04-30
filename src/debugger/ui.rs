use super::window::DebuggerWindow;
use crate::systems::despawn;
use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};

#[derive(Component)]
struct UiCamera;

#[derive(Component)]
pub struct Root;

pub fn plugin(app: &mut App) {
	app.add_observer(spawn).add_systems(
		OnExit(super::State::Enabled),
		despawn::<Or<(With<UiCamera>, With<Root>)>>,
	);
}

fn spawn(trigger: Trigger<OnAdd, DebuggerWindow>, mut commands: Commands) {
	let camera = commands
		.spawn((
			UiCamera,
			Camera2d::default(),
			Camera {
				target: RenderTarget::Window(WindowRef::Entity(trigger.target())),
				clear_color: ClearColorConfig::None,
				order: 1,
				..default()
			},
		))
		.id();

	commands.spawn((
		Root,
		Node {
			width: Val::Percent(100.0),
			height: Val::Percent(100.0),
			flex_direction: FlexDirection::Column,
			..default()
		},
		UiTargetCamera(camera),
	));
}
