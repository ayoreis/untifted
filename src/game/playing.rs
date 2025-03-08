use super::GameState;
use crate::MainCamera;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_camera_state, update_camera_transform).run_if(in_state(GameState::Playing)),
    );
}

fn update_camera_state(keyboard: Res<ButtonInput<KeyCode>>, mut camera: Single<&mut MainCamera>) {
    if !camera.transition_timer.finished() {
        return;
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        camera.previous_transform = Some(camera.next_transform);
        camera.next_transform.rotate_local_x(FRAC_PI_2);
        camera.transition_timer.reset();
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        camera.previous_transform = Some(camera.next_transform);
        camera.next_transform.rotate_local_y(FRAC_PI_2);
        camera.transition_timer.reset();
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        camera.previous_transform = Some(camera.next_transform);
        camera.next_transform.rotate_local_z(FRAC_PI_2);
        camera.transition_timer.reset();
    }
}

fn update_camera_transform(time: Res<Time>, camera: Single<(&mut MainCamera, &mut Transform)>) {
    let (mut state, mut transform) = camera.into_inner();

    if state.transition_timer.finished() {
        return;
    }

    state.transition_timer.tick(time.delta());

    transform.rotation = state.previous_transform.unwrap().rotation.slerp(
        state.next_transform.rotation,
        state.transition_timer.fraction(),
    );
    transform.translation = Vec3::ZERO - transform.forward() * 10.0;
}
