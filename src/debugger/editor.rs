use super::super::game;
use super::super::game::block::Block;
use super::super::game::camera::GameCamera;
use super::super::game::loading::game_loader::Game;
use super::super::game::loading::level_loader;
use super::super::game::loading::level_loader::Level;
use super::super::game::loading::{
    BlockMaterial, LoadingGame, MyTextureAtlasLayout, TextureAtlasImage, LEVELS_DIRECTORY,
    TEXTURE_ATLAS_COLUMNS, TEXTURE_ATLAS_ROWS,
};
use super::super::game::plane::{Rotation, Translation};
use crate::game::block::{BlockBundle, TextureAtlasIndices};
use bevy::log::tracing_subscriber::fmt::MakeWriter;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde_json::to_writer;
use std::fs::OpenOptions;

#[derive(Component)]
#[require(Node(ui_root_node))]
pub struct UiRoot;

fn ui_root_node() -> Node {
    Node {
        flex_grow: 1.0,
        ..default()
    }
}

#[derive(Resource, Default, Debug)]
pub struct TextureAtlasIndex(usize);

#[derive(Component)]
struct SelectedTextureAtlasButton;

pub fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .init_resource::<TextureAtlasIndex>()
        .add_observer(spawn)
        .add_systems(
            Update,
            (draw, save)
                .run_if(in_state(super::State::Enabled).and(in_state(game::State::Playing))),
        );
}

fn spawn(
    trigger: Trigger<OnAdd, UiRoot>,
    mut commands: Commands,
    layouts: Res<Assets<TextureAtlasLayout>>,
    layout: Res<MyTextureAtlasLayout>,
    image: Res<TextureAtlasImage>,
) {
    commands.entity(trigger.entity()).with_children(|parent| {
        parent
            .spawn(Node {
                height: Val::Percent(100.0),
                aspect_ratio: Some(1.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::fr(TEXTURE_ATLAS_COLUMNS as u16, 1.0),
                grid_template_rows: RepeatedGridTrack::fr(TEXTURE_ATLAS_ROWS as u16, 1.0),
                ..default()
            })
            .with_children(|parent| {
                for index in 0..layouts.get(&layout.0).unwrap().len() {
                    let texture_atlas = TextureAtlas {
                        layout: layout.0.clone(),
                        index,
                    };

                    parent
                        .spawn((
                            Node {
                                position_type: PositionType::Relative,
                                aspect_ratio: Some(1.0),
                                display: Display::Block,
                                ..default()
                            },
                            Outline::new(Val::Px(2.0), Val::ZERO, Color::NONE),
                            ZIndex::default(),
                            Button,
                            ImageNode::from_atlas_image(image.0.clone(), texture_atlas),
                        ))
                        .observe(button_over)
                        .observe(button_out)
                        .observe(button_click);
                }
            });
    });
}

fn button_over(trigger: Trigger<Pointer<Over>>, mut query: Query<(&mut Outline, &mut ZIndex)>) {
    let (mut outline, mut z_index) = query.get_mut(trigger.entity()).unwrap();

    outline.color = Color::WHITE.with_alpha(0.5);
    z_index.0 = 1;
}

fn button_out(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(
        &mut Outline,
        &mut ZIndex,
        Option<&SelectedTextureAtlasButton>,
    )>,
) {
    let (mut outline, mut z_index, selected) = query.get_mut(trigger.entity()).unwrap();

    if let Some(_) = selected {
        outline.color = Color::WHITE;
        return;
    }

    outline.color = Color::NONE;
    z_index.0 = 0;
}

fn button_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut index: ResMut<TextureAtlasIndex>,
    image_nodes: Query<&ImageNode>,
    previous_entity: Option<Single<Entity, With<SelectedTextureAtlasButton>>>,
    mut query: Query<(&mut Outline, &mut ZIndex)>,
) {
    let entity = trigger.entity();

    // Update index
    let image_node = image_nodes.get(entity).unwrap();
    index.0 = image_node.texture_atlas.as_ref().unwrap().index;

    // Update previous button
    if let Some(entity) = previous_entity {
        let (mut outline, mut z_index) = query.get_mut(*entity).unwrap();

        outline.color = Color::NONE;
        z_index.0 = 0;

        commands
            .entity(*entity)
            .remove::<SelectedTextureAtlasButton>();
    }

    // Update next button
    let (mut outline, mut z_index) = query.get_mut(entity).unwrap();

    outline.color = Color::WHITE;
    z_index.0 = 1;

    commands.entity(entity).insert(SelectedTextureAtlasButton);
}

