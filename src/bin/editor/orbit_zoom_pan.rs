use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Camera3d)]
pub struct OrbitZoomPanState {
    pub origin: Vec3,
}

pub fn orbit_zoom_pan(
    camera: Single<(&mut OrbitZoomPanState, &mut Transform), With<Camera3d>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_button_input: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    let (mut state, mut transform) = camera.into_inner();
    let radius = transform.translation.distance(state.origin) * (-mouse_scroll.delta.y).exp();

    if mouse_button_input.pressed(MouseButton::Middle) {
        if keyboard_button_input.pressed(KeyCode::ShiftLeft) {
            let delta = mouse_motion.delta * 0.001;

            if delta != Vec2::ZERO {
                state.origin += transform.right() * -delta.x * radius;
                state.origin += transform.up() * delta.y * radius;
            }
        } else {
            // TODO upside down bugs
            let delta = mouse_motion.delta * 0.005;
            let delta_yaw = delta.x;
            let delta_pitch = delta.y;

            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

            let yaw = yaw - delta_yaw;
            let pitch = pitch - delta_pitch;

            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }

    transform.translation = state.origin - transform.forward() * radius;
}
