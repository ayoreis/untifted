use super::{
	TILE_SIZE,
	block::BlockBundle,
	camera::GameCamera,
	game_loader::{Game, GameLoader},
	level_loader::{Level, LevelLoader},
	plane::{Rotate, Rotation, Translation},
	player::PlayerBundle,
};
use bevy::prelude::*;
use std::ops::RangeInclusive;

#[derive(Resource)]
pub struct MyTextureAtlasLayout(pub Handle<TextureAtlasLayout>);

pub const TEXTURE_ATLAS_COLUMNS: u32 = 4;
pub const TEXTURE_ATLAS_ROWS: u32 = 16;

const INDEX_PLAYER: usize = 64;
pub const INDEX_SPIKE: RangeInclusive<usize> = 4..=7;
pub const INDEX_X: usize = 8;
pub const INDEX_Y: usize = 9;
pub const INDEX_Z: usize = 10;
pub const INDEX_CHECKPOINT: usize = 11;

impl FromWorld for MyTextureAtlasLayout {
	fn from_world(world: &mut World) -> Self {
		let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
		let mut layout = TextureAtlasLayout::from_grid(
			UVec2::splat(TILE_SIZE),
			TEXTURE_ATLAS_COLUMNS,
			TEXTURE_ATLAS_ROWS,
			None,
			None,
		);
		layout.add_texture(URect::new(0, 0, 7, 7));
		Self(layouts.add(layout))
	}
}

#[derive(Resource)]
pub struct TextureAtlasImage(pub Handle<Image>);

impl FromWorld for TextureAtlasImage {
	fn from_world(world: &mut World) -> Self {
		let asset_server = world.resource::<AssetServer>();
		let handle = asset_server.load("texture_atlas.png");
		Self(handle)
	}
}

#[derive(Resource)]
pub struct TextureAtlasMaterial(pub Handle<StandardMaterial>);

impl FromWorld for TextureAtlasMaterial {
	fn from_world(world: &mut World) -> Self {
		let image_handle = world.resource::<TextureAtlasImage>().0.clone();
		let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
		let material = StandardMaterial {
			base_color_texture: Some(image_handle),
			alpha_mode: AlphaMode::Blend,
			unlit: true,
			..default()
		};
		let handle = materials.add(material);
		Self(handle)
	}
}

#[derive(Resource)]
pub struct LoadingGame(pub Handle<Game>);
pub const GAME_PATH: &str = "game.json";

impl FromWorld for LoadingGame {
	fn from_world(world: &mut World) -> Self {
		let asset_server = world.resource::<AssetServer>();
		Self(asset_server.load(GAME_PATH))
	}
}

#[derive(Resource)]
pub struct LoadingLevel(pub Handle<Level>);
pub const LEVEL_PATH: &str = "level.json";

impl FromWorld for LoadingLevel {
	fn from_world(world: &mut World) -> Self {
		let asset_server = world.resource::<AssetServer>();
		Self(asset_server.load(LEVEL_PATH))
	}
}

pub fn plugin(app: &mut App) {
	app.init_asset_loader::<GameLoader>()
		.init_asset_loader::<LevelLoader>()
		.init_asset::<Game>()
		.init_asset::<Level>()
		.init_resource::<MyTextureAtlasLayout>()
		.init_resource::<TextureAtlasImage>()
		.init_resource::<TextureAtlasMaterial>()
		.init_resource::<LoadingGame>()
		.init_resource::<LoadingLevel>()
		.init_resource::<Translation>()
		.add_systems(Update, await_assets.run_if(in_state(super::State::Loading)));
}

fn await_assets(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	games: Res<Assets<Game>>,
	levels: Res<Assets<Level>>,
	loading_game: Res<LoadingGame>,
	loading_level: Res<LoadingLevel>,
	mut meshes: ResMut<Assets<Mesh>>,
	texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
	texture_atlas_layout: Res<MyTextureAtlasLayout>,
	texture_atlas_material: Res<TextureAtlasMaterial>,
	mut next_state: ResMut<NextState<super::State>>,
) {
	let load_state_game = asset_server.get_load_state(loading_game.0.id()).unwrap();
	let load_state_level = asset_server.get_load_state(loading_game.0.id()).unwrap();

	if !load_state_game.is_loaded() || !load_state_level.is_loaded() {
		return;
	}

	let game = games.get(loading_game.0.id()).unwrap();
	let level = levels.get(loading_level.0.id()).unwrap();

	commands.insert_resource(Rotation::new(game.checkpoint.rotation));
	commands.spawn((Rotate, children![GameCamera]));

	let texture_atlas = TextureAtlas {
		layout: texture_atlas_layout.0.clone(),
		index: INDEX_PLAYER,
	};

	commands.spawn(PlayerBundle::new(
		&mut meshes,
		&texture_atlas_layouts,
		texture_atlas,
		texture_atlas_material.0.clone(),
		game.checkpoint.translation.into(),
	));

	for block in &level.blocks {
		commands.spawn(BlockBundle::new(
			block.kind.clone(),
			&mut meshes,
			&texture_atlas_layouts,
			&texture_atlas_layout.0.clone(),
			texture_atlas_material.0.clone(),
			block.translation,
		));
	}

	next_state.set(super::State::Playing);
}
