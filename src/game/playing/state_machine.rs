use super::super::block;
use super::super::block::Block;
use super::super::player::Player;
use super::plane::{Rotate, Rotation, Translation};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::EPSILON;
use std::mem::discriminant;

#[derive(Resource, Default, Debug, Clone)]
pub enum State {
    #[default]
    Standing,
    Running,
    Jumping(Timer),
    Falling,
    Rotating(Vec3),
}

impl State {
    fn jumping() -> Self {
        Self::Jumping(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<State>().add_systems(
        Update,
        state_machine.run_if(in_state(super::super::State::Playing)),
    );
}

/// Gravitational constant
const G: f32 = 10.0;
const RUNNING_SPEED: f32 = 8.0;
const JUMPING_FORCE: Vec3 = Vec3::new(0.0, 30.0, 0.0);
const JUMPING_SPEED: f32 = 5.0;
const FALLING_SPEED: f32 = 5.0;

fn state_machine(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<State>,
    mut plane_rotation: ResMut<Rotation>,
    mut previous_state: Local<State>,
    output: Option<Single<&KinematicCharacterControllerOutput>>,
    mut controller: Single<&mut KinematicCharacterController>,
    mut transforms: Query<&mut Transform, (With<Rotate>, Without<Player>)>,
    plane_translation: Single<&GlobalTransform, With<Translation>>,
    mut blocks: Query<(&GlobalTransform, &mut Visibility), With<Block>>,
    mut player: Single<(&mut Velocity, &mut Transform), With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let direction = direction.normalize_or_zero().extend(0.0);
    let grounded = output.as_ref().map_or(false, |output| output.grounded);

    // Input
    let next_state: Option<State> = match &*state {
        State::Standing => 'standing: {
            if keyboard.pressed(KeyCode::Digit1) {
                break 'standing Some(State::Rotating(Vec3::X));
            }

            if keyboard.pressed(KeyCode::Digit2) {
                break 'standing Some(State::Rotating(Vec3::Y));
            }

            if keyboard.pressed(KeyCode::Digit3) {
                break 'standing Some(State::Rotating(Vec3::Z));
            }

            if keyboard.pressed(KeyCode::Space) {
                break 'standing Some(State::jumping());
            }

            if direction.length() > 0.0 {
                break 'standing Some(State::Running);
            }

            None
        }

        State::Running => 'running: {
            if keyboard.pressed(KeyCode::Space) {
                break 'running Some(State::jumping());
            }

            if direction.length() <= 0.0 {
                break 'running Some(State::Standing);
            }

            None
        }

        State::Jumping(timer) => 'jumping: {
            if timer.just_finished() {
                break 'jumping Some(State::Falling);
            }

            None
        }

        State::Falling => 'falling: {
            if grounded {
                break 'falling Some(State::Standing);
            }

            None
        }

        State::Rotating(_axis) => 'rotating: {
            if plane_rotation.transition_timer.just_finished() {
                break 'rotating Some(previous_state.clone());
            }

            None
        }
    };

    let mut enter = false;

    if let Some(next_state) = next_state {
        if next_state != *state {
            enter = true;
            *previous_state = state.clone();
        }

        *state = next_state;
    }

    // Enter
    if enter {
        match &*state {
            State::Standing => {}
            State::Running => {}
            State::Jumping(_timer) => {}
            State::Falling => {}

            State::Rotating(axis) => {
                plane_rotation.set(axis);
            }
        }
    }

    // Update
    let mass = 1.0;
    let gravity = mass * G * Vec3::NEG_Y;
    let mut forces = vec![gravity];

    match &mut *state {
        State::Standing => {}

        State::Running => {
            forces.push(direction * RUNNING_SPEED);
        }

        State::Jumping(timer) => {
            timer.tick(time.delta());
            forces.push(JUMPING_FORCE * timer.fraction_remaining());
            forces.push(direction * JUMPING_SPEED);
        }

        State::Falling => {
            forces.push(direction * FALLING_SPEED);
        }

        State::Rotating(_axis) => {
            plane_rotation.transition_timer.tick(time.delta());

            controller.up = plane_rotation.get() * Vec3::Y;

            for mut transform in &mut transforms {
                transform.rotation = plane_rotation.get();
            }

            let plane_origin = plane_translation.translation().floor() + 0.5;
            let plane_normal = plane_rotation.get() * Vec3::Z;

            for (transform, mut visibility) in &mut blocks {
                *visibility =
                    if block_intersects_plane(transform.translation(), plane_origin, plane_normal) {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    }
            }
        }
    };

    let velocity = &mut player.0.linvel;
    let force = plane_rotation.get() * forces.iter().sum::<Vec3>();
    let acceleration = force / mass;
    let delta_time = time.delta_secs();
    // Todo: Should be `+=`, physics don't work like this
    *velocity = acceleration * delta_time;
    controller.translation = Some(*velocity);

    if let Some(output) = &output {
        let translation = &mut player.1.translation;
        *translation += output.effective_translation * delta_time;
    }
}

const CORNERS: [Vec3; 8] = [
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(1.0, 1.0, -1.0),
    Vec3::new(1.0, -1.0, 1.0),
    Vec3::new(-1.0, 1.0, 1.0),
    Vec3::new(1.0, -1.0, -1.0),
    Vec3::new(-1.0, 1.0, -1.0),
    Vec3::new(-1.0, -1.0, 1.0),
    Vec3::new(-1.0, -1.0, -1.0),
];

/// Plane equation: Ax + By + Cz + D = 0
fn block_intersects_plane(block_center: Vec3, plane_origin: Vec3, plane_normal: Vec3) -> bool {
    let plane_point = -plane_normal.dot(plane_origin);
    let mut above = false;
    let mut below = false;

    for corner in CORNERS
        .iter()
        .map(move |&corner| block_center + (block::SIZE / 2.0) * corner)
    {
        let distance = plane_normal.dot(corner) + plane_point;

        above |= distance > EPSILON;
        below |= distance < -EPSILON;
    }

    above && below
}
