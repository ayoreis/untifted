use super::block::TILE_SIZE;
use bevy::prelude::*;

#[derive(Resource)]
pub struct TextureAtlasImage(pub Handle<Image>);

impl FromWorld for TextureAtlasImage {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("texture-atlas.png"))
    }
}

#[derive(Resource)]
pub struct TextureAtlasMaterial(pub Handle<StandardMaterial>);

impl FromWorld for TextureAtlasMaterial {
    fn from_world(world: &mut World) -> Self {
        let image = world.resource::<TextureAtlasImage>().0.clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        Self(materials.add(StandardMaterial {
            base_color_texture: Some(image),
            unlit: true,
            ..default()
        }))
    }
}

#[derive(Resource)]
pub struct MyTextureAtlasLayout(pub Handle<TextureAtlasLayout>);

impl FromWorld for MyTextureAtlasLayout {
    fn from_world(world: &mut World) -> Self {
        let mut texture_atlas_layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(TILE_SIZE, TILE_SIZE),
            TEXTURE_ATLAS_COLUMNS,
            TEXTURE_ATLAS_ROWS,
            None,
            None,
        );
        Self(texture_atlas_layouts.add(texture_atlas))
    }
}

pub const TEXTURE_ATLAS_COLUMNS: u32 = 16;
pub const TEXTURE_ATLAS_ROWS: u32 = 16;

pub fn plugin(app: &mut App) {
    app.init_resource::<TextureAtlasImage>()
        .init_resource::<TextureAtlasMaterial>()
        .init_resource::<MyTextureAtlasLayout>();
}
