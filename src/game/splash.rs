use crate::systems::despawn;
use bevy::prelude::*;

#[derive(Component)]
struct UiCamera;

#[derive(Component)]
struct UiRoot;

pub fn plugin(app: &mut App) {
	app.add_systems(OnEnter(super::State::Splash), spawn)
		.add_systems(
			OnExit(super::State::Splash),
			despawn::<Or<(With<UiCamera>, With<UiRoot>)>>,
		)
		.add_systems(Update, play);
}

fn spawn(mut commands: Commands) {
	commands.spawn((
		UiCamera,
		Camera2d::default(),
		Camera {
			clear_color: ClearColorConfig::None,
			order: 1,
			..default()
		},
	));

	commands
		.spawn((
			UiRoot,
			Node {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..default()
			},
			BackgroundColor(Color::BLACK),
			children![Text::new("Untifted")],
		))
		.observe(
			|_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<super::State>>| {
				next_state.set(super::State::Loading);
			},
		);
}

fn play(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<super::State>>) {
	if keyboard.just_pressed(KeyCode::Enter) {
		next_state.set(super::State::Loading);
	}
}
