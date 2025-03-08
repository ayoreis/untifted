#![feature(stmt_expr_attributes, let_chains)]

mod editor;
mod game;
mod splash;
mod systems;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;

#[derive(Clone, PartialEq, Eq, Hash, Debug, States, Default)]
enum AppState {
    #[default]
    Splash,
    Game,
    Editor,
}

#[derive(Component)]
struct MainCamera {
    previous_transform: Option<Transform>,
    next_transform: Transform,
    transition_timer: Timer,
}

#[derive(Component)]
struct Tile;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 180;
const TILE_SIZE: u32 = 8;
const SCALE: u32 = 4;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Untifted".into(),
                        resizable: false,
                        resolution: WindowResolution::new(
                            (WIDTH * SCALE) as f32,
                            (HEIGHT * SCALE) as f32,
                        )
                        .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, gizmos)
        .add_plugins((splash::plugin, game::plugin, editor::plugin))
        .run();
}

fn spawn_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    translation: Vec3,
) {
    commands.spawn((
        Tile,
        Mesh3d(meshes.add(
            Mesh::from(Cuboid::from_length(1.0)).with_inserted_attribute(
                Mesh::ATTRIBUTE_UV_0,
                vec![
                    // Front side
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    // Back side
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    // Right side
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    // Left side
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    // Top side
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    // Bottom side
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                ],
            ),
        )),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("tile.png")),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform {
            translation: translation + Vec3::splat(0.5),
            ..default()
        },
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let transform = Transform {
        translation: Vec3::new(0.0, 0.0, 10.0),
        ..default()
    };
    let mut transition_timer = Timer::from_seconds(0.3, TimerMode::Once);
    transition_timer.tick(transition_timer.duration());

    commands.spawn((
        MainCamera {
            previous_transform: None,
            next_transform: transform,
            transition_timer,
        },
        Camera3d::default(),
        IsDefaultUiCamera,
        Projection::from(OrthographicProjection {
            scale: 1.0 / (TILE_SIZE * SCALE) as f32,
            ..OrthographicProjection::default_3d()
        }),
        transform,
    ));

    spawn_block(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        Vec3::splat(0.0),
    );

    spawn_block(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        Vec3::new(1.0, 0.0, 0.0),
    );

    spawn_block(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        Vec3::new(0.0, 1.0, 0.0),
    );

    spawn_block(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        Vec3::new(0.0, 0.0, 1.0),
    );
}

fn gizmos(mut gizmos: Gizmos) {
    gizmos.axes(Transform::default(), 5.0);
    gizmos.grid(
        Isometry3d::from_rotation(Quat::from_rotation_x(0.0)),
        UVec2::splat(8),
        Vec2::splat(1.0),
        Color::hsva(0.0, 0.0, 1.0, 0.1),
    );
}