fn draw(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        MeshRayCast,
        (
            ResMut<Assets<Mesh>>,
            Query<(&mut Mesh3d, &mut TextureAtlasIndices), With<Block>>,
        ),
    )>,
    index: Res<TextureAtlasIndex>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    layout: Res<MyTextureAtlasLayout>,
    plane_rotation: ResMut<Rotation>,
    material: Res<BlockMaterial>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
    plane_translation: Single<&mut Transform, With<Translation>>,
) {
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let mut mesh_ray_cast = param_set.p0();
    let ray = camera
        .0
        .viewport_to_world(camera.1, cursor_position)
        .unwrap();

    let hit = mesh_ray_cast
        .cast_ray(ray, &RayCastSettings::default())
        .first();

    if let Some(hit) = hit {
        let (entity, hit) = hit.to_owned();

        if mouse.pressed(MouseButton::Left) {
            let (mut meshes, mut blocks) = param_set.p1();

            if let Ok((mesh_handle, mut indices)) = blocks.get_mut(entity) {
                *indices = match hit.normal.abs() {
                    Vec3::X => TextureAtlasIndices {
                        x: index.0,
                        ..*indices
                    },
                    Vec3::Y => TextureAtlasIndices {
                        y: index.0,
                        ..*indices
                    },
                    Vec3::Z => TextureAtlasIndices {
                        z: index.0,
                        ..*indices
                    },
                    _ => panic!(),
                };

                let mesh = meshes.get_mut(mesh_handle.0.id()).unwrap();
                *mesh = BlockBundle::mesh(&layouts, layout.0.clone(), &*indices);
            };
        } else if mouse.pressed(MouseButton::Right) {
            commands.entity(entity).despawn_recursive();
        }
    } else if mouse.pressed(MouseButton::Left) {
        let Some(distance) = ray.intersect_plane(
            plane_translation.translation,
            InfinitePlane3d::new(plane_rotation.get() * Vec3::Z),
        ) else {
            return;
        };

        let point = ray.get_point(distance).floor();
        let (mut meshes, ..) = param_set.p1();

        commands.spawn(BlockBundle::new(
            &point,
            &mut meshes,
            &layouts,
            layout.0.clone(),
            TextureAtlasIndices {
                x: index.0,
                y: index.0,
                z: index.0,
            },
            material.0.clone(),
        ));
    }
}

fn save(
    keyboard: Res<ButtonInput<KeyCode>>,
    games: Res<Assets<Game>>,
    game: Res<LoadingGame>,
    blocks: Query<(&GlobalTransform, &TextureAtlasIndices), With<Block>>,
) {
    // Todo: Hardcoded for Dvorak
    if !(keyboard.pressed(KeyCode::ControlLeft) && keyboard.pressed(KeyCode::KeyM)) {
        return;
    }

    let level = Level {
        blocks: blocks
            .iter()
            .map(|(transform, indices)| level_loader::Block {
                translation: transform.translation() - 0.5,
                texture_atlas_indices: indices.clone(),
            })
            .collect(),
    };

    let level_file = &games.get(game.0.id()).unwrap().level;
    let level_path = format!("assets/{LEVELS_DIRECTORY}/{level_file}.json");

    let file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(level_path)
        .unwrap();

    to_writer::<_, Level>(file.make_writer(), &level).unwrap();
}
