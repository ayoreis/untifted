mod editor;
mod info;
mod ui;
mod window;

use bevy::{prelude::*, window::WindowClosing};
use std::time::Instant;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
	Enabled,
	#[default]
	Disabled,
}

pub fn plugin(app: &mut App) {
	app.init_state::<State>()
		.add_systems(
			Update,
			(
				enable.run_if(in_state(State::Disabled)),
				disable.run_if(in_state(State::Enabled)),
			),
		)
		.add_plugins((window::plugin, ui::plugin, info::plugin, editor::plugin));
}

fn enable(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut next_state: ResMut<NextState<State>>,
	mut last_press: Local<Option<Instant>>,
) {
	if keyboard.just_pressed(KeyCode::Escape) {
		if let Some(instant) = *last_press
			&& instant.elapsed().as_secs_f32() < 0.5
		{
			next_state.set(State::Enabled);
		}

		*last_press = Some(Instant::now());
	}
}

fn disable(
	window_closing_events: EventReader<WindowClosing>,
	mut next_state: ResMut<NextState<State>>,
) {
	if !window_closing_events.is_empty() {
		next_state.set(State::Disabled);
	}
}
