use super::super::game::data::{
    MyTextureAtlasLayout, TextureAtlasImage, TEXTURE_ATLAS_COLUMNS, TEXTURE_ATLAS_ROWS,
};
use super::window::DebuggerWindow;
use crate::systems::despawn_recursive;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;

#[derive(Resource, Default)]
struct SelectedTextureIndex(usize);

#[derive(Component)]
struct UiCamera;

#[derive(Component)]
struct Root;

#[derive(Component)]
struct SelectedTextureIndexText;

#[derive(Component)]
struct TextureButton;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_camera)
        .init_resource::<SelectedTextureIndex>()
        .add_observer(spawn_ui)
        .add_systems(
            OnExit(super::State::Enabled),
            despawn_recursive::<With<Root>>,
        )
        .add_systems(
            Update,
            (
                update_selected_texture_index_text,
                texture_button_interaction,
            )
                .run_if(in_state(super::State::Enabled)),
        );
}

fn spawn_camera(trigger: Trigger<OnAdd, DebuggerWindow>, mut commands: Commands) {
    commands.spawn((
        UiCamera,
        Camera2d::default(),
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(trigger.entity())),
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
    ));
}

fn spawn_ui(
    trigger: Trigger<OnAdd, UiCamera>,
    mut commands: Commands,
    texture_atlas_image: Res<TextureAtlasImage>,
    texture_atlas_layout: Res<MyTextureAtlasLayout>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn((
            Root,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                ..default()
            },
            TargetCamera(trigger.entity()),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Debugger"));
            parent
                .spawn(Text::new("Index: "))
                .with_child((SelectedTextureIndexText, TextSpan::default()));

            parent.spawn(Node::default()).with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            width: Val::Auto,
                            height: Val::Percent(100.0),
                            aspect_ratio: Some(1.0),
                            border: UiRect::all(Val::Px(4.0)),
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
                        },
                        BorderColor(Color::WHITE),
                    ))
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

                            parent.spawn((
                                TextureButton,
                                Node {
                                    position_type: PositionType::Relative,
                                    display: Display::Block,
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    aspect_ratio: Some(1.0),
                                    ..default()
                                },
                                Outline::new(Val::Px(4.0), Val::Px(0.0), Color::NONE),
                                ZIndex::default(),
                                Button,
                                ImageNode::from_atlas_image(
                                    texture_atlas_image.0.clone(),
                                    texture_atlas,
                                ),
                            ));
                        }
                    });
            });
        });
}

fn update_selected_texture_index_text(
    selected_texture_index: Res<SelectedTextureIndex>,
    mut selected_texture_index_text: Single<&mut TextSpan, With<SelectedTextureIndexText>>,
) {
    selected_texture_index_text.0 = format!("{:?}", selected_texture_index.0);
}

fn texture_button_interaction(
    mut selected_texture_index: ResMut<SelectedTextureIndex>,
    mut buttons: Query<(&Interaction, &mut Outline, &mut ZIndex, &ImageNode), With<TextureButton>>,
) {
    for (interaction, mut outline, mut z_index, image_node) in &mut buttons {
        let texture_atlas_layout_index = image_node.texture_atlas.as_ref().unwrap().index;

        match *interaction {
            Interaction::None => {
                if texture_atlas_layout_index == selected_texture_index.0 {
                    outline.color = Color::WHITE;
                    z_index.0 = 1;
                } else {
                    outline.color = Color::NONE;
                    z_index.0 = 0;
                }
            }

            Interaction::Hovered => {
                outline.color = Color::WHITE.with_alpha(0.5);
                z_index.0 = 1;
            }

            Interaction::Pressed => {
                outline.color = Color::WHITE;
                z_index.0 = 1;
                selected_texture_index.0 = texture_atlas_layout_index;
            }
        }
    }
}
