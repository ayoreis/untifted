use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Resource)]
pub struct Rotation {
    previous: Option<Quat>,
    next: Quat,
    pub transition_timer: Timer,
}

impl Rotation {
    pub fn set(&mut self, axis: &Vec3) {
        self.previous = Some(self.next);
        self.next *= Quat::from_axis_angle(*axis, FRAC_PI_2);
        self.transition_timer.reset();
    }

    pub fn get(&self) -> Quat {
        let Some(previous) = self.previous else {
            return self.next;
        };

        previous.slerp(self.next, self.transition_timer.fraction())
    }
}

const ROTATION_DURATION: f32 = 0.5;

impl Default for Rotation {
    fn default() -> Self {
        let mut transition_timer = Timer::from_seconds(ROTATION_DURATION, TimerMode::Once);
        transition_timer.tick(transition_timer.remaining());

        Self {
            previous: Option::default(),
            next: Quat::default(),
            transition_timer,
        }
    }
}

#[derive(Component, Default)]
pub struct Translation;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Rotate;

pub fn plugin(app: &mut App) {
    app.init_resource::<Rotation>();
}
