use super::super::AppState;
use super::GameState;
use crate::systems::despawn_recursive;
use crate::ui::{button, BLUE, LIGHT_BLUE, LIGHT_RED, RED};
use bevy::prelude::*;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct ExitButton;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Paused), spawn_ui)
        .add_systems(Update, pause.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            (resume, resume_button, exit_button).run_if(in_state(GameState::Paused)),
        )
        .add_systems(OnExit(GameState::Paused), despawn_recursive::<With<UiRoot>>);
}

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(button(ResumeButton, BLUE))
                .with_child(Text::new("Resume"));

            parent
                .spawn(button(ExitButton, RED))
                .with_child(Text::new("Exit"));
        });
}

fn pause(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn resume(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing)
    }
}

fn resume_button(
    button: Single<(&Interaction, &mut BackgroundColor), With<ResumeButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (interaction, mut background_color) = button.into_inner();

    match interaction {
        Interaction::None => {
            background_color.0 = BLUE;
        }
        Interaction::Hovered => {
            background_color.0 = LIGHT_BLUE;
        }
        Interaction::Pressed => {
            next_state.set(GameState::Playing);
        }
    }
}

fn exit_button(
    button: Single<(&Interaction, &mut BackgroundColor), With<ExitButton>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (interaction, mut background_color) = button.into_inner();

    match interaction {
        Interaction::None => {
            background_color.0 = RED;
        }
        Interaction::Hovered => {
            background_color.0 = LIGHT_RED;
        }
        Interaction::Pressed => {
            next_state.set(AppState::Splash);
        }
    }
}
