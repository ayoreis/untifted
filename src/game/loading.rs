use super::block::BlockBundle;
use super::camera::GameCamera;
use super::data;
use super::data::{MyTextureAtlasLayout, TextureAtlasMaterial};
use super::plane;
use super::player::Player;
use bevy::prelude::*;

#[derive(Resource)]
struct LoadingTimer(Timer);

impl Default for LoadingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<LoadingTimer>()
        .add_plugins(data::plugin)
        .add_systems(OnEnter(super::State::Loading), spawn)
        .add_systems(Update, load.run_if(in_state(super::State::Loading)));
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<TextureAtlasMaterial>,
    texture_atlas_layout: Res<MyTextureAtlasLayout>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn(plane::Rotation::default())
        .with_child(GameCamera);

    commands.spawn(BlockBundle::new(
        &mut *meshes,
        material.0.clone(),
        TextureAtlas {
            layout: texture_atlas_layout.0.clone(),
            index: 0,
        },
        &*texture_atlas_layouts,
        Vec3::splat(0.0),
    ));

    commands.spawn(BlockBundle::new(
        &mut *meshes,
        material.0.clone(),
        TextureAtlas {
            layout: texture_atlas_layout.0.clone(),
            index: 1,
        },
        &*texture_atlas_layouts,
        Vec3::new(1.0, 0.0, 0.0),
    ));

    commands.spawn(BlockBundle::new(
        &mut *meshes,
        material.0.clone(),
        TextureAtlas {
            layout: texture_atlas_layout.0.clone(),
            index: 2,
        },
        &*texture_atlas_layouts,
        Vec3::new(2.0, 0.0, 0.0),
    ));

    commands.spawn((
        Player,
        Name::new("Player"),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));
}

fn load(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<super::State>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(super::State::Playing);
    }
}
