use super::editor;
use super::info;
use super::window::DebuggerWindow;
use crate::systems::despawn_recursive;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::WindowRef;

#[derive(Component)]
struct UiCamera;

#[derive(Component)]
pub struct Root;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_camera)
        .add_observer(spawn_ui)
        .add_systems(
            OnExit(super::State::Enabled),
            despawn_recursive::<Or<(With<UiCamera>, With<Root>)>>,
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

fn spawn_ui(trigger: Trigger<OnAdd, UiCamera>, mut commands: Commands) {
    commands
        .spawn((
            Root,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            TargetCamera(trigger.entity()),
        ))
        .with_children(|parent| {
            parent.spawn(info::UiRoot);
            parent.spawn(editor::UiRoot);
        });
}
