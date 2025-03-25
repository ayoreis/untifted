use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
#[require(Transform)]
pub struct Rotation {
    pub previous: Option<Quat>,
    pub next: Quat,
    pub transition_timer: Timer,
}

impl Default for Rotation {
    fn default() -> Self {
        let mut transition_timer = Timer::from_seconds(0.5, TimerMode::Once);
        transition_timer.tick(transition_timer.remaining());

        Self {
            previous: None,
            next: Quat::default(),
            transition_timer,
        }
    }
}

impl Rotation {
    fn current(&self) -> Quat {
        let Some(previous) = self.previous else {
            return self.next;
        };

        previous.slerp(self.next, self.transition_timer.fraction())
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_rotations, update_transforms).run_if(in_state(super::State::Playing)),
    );
}

fn update_rotations(keyboard: Res<ButtonInput<KeyCode>>, mut rotations: Query<&mut Rotation>) {
    for mut rotation in &mut rotations {
        if !rotation.transition_timer.finished() {
            return;
        }

        let rotations = [
            (KeyCode::Digit1, Vec3::X),
            (KeyCode::Digit2, Vec3::Y),
            (KeyCode::Digit3, Vec3::Z),
        ];

        for (key, axis) in &rotations {
            if !keyboard.pressed(*key) {
                continue;
            };

            rotation.previous = Some(rotation.next);
            rotation.next *= Quat::from_axis_angle(*axis, FRAC_PI_2);
            rotation.transition_timer.reset();
            break;
        }
    }
}

fn update_transforms(time: Res<Time>, mut planes: Query<(&mut Rotation, &mut Transform)>) {
    for (mut rotation, mut transform) in &mut planes {
        if rotation.transition_timer.finished() {
            return;
        }

        rotation.transition_timer.tick(time.delta());
        transform.rotation = rotation.current();
    }
}
