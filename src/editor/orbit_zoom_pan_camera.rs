use super::EditorCamera;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

pub fn orbit_zoom_pan(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    camera: Single<(&mut EditorCamera, &mut Transform)>,
) {
    let (mut state, mut transform) = camera.into_inner();
    let radius = transform.translation.distance(state.origin) * (-mouse_scroll.delta.y).exp();

    if mouse.pressed(MouseButton::Middle) {
        if keyboard.pressed(KeyCode::ShiftLeft) {
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
