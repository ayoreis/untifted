use bevy::{
	asset::RenderAssetUsages,
	prelude::*,
	render::mesh::{Indices, PrimitiveTopology},
};

pub struct Cube {
	length: f32,
	uvs: (Rect, Rect, Rect),
}

impl Cube {
	pub fn new(length: f32, uvs: (Rect, Rect, Rect)) -> Self {
		Self { length, uvs }
	}
}

pub struct CubeMeshBuilder {
	length: f32,
	uvs: (Rect, Rect, Rect),
}

impl MeshBuilder for CubeMeshBuilder {
	fn build(&self) -> Mesh {
		let min = self.length / -2.0;
		let max = self.length / 2.0;

		#[rustfmt::skip]
		let positions = vec![
			// Positive X
			[max, max, max], [max, max, min], [max, min, min], [max, min, max],
			// Negative P
			[min, max, max], [min, max, min], [min, min, min], [min, min, max],
			// Positive Y
			[min, max, min], [max, max, min], [max, max, max], [min, max, max],
			// Negative Y
			[min, min, min], [max, min, min], [max, min, max], [min, min, max],
			// Positive Z
			[min, max, max], [max, max, max], [max, min, max], [min, min, max],
			// Negative Z
			[min, max, min], [max, max, min], [max, min, min], [min, min, min],
		];

		let uvs = build_uvs(self.uvs);

		#[rustfmt::skip]
		let indices = Indices::U32(vec![
			// Positive X
			0, 3, 1, 1, 3, 2,
			// Negative X
			4, 6, 7, 4, 5, 6,
			// Positive Y
			8, 11, 9, 9, 11, 10,
			// Negative Y
			12, 14, 15, 12, 13, 14,
			// Positive Z
			16, 19, 17, 17, 19, 18,
			// Negative Z
			20, 22, 23, 20, 21, 22,
		]);

		Mesh::new(
			PrimitiveTopology::TriangleList,
			RenderAssetUsages::default(),
		)
		.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
		.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
		.with_inserted_indices(indices)
	}
}

pub fn build_uvs(uvs: (Rect, Rect, Rect)) -> Vec<[f32; 2]> {
	#[rustfmt::skip]
	vec![
		// Positive X
		[uvs.0.min.x, uvs.0.min.y], [uvs.0.max.x, uvs.0.min.y],
		[uvs.0.max.x, uvs.0.max.y], [uvs.0.min.x, uvs.0.max.y],
		// Negative X
		[uvs.0.min.x, uvs.0.min.y], [uvs.0.max.x, uvs.0.min.y],
		[uvs.0.max.x, uvs.0.max.y], [uvs.0.min.x, uvs.0.max.y],
		// Positive Y
		[uvs.1.min.x, uvs.1.min.y], [uvs.1.max.x, uvs.1.min.y],
		[uvs.1.max.x, uvs.1.max.y], [uvs.1.min.x, uvs.1.max.y],
		// Negative Y
		[uvs.1.min.x, uvs.1.min.y], [uvs.1.max.x, uvs.1.min.y],
		[uvs.1.max.x, uvs.1.max.y], [uvs.1.min.x, uvs.1.max.y],
		// Positive Z
		[uvs.2.min.x, uvs.2.min.y], [uvs.2.max.x, uvs.2.min.y],
		[uvs.2.max.x, uvs.2.max.y], [uvs.2.min.x, uvs.2.max.y],
		// Negative Z
		[uvs.2.min.x, uvs.2.min.y], [uvs.2.max.x, uvs.2.min.y],
		[uvs.2.max.x, uvs.2.max.y], [uvs.2.min.x, uvs.2.max.y],
	]
}

impl Meshable for Cube {
	type Output = CubeMeshBuilder;

	fn mesh(&self) -> Self::Output {
		Self::Output {
			length: self.length,
			uvs: self.uvs,
		}
	}
}

impl From<Cube> for Mesh {
	fn from(cube: Cube) -> Self {
		cube.mesh().build()
	}
}
