use super::super::game;
use super::super::game::block::Block;
use super::super::game::camera::GameCamera;
use super::super::game::loading::{
    MyTextureAtlasLayout, TextureAtlasImage, TEXTURE_ATLAS_COLUMNS, TEXTURE_ATLAS_ROWS,
};
use super::ui;
use crate::game::block::{BlockBundle, TextureAtlasIndices};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
struct SelectedTextureAtlasIndex(usize);

#[derive(Component)]
struct SelectedTextureAtlasIndexText;

#[derive(Component)]
struct SelectedTextureAtlasIndexButton;

pub fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .init_resource::<SelectedTextureAtlasIndex>()
        .add_observer(spawn_ui)
        .add_systems(
            Update,
            draw.run_if(in_state(super::State::Enabled))
                .run_if(in_state(game::State::Playing)),
        );
}

fn texture_atlas_button_over(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<(&mut Outline, &mut ZIndex)>,
) {
    let (mut outline, mut z_index) = query.get_mut(trigger.entity()).unwrap();

    outline.color = Color::WHITE.with_alpha(0.5);
    z_index.0 = 1;
}

fn texture_atlas_button_out(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(
        &mut Outline,
        &mut ZIndex,
        Option<&SelectedTextureAtlasIndexButton>,
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

fn texture_atlas_button_click(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut index: ResMut<SelectedTextureAtlasIndex>,
    image_nodes: Query<&ImageNode>,
    mut text: Single<&mut TextSpan, With<SelectedTextureAtlasIndexText>>,
    previous_entity: Option<Single<Entity, With<SelectedTextureAtlasIndexButton>>>,
    mut query: Query<(&mut Outline, &mut ZIndex)>,
) {
    let entity = trigger.entity();

    // Update index
    let image_node = image_nodes.get(entity).unwrap();
    index.0 = image_node.texture_atlas.as_ref().unwrap().index;

    // Update index text
    text.0 = format!("{index}", index = index.0);

    // Update previous
    if let Some(entity) = previous_entity {
        let (mut outline, mut z_index) = query.get_mut(*entity).unwrap();
        outline.color = Color::NONE;
        z_index.0 = 0;

        commands
            .entity(*entity)
            .remove::<SelectedTextureAtlasIndexButton>();
    }

    // Update next
    let (mut outline, mut z_index) = query.get_mut(entity).unwrap();
    outline.color = Color::WHITE;
    z_index.0 = 1;

    commands
        .entity(entity)
        .insert(SelectedTextureAtlasIndexButton);
}

fn spawn_ui(
    trigger: Trigger<OnAdd, ui::Root>,
    mut commands: Commands,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    texture_atlas_layout: Res<MyTextureAtlasLayout>,
    texture_atlas_image: Res<TextureAtlasImage>,
) {
    commands.entity(trigger.entity()).with_children(|parent| {
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Grid,
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::fr(1.0)],
                    ..default()
                },
                TargetCamera(trigger.entity()),
            ))
            .with_children(|parent| {
                parent
                    .spawn(Text::new("Index: "))
                    .with_child((SelectedTextureAtlasIndexText, TextSpan::default()));

                parent.spawn(Node::default()).with_children(|parent| {
                    parent
                        .spawn(Node {
                            width: Val::Auto,
                            height: Val::Percent(100.0),
                            aspect_ratio: Some(1.0),
                            display: Display::Grid,
                            grid_template_columns: RepeatedGridTrack::fr(
                                TEXTURE_ATLAS_COLUMNS as u16,
                                1.0,
                            ),
                            grid_template_rows: RepeatedGridTrack::fr(
                                TEXTURE_ATLAS_ROWS as u16,
                                1.0,
                            ),
                            ..default()
                        })
                        .with_children(|parent| {
                            for index in 0..texture_atlas_layouts
                                .get(&texture_atlas_layout.0)
                                .unwrap()
                                .len()
                            {
                                let texture_atlas = TextureAtlas {
                                    layout: texture_atlas_layout.0.clone(),
                                    index,
                                };

                                parent
                                    .spawn((
                                        Node {
                                            position_type: PositionType::Relative,
                                            width: Val::Auto,
                                            height: Val::Auto,
                                            aspect_ratio: Some(1.0),
                                            display: Display::Block,
                                            ..default()
                                        },
                                        Outline::new(Val::Px(2.0), Val::ZERO, Color::NONE),
                                        ZIndex::default(),
                                        Button,
                                        ImageNode::from_atlas_image(
                                            texture_atlas_image.0.clone(),
                                            texture_atlas,
                                        ),
                                    ))
                                    .observe(texture_atlas_button_over)
                                    .observe(texture_atlas_button_out)
                                    .observe(texture_atlas_button_click);
                            }
                        });
                });
            });
    });
}

fn draw(
    mouse: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        MeshRayCast,
        (
            Query<(&mut Mesh3d, &TextureAtlasIndices), With<Block>>,
            ResMut<Assets<Mesh>>,
        ),
    )>,
    selected_texture_atlas_index: Res<SelectedTextureAtlasIndex>,
    layouts: Res<Assets<TextureAtlasLayout>>,
    layout: Res<MyTextureAtlasLayout>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, transform) = *camera;
    let ray = camera
        .viewport_to_world(transform, cursor_position)
        .unwrap();
    let mut mesh_ray_cast = param_set.p0();
    let hits = mesh_ray_cast.cast_ray(ray, &RayCastSettings::default());

    if let Some(hit) = hits.first() {
        let (entity, hit) = hit.to_owned();
        let (blocks, mut meshes) = param_set.p1();

        if let Ok((mesh_handle, indices)) = blocks.get(entity) {
            let mesh = meshes.get_mut(mesh_handle.0.id()).unwrap();

            let indices = match hit.normal.abs() {
                Vec3::X => TextureAtlasIndices {
                    x: selected_texture_atlas_index.0,
                    ..*indices
                },
                Vec3::Y => TextureAtlasIndices {
                    y: selected_texture_atlas_index.0,
                    ..*indices
                },
                Vec3::Z => TextureAtlasIndices {
                    z: selected_texture_atlas_index.0,
                    ..*indices
                },
                _ => panic!(),
            };

            *mesh = BlockBundle::mesh(&layouts, layout.0.clone(), &indices);
        };
    }
}
