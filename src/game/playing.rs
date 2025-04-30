use super::{
	block::filter_visibility,
	physics::physics,
	plane::{Rotation, Translation, propagate_rotation},
	player::{self, MovementState, die_out_of_bounds, movement, respawn},
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
	app.add_sub_state::<player::State>()
		.init_resource::<MovementState>()
		.add_systems(OnEnter(super::State::Playing), (filter_visibility, physics))
		.add_systems(
			OnEnter(player::State::Dead),
			(respawn, propagate_rotation, filter_visibility).chain(),
		)
		.add_systems(
			Update,
			(
				movement,
				propagate_rotation
					.run_if(resource_exists::<Rotation>.and(resource_changed::<Rotation>)),
				die_out_of_bounds,
				physics,
				filter_visibility.run_if(
					resource_exists::<Rotation>
						.and(resource_changed::<Rotation>)
						.or(resource_changed::<Translation>),
				),
			)
				.run_if(in_state(player::State::Alive))
				.chain(),
		);
}
