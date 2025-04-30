use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Resource, Debug)]
pub struct Rotation {
	previous: Option<Quat>,
	next: Quat,
	pub transition_timer: Timer,
}

impl Rotation {
	pub fn new(rotation: Quat) -> Self {
		let mut transition_timer = Timer::from_seconds(0.5, TimerMode::Once);
		transition_timer.tick(transition_timer.remaining());

		Self {
			previous: None,
			next: rotation,
			transition_timer,
		}
	}

	pub fn get(&self) -> Quat {
		let Some(previous) = self.previous else {
			return self.next;
		};

		previous.slerp(self.next, self.transition_timer.fraction())
	}

	pub fn set(&mut self, rotation: Quat) {
		self.previous = Some(self.next);
		self.next = rotation;
	}

	pub fn set_axis(&mut self, axis: Vec3) {
		self.previous = Some(self.next);
		self.next *= Quat::from_axis_angle(axis, FRAC_PI_2);
		self.transition_timer.reset();
	}
}

#[derive(Resource, Default)]
pub struct Translation(pub Vec3);

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Rotate;

pub fn propagate_rotation(
	rotation: ResMut<Rotation>,
	mut transforms: Query<&mut Transform, With<Rotate>>,
) {
	for mut transform in &mut transforms {
		transform.rotation = rotation.get();
	}
}
