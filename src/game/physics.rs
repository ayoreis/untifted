use super::{
	block::{self, Block},
	plane::Rotation,
	player::{self, Player},
};
use bevy::{
	math::{
		InvalidDirectionError,
		bounding::{Aabb3d, AabbCast3d, BoundingVolume, IntersectsVolume},
	},
	prelude::*,
};
use std::{
	collections::HashMap,
	mem::{Discriminant, discriminant, replace},
};

#[derive(Component, Default)]
#[require(Output)]
pub struct Input {
	pub velocity: Vec2,
}

#[derive(Component, Default)]
pub struct Output {
	pub grounded: bool,
	previous: HashMap<Discriminant<Block>, Vec3>,
	current: HashMap<Discriminant<Block>, Vec3>,
}

impl Output {
	fn reset(&mut self) {
		self.grounded = false;
		self.previous = replace(&mut self.current, HashMap::new());
	}

	pub fn just_collided(&self, block: Block) -> Option<Vec3> {
		if !self.previous.contains_key(&discriminant(&block)) {
			return self.current.get(&discriminant(&block)).map(|vec3| *vec3);
		}

		None
	}
}

fn intersects_x(a: &Aabb3d, b: &Aabb3d) -> bool {
	a.max.x > b.min.x && a.min.x < b.max.x
}

fn intersects_y(a: &Aabb3d, b: &Aabb3d) -> bool {
	a.max.y > b.min.y && a.min.y < b.max.y
}

fn intersects_z(a: &Aabb3d, b: &Aabb3d) -> bool {
	a.max.z > b.min.z && a.min.z < b.max.z
}

fn intersects(a: &Aabb3d, b: &Aabb3d) -> bool {
	intersects_x(a, b) && intersects_y(a, b) && intersects_z(a, b)
}

pub fn physics(
	time: Res<Time>,
	rotation: Res<Rotation>,
	player: Single<(&mut Transform, &mut Input, &mut Output), (With<Player>, Without<Block>)>,
	blocks: Query<(&Visibility, &Block, &Transform)>,
) {
	let (mut transform, mut input, mut output) = player.into_inner();
	output.reset();

	let player_aabb = Aabb3d::new(Vec3::ZERO, Vec3::splat(player::SIZE / 2.0));
	let velocity = rotation.get() * input.velocity.extend(0.0);
	let mut delta_translation = velocity * time.delta_secs();

	let blocks = blocks
		.iter()
		.filter(|(visibility, ..)| !matches!(visibility, Visibility::Hidden))
		.map(|(_, block, transform)| {
			(
				block,
				Aabb3d::new(transform.translation, Vec3::splat(block::SIZE / 2.0)),
			)
		});

	let mut right = false;
	let mut left = false;
	let mut top = false;
	let mut bottom = false;
	let mut front = false;
	let mut back = false;

	for (block, block_aabb) in blocks {
		let previous = Aabb3d::new(transform.translation, player_aabb.half_size());
		let broad_aabb = {
			let next = Aabb3d::new(
				previous.center() + Vec3A::from(delta_translation),
				previous.half_size(),
			);

			Aabb3d {
				min: Vec3A::min(previous.min, next.min),
				max: Vec3A::max(previous.max, next.max),
			}
		};

		match block {
			Block::Generic(_) if intersects(&broad_aabb, &block_aabb) => {
				let player_aabb_cast: Result<AabbCast3d, InvalidDirectionError> = try {
					AabbCast3d::new(
						player_aabb,
						transform.translation,
						// TODO: Why does this sometimes fail?
						Dir3::new(delta_translation)?,
						delta_translation.length(),
					)
				};

				if let Ok(player_aabb_cast) = player_aabb_cast {
					if let Some(toi) = player_aabb_cast.aabb_collision_at(block_aabb) {
						delta_translation = Vec3::from(
							player_aabb_cast.ray.origin + *player_aabb_cast.ray.direction * toi,
						) - transform.translation;

						input.velocity = Vec2::ZERO;
					}
				}
			}

			block if previous.intersects(&block_aabb) => {
				output
					.current
					.insert(discriminant(block), block_aabb.center().into());
			}

			_ => {}
		}

		let player_aabb = Aabb3d::new(
			transform.translation + delta_translation,
			player_aabb.half_size(),
		);

		let intersects_x = intersects_x(&player_aabb, &block_aabb);
		let intersects_y = intersects_y(&player_aabb, &block_aabb);
		let intersects_z = intersects_z(&player_aabb, &block_aabb);

		if intersects_y && intersects_z {
			right |= block_aabb.min.x == player_aabb.max.x;
			left |= block_aabb.max.x == player_aabb.min.x;
		}

		if intersects_x && intersects_z {
			top |= block_aabb.min.y == player_aabb.max.y;
			bottom |= block_aabb.max.y == player_aabb.min.y;
		}

		if intersects_x && intersects_y {
			front |= block_aabb.min.z == player_aabb.max.z;
			back |= block_aabb.max.z == player_aabb.min.z;
		}
	}

	let positive =
		rotation.get().inverse() * IVec3::new(right as i32, top as i32, front as i32).as_vec3();
	let negative =
		rotation.get().inverse() * -IVec3::new(left as i32, bottom as i32, back as i32).as_vec3();

	output.grounded = f32::min(positive.y, negative.y) != 0.0;

	transform.translation += delta_translation;
}
