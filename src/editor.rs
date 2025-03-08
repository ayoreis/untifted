mod orbit_zoom_pan_camera;

use super::AppState;
use crate::MainCamera;
use bevy::prelude::*;
use orbit_zoom_pan_camera::*;
use std::time::Instant;

#[derive(Component)]
struct EditorCamera {
    origin: Vec3,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (orbit_zoom_pan, gizmos).run_if(in_state(AppState::Editor)),
        )
        .add_systems(Update, toggle_editor);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        EditorCamera { origin: Vec3::ZERO },
        Camera3d::default(),
        Camera {
            is_active: false,
            ..default()
        },
        Transform::from_translation(Vec3::splat(5.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn toggle_editor(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut last_escape: Local<Option<Instant>>,
    mut main_camera: Single<&mut Camera, With<MainCamera>>,
    mut editor_camera: Single<&mut Camera, (With<EditorCamera>, Without<MainCamera>)>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some(instant) = *last_escape
            && instant.elapsed().as_secs_f32() <= 0.5
        {
            next_state.set(match **state {
                AppState::Editor => AppState::Game,
                _ => AppState::Editor,
            });

            main_camera.is_active = !main_camera.is_active;
            editor_camera.is_active = !editor_camera.is_active;
        }

        *last_escape = Some(Instant::now());
    }
}

fn gizmos(mut gizmos: Gizmos, camera: Single<(&EditorCamera, &Transform)>) {
    let (state, transform) = camera.into_inner();

    gizmos.circle(
        Isometry3d::new(state.origin, transform.rotation),
        0.1,
        Color::WHITE,
    );
}
