pub mod game_loader;
pub mod level_loader;

use super::block::{BlockBundle, TILE_SIZE};
use super::camera::GameCamera;
use super::plane::Rotate;
use super::player::Player;
use bevy::asset::LoadState;
use bevy::prelude::*;
use game_loader::Game;
use level_loader::Level;
use std::env;
use std::path::PathBuf;

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(super::State = super::State::Loading)]
enum State {
    #[default]
    Game,
    Level,
    Spawn,
}

#[derive(Resource)]
pub struct MyTextureAtlasLayout(pub Handle<TextureAtlasLayout>);

pub const TEXTURE_ATLAS_COLUMNS: u32 = 16;
pub const TEXTURE_ATLAS_ROWS: u32 = 16;

impl FromWorld for MyTextureAtlasLayout {
    fn from_world(world: &mut World) -> Self {
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(TILE_SIZE, TILE_SIZE),
            TEXTURE_ATLAS_COLUMNS,
            TEXTURE_ATLAS_ROWS,
            None,
            None,
        );

        Self(layouts.add(texture_atlas))
    }
}

#[derive(Resource)]
pub struct TextureAtlasImage(pub Handle<Image>);

impl FromWorld for TextureAtlasImage {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("texture-atlas.png"))
    }
}

#[derive(Resource)]
pub struct BlockMaterial(pub Handle<StandardMaterial>);

impl FromWorld for BlockMaterial {
    fn from_world(world: &mut World) -> Self {
        let image = world.resource::<TextureAtlasImage>().0.clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = StandardMaterial {
            base_color_texture: Some(image),
            unlit: true,
            cull_mode: None,
            ..default()
        };

        Self(materials.add(material))
    }
}

#[derive(Resource)]
pub struct LoadingGame(pub Handle<Game>);

const GAME_DIRECTORY: &str = ".untifted";
const GAME_FILE: &str = "game.json";

impl FromWorld for LoadingGame {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let home_directory = env::var("HOME").unwrap();
        let path = PathBuf::from(format!("{home_directory}/{GAME_DIRECTORY}/{GAME_FILE}"));
        Self(asset_server.load(path))
    }
}

#[derive(Resource)]
struct LoadingLevel(Handle<Level>);

pub const LEVELS_DIRECTORY: &str = "levels";

impl FromWorld for LoadingLevel {
    fn from_world(world: &mut World) -> Self {
        let games = world.resource::<Assets<Game>>();
        let game = games.get(&world.resource::<LoadingGame>().0).unwrap();
        let asset_server = world.resource::<AssetServer>();
        let file = format!("{level}.json", level = game.level);
        let path = PathBuf::from(format!("{LEVELS_DIRECTORY}/{file}"));
        Self(asset_server.load(path))
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins((game_loader::plugin, level_loader::plugin))
        .add_sub_state::<State>()
        .init_resource::<MyTextureAtlasLayout>()
        .init_resource::<TextureAtlasImage>()
        .init_resource::<BlockMaterial>()
        .add_systems(OnEnter(State::Game), load_game)
        .add_systems(Update, await_game.run_if(in_state(State::Game)))
        .add_systems(OnEnter(State::Level), load_level)
        .add_systems(Update, await_level.run_if(in_state(State::Level)))
        .add_systems(OnEnter(State::Spawn), spawn);
}

fn load_game(mut commands: Commands) {
    commands.init_resource::<LoadingGame>();
}

fn await_game(
    asset_server: Res<AssetServer>,
    mut loading_game: ResMut<LoadingGame>,
    mut games: ResMut<Assets<Game>>,
    mut next_state: ResMut<NextState<State>>,
) {
    let load_state = asset_server.get_load_state(loading_game.0.id()).unwrap();

    match &load_state {
        LoadState::Loaded => {
            next_state.set(State::Level);
        }

        LoadState::Failed(_error) => {
            loading_game.0 = games.add(Game::default());
            next_state.set(State::Level);
        }

        _ => (),
    }
}

fn load_level(mut commands: Commands) {
    commands.init_resource::<LoadingLevel>();
}

fn await_level(
    asset_server: Res<AssetServer>,
    loading_level: Res<LoadingLevel>,
    mut next_state: ResMut<NextState<State>>,
) {
    let load_state = asset_server.get_load_state(loading_level.0.id()).unwrap();

    if load_state.is_loaded() {
        next_state.set(State::Spawn);
    }
}

fn spawn(
    mut commands: Commands,
    levels: Res<Assets<Level>>,
    loading_level: Res<LoadingLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    layout: Res<MyTextureAtlasLayout>,
    material: Res<BlockMaterial>,
    mut next_state: ResMut<NextState<super::State>>,
) {
    commands
        .spawn((Name::new("Camera plane rotation"), Rotate))
        .with_child(GameCamera);

    commands.spawn((Player, Transform::from_xyz(0.0, 10.0, 0.5)));

    let level = levels.get(loading_level.0.id()).unwrap();

    for block in &level.blocks {
        commands.spawn(BlockBundle::new(
            &block.translation,
            &mut meshes,
            &layouts,
            layout.0.clone(),
            block.texture_atlas_indices.clone(),
            material.0.clone(),
        ));
    }

    next_state.set(super::State::Playing);
}
