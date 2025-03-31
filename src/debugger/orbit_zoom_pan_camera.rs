use super::window::DebuggerWindow;
use crate::systems::despawn_recursive;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;

#[derive(Component, Default)]
struct OrbitZoomPanCamera {
    origin: Vec3,
}

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_camera)
        .add_systems(Update, update.run_if(in_state(super::State::Enabled)))
        .add_systems(
            OnExit(super::State::Enabled),
            despawn_recursive::<With<OrbitZoomPanCamera>>,
        );
}

const STARTING_DISTANCE: f32 = 5.0;

fn spawn_camera(trigger: Trigger<OnAdd, DebuggerWindow>, mut commands: Commands) {
    commands.spawn((
        OrbitZoomPanCamera::default(),
        Camera3d::default(),
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(trigger.entity())),
            ..default()
        },
        Transform::from_translation(Vec3::splat(STARTING_DISTANCE)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

const ORBIT_SENSITIVITY: f32 = 0.005;
const ZOOM_SESITIVITY: f32 = 1.0;
const PAN_SENSITIVITY: f32 = 0.001;

fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    camera: Single<(&mut OrbitZoomPanCamera, &mut Transform)>,
) {
    let (mut camera, mut transform) = camera.into_inner();
    let distance = transform.translation.distance(camera.origin)
        * (-mouse_scroll.delta.y).exp()
        * ZOOM_SESITIVITY;

    if mouse.pressed(MouseButton::Middle) {
        if keyboard.pressed(KeyCode::ShiftLeft) {
            let delta = mouse_motion.delta * PAN_SENSITIVITY;
            camera.origin += transform.left() * delta.x * distance;
            camera.origin += transform.up() * delta.y * distance;
        } else {
            // Todo: upside down bugs
            let delta = mouse_motion.delta * ORBIT_SENSITIVITY;
            let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            yaw -= delta.x;
            pitch -= delta.y;
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }

    transform.translation = camera.origin - transform.forward() * distance;
}
