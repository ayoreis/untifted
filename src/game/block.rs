use super::SCALE;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Component, Clone, Serialize)]
pub struct TextureAtlasIndices {
    /// Right
    pub x: usize,
    /// Top
    pub y: usize,
    /// Front
    pub z: usize,
}

#[derive(Component)]
pub struct Block;

#[derive(Bundle)]
pub struct BlockBundle {
    block: Block,
    transform: Transform,
    mesh3d: Mesh3d,
    texture_atlas_indices: TextureAtlasIndices,
    mesh_material3d: MeshMaterial3d<StandardMaterial>,
    collider: Collider,
}

pub const SIZE: f32 = 1.0;

impl BlockBundle {
    pub fn new(
        translation: &Vec3,
        meshes: &mut Assets<Mesh>,
        texture_atlas_layouts: &Assets<TextureAtlasLayout>,
        texture_atlas_layout_handle: Handle<TextureAtlasLayout>,
        texture_atlas_indices: TextureAtlasIndices,
        material_handle: Handle<StandardMaterial>,
    ) -> Self {
        let mesh = Self::mesh(
            texture_atlas_layouts,
            texture_atlas_layout_handle,
            &texture_atlas_indices,
        );

        let mesh_handle = meshes.add(mesh);
        let half_extents = SIZE / 2.0;

        Self {
            block: Block,
            transform: Transform::from_translation(translation + 0.5),
            mesh3d: Mesh3d(mesh_handle),
            texture_atlas_indices,
            mesh_material3d: MeshMaterial3d(material_handle),
            collider: Collider::cuboid(half_extents, half_extents, half_extents),
        }
    }

    pub fn mesh(
        layouts: &Assets<TextureAtlasLayout>,
        layout_handle: Handle<TextureAtlasLayout>,
        indices: &TextureAtlasIndices,
    ) -> Mesh {
        let main = 0.0;
        let cross_min = -0.5;
        let cross_max = 0.5;

        let positions = vec![
            // X
            [main, cross_max, cross_max],
            [main, cross_max, cross_min],
            [main, cross_min, cross_min],
            [main, cross_min, cross_max],
            // Y
            [cross_min, main, cross_min],
            [cross_max, main, cross_min],
            [cross_max, main, cross_max],
            [cross_min, main, cross_max],
            // Z
            [cross_min, cross_max, main],
            [cross_max, cross_max, main],
            [cross_max, cross_min, main],
            [cross_min, cross_min, main],
        ];

        let uvs = Self::uv(layouts, layout_handle.clone(), indices.x)
            .into_iter()
            .chain(Self::uv(layouts, layout_handle.clone(), indices.y))
            .chain(Self::uv(layouts, layout_handle.clone(), indices.z))
            .collect::<Vec<_>>();

        let indices = Indices::U32(vec![
            0, 3, 1, 1, 3, 2, // X
            4, 7, 5, 5, 7, 6, // Y
            8, 11, 9, 9, 11, 10, // Z
        ]);

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(indices)
    }

    fn uv(
        layouts: &Assets<TextureAtlasLayout>,
        layout_handle: Handle<TextureAtlasLayout>,
        index: usize,
    ) -> [[f32; 2]; 4] {
        let atlas = TextureAtlas {
            layout: layout_handle,
            index,
        };

        let texture_rectangle = atlas.texture_rect(layouts).unwrap().as_rect();
        let atlas_size = layouts.get(atlas.layout.id()).unwrap().size.as_vec2();

        let x_min = texture_rectangle.min.x / atlas_size.x;
        let x_max = texture_rectangle.max.x / atlas_size.x;
        let y_min = texture_rectangle.min.y / atlas_size.y;
        let y_max = texture_rectangle.max.y / atlas_size.y;

        return [
            [x_min, y_min],
            [x_max, y_min],
            [x_max, y_max],
            [x_min, y_max],
        ];
    }
}

pub const TILE_SIZE: u32 = 8;
pub const SCALED_TILE_SIZE: f32 = (TILE_SIZE * SCALE) as f32;
