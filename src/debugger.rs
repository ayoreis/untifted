mod entities;
mod orbit_zoom_pan_camera;
mod physics_debug_render;
mod ui;
mod window;

use bevy::prelude::*;
use bevy::window::WindowClosing;
use std::time::Instant;
use window::DebuggerWindow;

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
            orbit_zoom_pan_camera::plugin,
            entities::plugin,
            physics_debug_render::plugin,
        ));
}

fn enable(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<State>>,
    mut last_escape: Local<Option<Instant>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some(instant) = *last_escape
            && instant.elapsed().as_secs_f32() > 0.2
        {
            next_state.set(State::Enabled);
        }

        *last_escape = Some(Instant::now());
    }
}

fn disable(
    mut next_state: ResMut<NextState<State>>,
    mut window_closing_events: EventReader<WindowClosing>,
    window: Single<Entity, With<DebuggerWindow>>,
) {
    for event in window_closing_events.read() {
        if event.window == *window {
            next_state.set(State::Disabled);
            break;
        }
    }
}
