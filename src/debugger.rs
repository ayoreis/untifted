mod editor;
mod entities;
mod info;
mod orbit_zoom_pan_camera;
mod physics_debug_render;
mod ui;
mod window;

use bevy::prelude::*;
use bevy::window::WindowClosing;
use std::time::Instant;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum State {
    #[default]
    Enabled,
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
        .add_plugins((
            window::plugin,
            ui::plugin,
            info::plugin,
            editor::plugin,
            orbit_zoom_pan_camera::plugin,
            physics_debug_render::plugin,
            entities::plugin,
        ));
}

const DOUBLE_PRESS_SPEED: f32 = 0.5;

fn enable(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<State>>,
    mut last_press: Local<Option<Instant>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some(instant) = *last_press
            && instant.elapsed().as_secs_f32() > DOUBLE_PRESS_SPEED
        {
            next_state.set(State::Enabled);
        }

        *last_press = Some(Instant::now());
    }
}

fn disable(
    mut window_closing_events: EventReader<WindowClosing>,
    mut next_state: ResMut<NextState<State>>,
) {
    for _event in window_closing_events.read() {
        next_state.set(State::Disabled);
    }
}
