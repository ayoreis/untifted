use super::SCALED_TILE_SIZE;
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};

#[derive(Component)]
#[require(
	Camera3d,
	Projection::Orthographic(OrthographicProjection {
		scale: 1.0 / SCALED_TILE_SIZE as f32,
		..OrthographicProjection::default_3d()
	}),
	Tonemapping::None,
	Transform::from_xyz(0.0, 0.0, 16.0)
)]
pub struct GameCamera;
