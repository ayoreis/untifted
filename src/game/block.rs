use super::SCALE;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Block;

#[derive(Bundle)]
pub struct BlockBundle {
    block: Block,
    mesh3d: Mesh3d,
    mesh_material3d: MeshMaterial3d<StandardMaterial>,
    collider: Collider,
    collision_groups: CollisionGroups,
    transform: Transform,
}

impl BlockBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        material: Handle<StandardMaterial>,
        texture_atlas: TextureAtlas,
        texture_atlas_layouts: &Assets<TextureAtlasLayout>,
        translation: Vec3,
    ) -> Self {
        let texture_atlas_layout_size = texture_atlas_layouts
            .get(&texture_atlas.layout)
            .unwrap()
            .size
            .as_vec2();

        let texture: Rect = texture_atlas
            .texture_rect(texture_atlas_layouts)
            .unwrap()
            .as_rect();

        let x_min = texture.min.x / texture_atlas_layout_size.x;
        let x_max = texture.max.x / texture_atlas_layout_size.x;
        let y_min = texture.min.y / texture_atlas_layout_size.y;
        let y_max = texture.max.y / texture_atlas_layout_size.y;

        let mesh = meshes.add(Self::mesh(x_min, x_max, y_min, y_max));

        Self {
            block: Block,
            mesh3d: Mesh3d(mesh),
            mesh_material3d: MeshMaterial3d(material),
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            collision_groups: CollisionGroups::new(COLLISION_GROUP, COLLISION_FILTER),
            transform: Transform::from_translation(translation + Vec3::splat(0.5)),
        }
    }

    fn mesh(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Mesh {
        let min = -0.5;
        let max = 0.5;

        #[rustfmt::skip]
        let positions = vec![
            // X positive
            [max, max, max], [max, max, min], [max, min, min], [max, min, max],
            // Y positive
            [min, max, min], [max, max, min], [max, max, max], [min, max, max],
            // Z positive
            [min, max, max], [max, max, max], [max, min, max], [min, min, max],
        ];

        #[rustfmt::skip]
        let uvs = vec![
            // X positive
            [x_min, y_min], [x_max, y_min], [x_max, y_max], [x_min, y_max],
            // Y positive
            [x_min, y_min], [x_max, y_min], [x_max, y_max], [x_min, y_max],
            // Z positive
            [x_min, y_min], [x_max, y_min], [x_max, y_max], [x_min, y_max],
        ];

        #[rustfmt::skip]
        let indices = Indices::U32(vec![
        	// X positive
            0, 3, 1, 1, 3, 2,
            // Y positive
            4, 7, 5, 5, 7, 6,
            // Z positive
            8, 11, 9, 9, 11, 10,
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

pub const TILE_SIZE: u32 = 8;
pub const SCALED_TILE_SIZE: f32 = (TILE_SIZE * SCALE) as f32;
const COLLISION_GROUP: Group = Group::GROUP_1;
const COLLISION_FILTER: Group = Group::all();
