use super::{
	cube::Cube,
	loading::{INDEX_CHECKPOINT, INDEX_X, INDEX_Y, INDEX_Z},
	plane::{Rotation, Translation},
};
use bevy::prelude::*;
use glam::USizeVec3;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
	Generic(USizeVec3),
	Spike(usize),
	X,
	Y,
	Z,
	Checkpoint,
}

#[derive(Bundle)]
pub struct BlockBundle {
	block: Block,
	mesh_3d: Mesh3d,
	mesh_material_3d: MeshMaterial3d<StandardMaterial>,
	transform: Transform,
	visibility: Visibility,
}

pub const SIZE: f32 = 1.0;

impl BlockBundle {
	pub fn new(
		block: Block,
		meshes: &mut Assets<Mesh>,
		texture_atlas_layouts: &Assets<TextureAtlasLayout>,
		texture_atlas_layout_handle: &Handle<TextureAtlasLayout>,
		material_handle: Handle<StandardMaterial>,
		translation: Vec3,
	) -> Self {
		let texture_atlas_indices = build_texture_atlas_indices(&block);
		let uvs = normalise_uvs(
			texture_atlas_layouts,
			texture_atlas_layout_handle,
			texture_atlas_indices,
		);

		Self {
			block,
			mesh_3d: Mesh3d(meshes.add(Cube::new(SIZE, uvs))),
			mesh_material_3d: MeshMaterial3d(material_handle),
			transform: Transform::from_translation(translation + 0.5),
			visibility: Visibility::default(),
		}
	}
}

pub fn build_texture_atlas_indices(block: &Block) -> USizeVec3 {
	match block {
		Block::Generic(indices) => *indices,
		Block::Spike(index) => USizeVec3::splat(*index),
		Block::X => USizeVec3::splat(INDEX_X),
		Block::Y => USizeVec3::splat(INDEX_Y),
		Block::Z => USizeVec3::splat(INDEX_Z),
		Block::Checkpoint => USizeVec3::splat(INDEX_CHECKPOINT),
	}
}

pub fn normalise_uvs(
	texture_atlas_layouts: &Assets<TextureAtlasLayout>,
	texture_atlas_layout_handle: &Handle<TextureAtlasLayout>,
	texture_atlas_indices: USizeVec3,
) -> (Rect, Rect, Rect) {
	let atlas_size = texture_atlas_layouts
		.get(texture_atlas_layout_handle.id())
		.unwrap()
		.size
		.as_vec2();

	let mut x = TextureAtlas {
		layout: texture_atlas_layout_handle.clone(),
		index: texture_atlas_indices.x,
	}
	.texture_rect(texture_atlas_layouts)
	.unwrap()
	.as_rect();

	x.min /= atlas_size;
	x.max /= atlas_size;

	let mut y = TextureAtlas {
		layout: texture_atlas_layout_handle.clone(),
		index: texture_atlas_indices.y,
	}
	.texture_rect(texture_atlas_layouts)
	.unwrap()
	.as_rect();

	y.min /= atlas_size;
	y.max /= atlas_size;

	let mut z = TextureAtlas {
		layout: texture_atlas_layout_handle.clone(),
		index: texture_atlas_indices.z,
	}
	.texture_rect(texture_atlas_layouts)
	.unwrap()
	.as_rect();

	z.min /= atlas_size;
	z.max /= atlas_size;

	(x, y, z)
}

const CUBE_CORNERS: [Vec3; 8] = [
	Vec3::new(1.0, 1.0, 1.0),
	Vec3::new(1.0, 1.0, -1.0),
	Vec3::new(1.0, -1.0, 1.0),
	Vec3::new(-1.0, 1.0, 1.0),
	Vec3::new(1.0, -1.0, -1.0),
	Vec3::new(-1.0, 1.0, -1.0),
	Vec3::new(-1.0, -1.0, 1.0),
	Vec3::new(-1.0, -1.0, -1.0),
];

pub fn filter_visibility(
	plane_rotation: Res<Rotation>,
	plane_translation: Res<Translation>,
	mut blocks: Query<(&Transform, &mut Visibility), With<Block>>,
) {
	// Plane equation: Ax + By + Cz + D = 0
	let plane_origin = plane_translation.0.floor() + 0.5;
	let plane_normal = plane_rotation.get() * Vec3::Z;
	let plane_point = -plane_normal.dot(plane_origin);

	for (transform, mut visibility) in &mut blocks {
		let mut above = false;
		let mut below = false;

		for corner in CUBE_CORNERS
			.iter()
			.map(|&corner| transform.translation + (SIZE / 2.0) * corner)
		{
			let distance = plane_normal.dot(corner) + plane_point;
			above |= distance > f32::EPSILON;
			below |= distance < -f32::EPSILON;
		}

		*visibility = if above && below {
			Visibility::Visible
		} else {
			Visibility::Hidden
		};
	}
}
